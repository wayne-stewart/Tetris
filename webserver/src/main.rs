use webserver::{
    middleware::{
        logging::RequestLoggingMiddleware, 
        static_file::StaticFileMiddlware,
        router::{RouterMiddleware, RouteHandler},
    },
    server::{Server, HttpContext},
    Result,
};
use std::sync::Arc;
use std::collections::HashMap;

const WWWROOT: &str = "../";
const WWWHOST: &str = "0.0.0.0:8080";

fn route_hello(context: &mut HttpContext) -> Result<()>{
    webserver::http::send(&context.client.stream, 200, "Ok", "Hello, World!")?;
    context.status = 200;
    Ok(())
}

fn route_echo(context: &mut HttpContext) -> Result<()> {
    webserver::http::send_echo(context)?;
    context.status = 200;
    Ok(())
}

fn main() -> webserver::Result<()> {

    let static_file_middleware = StaticFileMiddlware {
        wwwroot: WWWROOT,
        next: None,
    };

    let mut routes = HashMap::new();
    routes.insert("/hello", route_hello as RouteHandler);
    routes.insert("/echo", route_echo as RouteHandler);
    
    let router_middleware = RouterMiddleware {
        routes: Arc::new(routes),
        next: Some(Box::new(static_file_middleware)),
    };

    let logging_middleware = RequestLoggingMiddleware {
        next: Some(Box::new(router_middleware)),
    };

    let server = Server {
        middleware: Some(std::sync::Arc::new(logging_middleware)),
    };

    server.start(WWWHOST)?;

    Ok(())
}
