use std::fs::File;
use std::path::PathBuf;

use crate::server::HttpContext;
use crate::server::Middleware;
use crate::Result;

pub struct StaticFileMiddlware<'a> {
    pub wwwroot: &'a str,
    pub next: Option<Box<dyn Middleware + Send + Sync>>,
}

impl StaticFileMiddlware<'_> {
    fn handle_static_file(&self, context: &mut HttpContext) -> Result<()> {
        let mut path_buf = PathBuf::from(self.wwwroot);
        path_buf.push(context.path.trim_start_matches('/'));
        if let Ok(file) = File::open(path_buf) {
            crate::http::send_file(context, file)?;
        }
        Ok(())
    }
}

impl Middleware for StaticFileMiddlware<'_> {
    fn run(&self, context: &mut HttpContext) -> Result<()> {

        if context.verb.eq("GET") {
            self.handle_static_file(context)?;
        } 

        if let Some(next) = &self.next && context.status == 0
        {
            next.run(context)?;
        }
        
        Ok(())
    }
}
