use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

use crate::server::HttpContext;

pub fn send_file(http_context: &mut HttpContext, mut file: File) -> std::io::Result<()> {
    let mut stream = &http_context.client.stream;
    if let Some(content_type) = content_type_from_path_extension(http_context.path) {
        http_context.status = 200;
        let mut filebuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut filebuf)?;
        stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
        stream.write_all(format!("Content-Type: {}\r\n", content_type).as_bytes())?;
        stream.write_all(format!("Content-Length: {}\r\n", filebuf.len()).as_bytes())?;
        stream.write_all(b"\r\n")?;
        stream.write_all(&filebuf)?;
        Ok(())
    } else {
        send_400(http_context, "file extension not supported!")
    }
}

pub fn send_400(http_context: &mut HttpContext, msg: &str) -> std::io::Result<()> {
    http_context.status = 400;
    send(&http_context.client.stream, 400, "Bad Request", msg)
}

pub fn send_404(http_context: &mut HttpContext) -> std::io::Result<()> {
    http_context.status = 404;
    send(
        &http_context.client.stream,
        404,
        "Not Found",
        "Resource Not Found",
    )
}

pub fn send_405(http_context: &mut HttpContext) -> std::io::Result<()> {
    http_context.status = 405;
    send(
        &http_context.client.stream,
        405,
        "Method Not Allowed",
        "Method Not Allowed",
    )
}

pub fn send(
    mut stream: &TcpStream,
    status_code: u16,
    status_msg: &str,
    msg: &str,
) -> std::io::Result<()> {
    stream.write_all(format!("HTTP/1.1 {} {}\r\n", status_code, status_msg).as_bytes())?;
    stream.write_all(b"Content-Type: text/html\r\n")?;
    stream.write_all(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())?;
    Ok(())
}

/*pub fn send_echo(context: &mut HttpContext) -> std::io::Result<()> {
    let mut stream = &context.client.stream;
    let bytes_read = context.readbuf_len;
    let buf = &context.readbuf;
    stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n")?;
    stream.write_all(format!("Content-Length: {}\r\n\r\n", bytes_read + 11).as_bytes())?;
    stream.write_all(b"<pre>")?;
    stream.write_all(&buf[..bytes_read])?;
    stream.write_all(b"</pre>")?;
    Ok(())
}*/

pub fn content_type_from_path_extension(path: &str) -> Option<&str> {
    if let Some(ext_index) = path.rfind('.') {
        let (_, ext) = path.split_at(ext_index);
        match ext {
            ".js" => Some("text/javascript"),
            ".css" => Some("text/css"),
            ".html" => Some("text/html"),
            _ => None,
        }
    } else {
        None
    }
}
