use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;

pub struct RequestLoggingMiddleware {
    pub next: Option<Box<dyn Middleware + Send + Sync>>,
}

impl Middleware for RequestLoggingMiddleware {
    fn run(&self, context: &mut HttpContext) -> Result<()> {
        if let Some(next) = &self.next && context.status == 0
        {
            next.run(context)?;
        }
        if context.status == 0 {
            crate::http::send_404(context)?;
        }
        crate::log::log_request(context);
        Ok(())
    }
}
