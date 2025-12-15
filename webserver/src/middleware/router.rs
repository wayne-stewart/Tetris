use std::collections::HashMap;
use std::sync::Arc;

use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;

pub type RouteHandler = fn(&mut HttpContext) -> Result<()>;

pub struct RouterMiddleware {
    pub routes: Arc<HashMap<&'static str, RouteHandler>>,
    pub next: Option<Box<dyn Middleware + Send + Sync>>,
}

impl Middleware for RouterMiddleware {
    fn run(&self, context: &mut HttpContext) -> Result<()> {
        if let Some(handler) = self.routes.get(context.path) {
            handler(context)?;
        }
        if let Some(next) = &self.next && context.status == 0
        {
            next.run(context)?;
        }
        Ok(())
    }
}
