use std::{
    fs::File, 
    io::{Read, Write}, 
    net::{TcpListener, TcpStream}, 
    path::PathBuf, 
    time::Duration,
    thread::spawn,
};

const WWWROOT: &str = "../";

struct ClientConnection {
    stream: TcpStream,
    keep_alive: bool,
    id: u32,
    request_count: u32,
}

/*struct HttpContext<'a> {
    client: &'a ClientConnection,
    path_and_query: &'a str,
    path: &'a str,
    query: &'a str,
    status: u16,
}*/

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
    //client.stream.set_nonblocking(true);
    while client.keep_alive {
        client.request_count += 1;
        match read_request(&mut client) {
            Err(e) => { log_error(format!("conn id: {}, read_request, {:?}", client.id, e).as_str()); }
            _ => {}
        }
    }
    match client.stream.shutdown(std::net::Shutdown::Both) {
        Err(e) => { log_error(format!("conn id: {}, stream.shutdown, {:?}", client.id, e).as_str()); }
        _ => { }
    }
}

fn read_request(client: &mut ClientConnection) -> std::io::Result<()> {
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
    let is_get = readbuf.starts_with(b"GET ");
    if !is_get { send_404(&client.stream, "invalid verb!")?; return Ok(()); }
    let end_of_file_path = readbuf.iter().skip(4).position(|&b| b == b' ').unwrap();
    let uri = std::str::from_utf8(&readbuf[4..(end_of_file_path+4)]).unwrap();
    let mut path_buf = PathBuf::from(WWWROOT);
    path_buf.push(uri.trim_start_matches('/'));
    log_request(format!("{} {} {:?}", client.id, client.request_count, path_buf).as_str());
    match File::open(path_buf) {
        Ok(file) => { send_file(&client.stream, file, uri)?; },
        Err(e) => { send_404(&client.stream, &format!("error: {:?}", e))?; }
    };
    Ok(())
}

fn send_file(mut stream: &TcpStream, mut file: File, uri: &str) -> std::io::Result<()> {
    let content_type = match uri.split(".").last().unwrap() {
        "js" => "text/javascript",
        "css" => "text/css",
        "html" => "text/html",
        _ => "",
    };
    if content_type.len() == 0 { send_404(stream, "file extension not supported!")?; return Ok(()); }
    let mut filebuf: Vec<u8> = Vec::new();
    file.read_to_end(&mut filebuf)?;
    stream.write(b"HTTP/1.1 200 OK\r\n")?;
    stream.write(format!("Content-Type: {}\r\n", content_type).as_bytes())?;
    stream.write(format!("Content-Length: {}\r\n", filebuf.len()).as_bytes())?;
    stream.write(b"\r\n")?;
    stream.write_all(&filebuf)?;
    Ok(())
}

fn send_404(mut stream: &TcpStream, msg: &str) -> std::io::Result<()> {
    stream.write(b"HTTP/1.1 404 NotFound\r\nContent-Type: text/html\r\n")?;
    stream.write(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())?;
    Ok(())
}

fn _echo_client(mut stream: TcpStream ) -> std::io::Result<()> {
    let mut buf: [u8; 4096] = [0; 4096];
    let bytes_read = stream.read(&mut buf)?;
    println!("{}", bytes_read);
    //stream.write_all(&buf)?;
    stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n")?;
    stream.write(format!("Content-Length: {}\r\n\r\n", bytes_read + 11).as_bytes())?;
    stream.write(b"<pre>")?;
    stream.write(&buf[..bytes_read])?;
    stream.write(b"</pre>")?;
    
    Ok(())
}

fn log_error(msg: &str) {
    println!("ERROR: {}", msg);
}
fn log_request(msg: &str) {
    println!("{}", msg);
}