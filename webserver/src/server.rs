use crate::log::*;
use crate::Result;
use std::io::BufRead;
use std::io::Read;
use std::io::{BufReader};
use std::net::TcpListener;
use std::str::Utf8Error;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

pub struct ClientConnection {
    pub stream: std::net::TcpStream,
    pub keep_alive: bool,
    pub id: usize,
    pub request_count: usize,
}

pub enum ConnectionKeepAlive { KeepAlive, Close}
pub enum ContentType { TextHtml, ApplicationJson, ImagePng, ImageJpeg, Unknown }

pub struct Headers {
    pub accept: String,
    pub content_type: ContentType,
    pub content_length: usize,
    pub connection: ConnectionKeepAlive,
    pub user_agent: String,
}

pub struct HttpContext<'a> {
    pub client: &'a ClientConnection,
    pub verb: &'a str,
    pub path_and_query: &'a str,
    pub path: &'a str,
    pub query: &'a str,
    pub status: u16,
}

pub struct Server<'a> {
    middleware: Option<Arc<dyn Middleware + Sync + Send + 'a>>,
    middleware_last: Option<Arc<dyn Middleware + Sync + Send + 'a>>,
}

pub trait Middleware {
    fn run(&self, context: &mut HttpContext) -> Result<()>;
    fn set_next(&self, next: Arc<dyn Middleware + Send + Sync>);
}

impl Server<'static> {
    pub fn new () -> Self {
        Server {
            middleware: None,
            middleware_last: None,
        }
    }

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

    pub fn add_middleware<T>(&mut self, middleware: T) where T: Middleware + Send + Sync + 'static {
        let middleware = Arc::new(middleware);
        if let Some(last) = &self.middleware_last {
            last.set_next(middleware.clone());
            self.middleware_last = Some(middleware);
        } else {
            self.middleware = Some(middleware.clone());
            self.middleware_last = Some(middleware);
        }
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
    
    let reader = BufReader::new(&client.stream);
    let mut line = String::with_capacity(4096);
    if let Err(e) = reader.take(4096).read_line(&mut line) {
        client.keep_alive = false;
        if  e.kind() == std::io::ErrorKind::WouldBlock ||
            e.kind() == std::io::ErrorKind::NotConnected {
                return Ok(());
        } else {
            return Err(e.into());
        }
    }

    let mut request_line_parts = line.split(' ');
    let verb = request_line_parts.next().unwrap_or("").trim();
    let path_and_query = request_line_parts.next().unwrap_or("").trim();
    let _http_version = request_line_parts.next().unwrap_or("").trim();
    
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
        query,
        status: 0,
    };

    middleware.run(&mut http_context)?;
    
    Ok(())
}

fn utf8(x: Option<&[u8]>) -> std::result::Result<&str, Utf8Error> {
    std::str::from_utf8(x.or(None).unwrap_or(b""))
}
