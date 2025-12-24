use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;
use std::sync::Arc;
use std::cell::RefCell;

pub struct RequestLoggingMiddleware {
    next: RefCell<Option<Arc<dyn Middleware + Send + Sync>>>,
}

impl RequestLoggingMiddleware {
    pub fn new() -> Self {
        RequestLoggingMiddleware {
            next: RefCell::new(None),
        }
    }
}

unsafe impl Sync for RequestLoggingMiddleware {}

impl Middleware for RequestLoggingMiddleware {
    fn run(&self, context: &mut HttpContext) -> Result<()> {
        if let Some(next) = &*self.next.borrow() && context.status == 0
        {
            (*next).run(context)?;
        }
        if context.status == 0 {
            crate::http::send_404(context)?;
        }
        crate::log::log_request(context);
        Ok(())
    }

    fn set_next(&self, next: Arc<dyn Middleware + Send + Sync>) {
        self.next.borrow_mut().replace(next);
    }
}
