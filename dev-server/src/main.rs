use std::{
    io::{Read, Write}, 
    net::{TcpListener, TcpStream},
    fs::{File},
    path::PathBuf,
};

const WWWROOT: &str = "../";

fn main() -> std::io::Result<()> {
    //println!("{:?}", std::env::current_dir());
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut readbuf: [u8; 4096] = [0; 4096];
    let _ = stream.read(&mut readbuf)?;
    let is_get = readbuf.starts_with(b"GET ");
    if !is_get { send_404(stream, "invalid verb!")?; return Ok(()); }
    let end_of_file_path = readbuf.iter().skip(4).position(|&b| b == b' ').unwrap();
    let uri = std::str::from_utf8(&readbuf[4..(end_of_file_path+4)]).unwrap();
    let mut path_buf = PathBuf::from(WWWROOT);
    path_buf.push(uri.trim_start_matches('/'));
    //let file_path = format!("{}/{}{}", std::env::current_dir().unwrap().display(), WWWROOT, uri);
    println!("{:?}", path_buf);
    match File::open(path_buf) {
        Ok(file) => { send_file(stream, file, uri)?; },
        Err(e) => { send_404(stream, &format!("error: {:?}", e))?; }
    };
    Ok(())
}

fn send_file(mut stream: TcpStream, mut file: File, uri: &str) -> std::io::Result<()> {
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

fn send_404(mut stream: TcpStream, msg: &str) -> std::io::Result<()> {
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