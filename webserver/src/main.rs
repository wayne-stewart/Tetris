use std::{
    fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, path::PathBuf, thread::spawn, time::Duration
};

const WWWROOT: &str = "../";

struct ClientConnection {
    stream: TcpStream,
    keep_alive: bool,
    id: u32,
    request_count: u32,
}

struct HttpContext<'a> {
    client: &'a ClientConnection,
    verb: &'a str,
    path_and_query: &'a str,
    path: &'a str,
    _query: &'a str,
    status: u16,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut client_id_source = 0;
    for stream in listener.incoming() {
        client_id_source += 1;
        let client_id = client_id_source;
        spawn(move || {
            match stream {
                Err(e) => { log_error(format!("conn id: {}, new stream failed, {:?}", client_id, e).as_str()); },
                Ok(stream) => {
                    let client = ClientConnection { 
                        stream: stream, 
                        keep_alive: true,
                        id: client_id,
                        request_count: 0,
                    };
                    handle_client(client);
                },
            };
        });
    }
    Ok(())
}

fn handle_client(mut client: ClientConnection) {
    client.stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    while client.keep_alive {
        client.request_count += 1;
        match handle_request(&mut client) {
            Err(e) => { log_error(format!("conn id: {}, handle_request, {:?}", client.id, e).as_str()); }
            _ => {}
        }
    }
    match client.stream.shutdown(std::net::Shutdown::Both) {
        Err(e) => { log_error(format!("conn id: {}, stream.shutdown, {:?}", client.id, e).as_str()); }
        _ => { }
    }
}

fn handle_request(client: &mut ClientConnection) -> std::io::Result<()> {
    let mut readbuf: [u8; 4096] = [0; 4096];
    let bytes_read = match client.stream.read(&mut readbuf) {
        Ok(s) => s,
        Err(_e) => 0
    };
    if bytes_read == 0 {
        client.keep_alive = false;
        return Ok(());
    }
    //println!("{}", String::from_utf8(readbuf[0..bytes_read].to_vec()).unwrap());
    let mut iter = readbuf.split(|&c| c == b' ' || c == b'\r');
    let verb = std::str::from_utf8(iter.next().unwrap()).unwrap();
    let path_and_query = std::str::from_utf8(iter.next().unwrap()).unwrap();
    let _http_version = std::str::from_utf8(iter.next().unwrap()).unwrap();
    let mut path: &str = path_and_query;
    let mut query: &str = "";
    if let Some(query_index) = path_and_query.find('?') {
        (path, query) = path_and_query.split_at(query_index);
    }
    let mut http_context = HttpContext {
        client: client,
        verb: verb,
        path_and_query: path_and_query,
        path: path,
        _query: query,
        status: 0,
    };
    if http_context.verb.eq("GET") {
        handle_static_file(&mut http_context)?;
    }
    else {
        send_405(&mut http_context)?
    }
    log_request(&http_context);
    Ok(())
}

fn handle_static_file(mut http_context: &mut HttpContext) -> std::io::Result<()> {
    let mut path_buf = PathBuf::from(WWWROOT);
    path_buf.push(http_context.path.trim_start_matches('/'));
    match File::open(path_buf) {
        Ok(file) => { send_file(&mut http_context, file)?; },
        Err(_) => { send_404(&mut http_context)?; }
    };
    Ok(())
}

fn send_file(http_context: &mut HttpContext, mut file: File) -> std::io::Result<()> {
    let mut stream = &http_context.client.stream;
    if let Some(content_type) = content_type_from_path_extension(http_context.path) {
        http_context.status = 200;
        let mut filebuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut filebuf)?;
        stream.write(b"HTTP/1.1 200 OK\r\n")?;
        stream.write(format!("Content-Type: {}\r\n", content_type).as_bytes())?;
        stream.write(format!("Content-Length: {}\r\n", filebuf.len()).as_bytes())?;
        stream.write(b"\r\n")?;
        stream.write_all(&filebuf)?;
        Ok(())
    } else {
        send_400(http_context, "file extension not supported!")
    }
}

fn send_400(http_context: &mut HttpContext, msg: &str) -> std::io::Result<()> {
    http_context.status = 400;
    send_message(&http_context.client.stream, 400, "Bad Request", msg)
}

fn send_404(http_context: &mut HttpContext) -> std::io::Result<()> {
    http_context.status = 404;
    send_message(&http_context.client.stream, 404, "Not Found", "Resource Not Found")
}

fn send_405(http_context: &mut HttpContext) -> std::io::Result<()> {
    http_context.status = 405;
    send_message(&http_context.client.stream, 405, "Method Not Allowed", "Method Not Allowed")
}

fn send_message(mut stream: &TcpStream, status_code: u16, status_msg: &str, msg: &str) -> std::io::Result<()> {
    stream.write(format!("HTTP/1.1 {} {}\r\n", status_code, status_msg).as_bytes())?;
    stream.write(format!("Content-Type: text/html\r\n").as_bytes())?;
    stream.write(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())?;
    Ok(())
}

fn _echo_client(mut stream: TcpStream ) -> std::io::Result<()> {
    let mut buf: [u8; 4096] = [0; 4096];
    let bytes_read = stream.read(&mut buf)?;
    println!("{}", bytes_read);
    stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n")?;
    stream.write(format!("Content-Length: {}\r\n\r\n", bytes_read + 11).as_bytes())?;
    stream.write(b"<pre>")?;
    stream.write(&buf[..bytes_read])?;
    stream.write(b"</pre>")?;
    Ok(())
}

fn content_type_from_path_extension(path: &str) -> Option<&str> {
    if let Some(ext_index) = path.rfind('.') {
        let (_, ext) = path.split_at(ext_index);
        return match ext {
            ".js" => Some("text/javascript"),
            ".css" => Some("text/css"),
            ".html" => Some("text/html"),
            _ => None,
        }
    } else {
        return None;
    }
}

fn log_error(msg: &str) {
    println!("ERROR: {}", msg);
}
fn _log_debug(msg: &str) {
    println!("DEBUG: {}", msg);
}
fn log_request(ctx: &HttpContext) {
    println!("{} {} {} {}", 
        ctx.client.id, 
        ctx.client.request_count, 
        ctx.status, 
        ctx.path_and_query);
}