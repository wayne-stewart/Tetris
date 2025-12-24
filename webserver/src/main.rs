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

/*fn route_echo(context: &mut HttpContext) -> Result<()> {
    webserver::http::send_echo(context)?;
    context.status = 200;
    Ok(())
}*/

fn main() -> webserver::Result<()> {

    let static_file_middleware = StaticFileMiddlware::new(WWWROOT);

    let mut routes = HashMap::new();
    routes.insert("/hello", route_hello as RouteHandler);
    //routes.insert("/echo", route_echo as RouteHandler);

    let router_middleware = RouterMiddleware::new(Arc::new(routes));

    let logging_middleware = RequestLoggingMiddleware::new();

    let mut server = Server::new();

    server.add_middleware(logging_middleware);
    server.add_middleware(router_middleware);
    server.add_middleware(static_file_middleware);


    server.start(WWWHOST)?;

    Ok(())
}
