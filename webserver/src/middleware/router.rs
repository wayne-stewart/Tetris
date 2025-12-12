
use crate::server::Middleware;
use crate::server::HttpContext;

pub struct RouterMiddleware {
    pub next: Option<Box<dyn Middleware>>,
}

impl Middleware for RouterMiddleware {
    fn run(&self, context: &mut HttpContext) -> std::io::Result<()>{
        println!("router middleware PRE");
        if let Some(next) = &self.next {
            if context.status == 0 {
                next.run(context)?;
            }
        }
        println!("router middleware POST");
        Ok(())
    }
}

