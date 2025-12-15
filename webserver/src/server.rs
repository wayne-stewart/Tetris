use crate::log::*;
use crate::Result;
use std::io::Read;
use std::net::TcpListener;
use std::str::Utf8Error;
use std::thread::spawn;
use std::time::Duration;
use std::sync::Arc;

pub struct ClientConnection {
    pub stream: std::net::TcpStream,
    pub keep_alive: bool,
    pub id: usize,
    pub request_count: usize,
}

pub struct HttpContext<'a> {
    pub client: &'a ClientConnection,
    pub verb: &'a str,
    pub path_and_query: &'a str,
    pub path: &'a str,
    pub _query: &'a str,
    pub status: u16,
    pub readbuf: [u8; 4096],
    pub readbuf_len: usize,
    pub writebuf: [u8; 4096],
}

pub struct Server {
    pub middleware: Option<Arc<dyn Middleware + Sync + Send>>,
}

pub trait Middleware {
    fn run(&self, context: &mut HttpContext) -> Result<()>;
}

impl Server {
    pub fn start(&self, http_addr: &str) -> Result<()> {
        if let Some(m) = &self.middleware {
            let listener = TcpListener::bind(http_addr)?;
            for (client_id, stream) in listener.incoming().enumerate() {
                let middleware = m.clone();
                spawn(move || {
                    match stream {
                        Err(e) => {
                            log_error(
                                format!("conn id: {}, new stream failed, {:?}", client_id, e)
                                    .as_str(),
                            );
                        }
                        Ok(stream) => {
                            let client = ClientConnection {
                                stream,
                                keep_alive: true,
                                id: client_id,
                                request_count: 0,
                            };
                            handle_client(middleware.as_ref(), client);
                        }
                    };
                });
            }
        } else {
            log_error("Server cannot start! No middleware defined.");
        }
        Ok(())
    }
}

fn handle_client(middleware: &dyn Middleware, mut client: ClientConnection) {
    client
        .stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    while client.keep_alive {
        client.request_count += 1;
        if let Err(e) = handle_request(middleware, &mut client) {
            log_error(format!("conn id: {}, handle_request, {:?}", client.id, e).as_str());
        }
    }
    if let Err(e) = client.stream.shutdown(std::net::Shutdown::Both) {
        log_error(format!("conn id: {}, stream.shutdown, {:?}", client.id, e).as_str());
    }
}

fn handle_request(middleware: &dyn Middleware, client: &mut ClientConnection) -> Result<()> {
    let mut readbuf: [u8; 4096] = [0; 4096];
    
    let bytes_read = client.stream.read(&mut readbuf).unwrap_or_default();
    if bytes_read == 0 {
        client.keep_alive = false;
        return Ok(());
    }
    
    let mut iter = readbuf.split(|&c| c == b' ' || c == b'\r');
    let verb = utf8(iter.next())?;
    let path_and_query = utf8(iter.next())?;
    let _http_version = utf8(iter.next())?;
    let mut path: &str = path_and_query;
    let mut query: &str = "";

    if let Some(query_index) = path_and_query.find('?') {
        (path, query) = path_and_query.split_at(query_index);
    }

    let mut http_context = HttpContext {
        client,
        verb,
        path_and_query,
        path,
        _query: query,
        status: 0,
        readbuf,
        readbuf_len: bytes_read,
        writebuf: [0; 4096],
    };

    middleware.run(&mut http_context)?;
    
    Ok(())
}

fn utf8(x: Option<&[u8]>) -> std::result::Result<&str, Utf8Error> {
    std::str::from_utf8(x.or(None).unwrap_or(b""))
}
