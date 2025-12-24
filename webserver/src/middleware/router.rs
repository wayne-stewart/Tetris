use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;

use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;

pub type RouteHandler = fn(&mut HttpContext) -> Result<()>;

pub struct RouterMiddleware {
    pub routes: Arc<HashMap<&'static str, RouteHandler>>,
    next: RefCell<Option<Arc<dyn Middleware + Send + Sync>>>,
}

unsafe impl Sync for RouterMiddleware {}

impl RouterMiddleware {
    pub fn new(routes: Arc<HashMap<&'static str, RouteHandler>>) -> Self {
        RouterMiddleware {
            routes,
            next: RefCell::new(None),
        }
    }
}

impl Middleware for RouterMiddleware {
    fn run(&self, context: &mut HttpContext) -> Result<()> {
        if let Some(handler) = self.routes.get(context.path) {
            handler(context)?;
        }
        if let Some(next) = &*self.next.borrow() && context.status == 0
        {
            next.run(context)?;
        }
        Ok(())
    }

    fn set_next(&self, next: Arc<dyn Middleware + Send + Sync>) {
        self.next.borrow_mut().replace(next);
    }
}
