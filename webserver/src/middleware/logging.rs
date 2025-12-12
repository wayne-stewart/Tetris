use crate::server::HttpContext;
use crate::server::Middleware;

pub struct RequestLoggingMiddleware {
    pub next: Option<Box<dyn Middleware + Send + Sync>>,
}

impl Middleware for RequestLoggingMiddleware {
    fn run(&self, context: &mut HttpContext) -> std::io::Result<()> {
        if let Some(next) = &self.next
            && context.status == 0
        {
            next.run(context)?;
        }
        crate::log::log_request(context);
        Ok(())
    }
}
