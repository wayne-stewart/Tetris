use crate::server::HttpContext;
use crate::server::Middleware;

pub struct RouterMiddleware {
    pub next: Option<Box<dyn Middleware>>,
}

impl Middleware for RouterMiddleware {
    fn run(&self, context: &mut HttpContext) -> std::io::Result<()> {
        if let Some(next) = &self.next
            && context.status == 0
        {
            next.run(context)?;
        }
        Ok(())
    }
}
