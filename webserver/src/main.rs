
use webserver::{
    middleware::{
        logging::RequestLoggingMiddleware,
        static_file::StaticFileMiddlware,},
    server::{Server}
};

const WWWROOT: &str = "../";
const WWWHOST: &str = "0.0.0.0:8080";

fn main() -> std::io::Result<()> {
    let static_file_middleware = StaticFileMiddlware { wwwroot: WWWROOT, next: None };
    let logging_middleware = RequestLoggingMiddleware { next: Some(Box::new(static_file_middleware)) };
    let server = Server{ middleware: Some(std::sync::Arc::new(logging_middleware)) };
    server.start(WWWHOST)?;
    Ok(())
}

