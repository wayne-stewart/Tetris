use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::cell::RefCell;

use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;

pub struct StaticFileMiddlware {
    pub wwwroot: String,
    next: RefCell<Option<Arc<dyn Middleware + Send + Sync>>>,
}

unsafe impl Sync for StaticFileMiddlware {}

impl StaticFileMiddlware {
    pub fn new(wwwroot: &str) -> Self {
        StaticFileMiddlware {
            wwwroot: String::from(wwwroot),
            next: RefCell::new(None),
        }
    }

    fn handle_static_file(&self, context: &mut HttpContext) -> Result<()> {
        let mut path_buf = PathBuf::from(&self.wwwroot);
        path_buf.push(context.path.trim_start_matches('/'));
        if let Ok(file) = File::open(path_buf) {
            crate::http::send_file(context, file)?;
        }
        Ok(())
    }
}

impl Middleware for StaticFileMiddlware {
    fn run(&self, context: &mut HttpContext) -> Result<()> {

        if context.verb.eq("GET") {
            self.handle_static_file(context)?;
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
