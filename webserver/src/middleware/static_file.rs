use std::fs::File;
use std::path::PathBuf;

use crate::server::Middleware;
use crate::server::HttpContext;

pub struct StaticFileMiddlware<'a> {
    pub wwwroot: &'a str,
    pub next: Option<Box<dyn Middleware + Send + Sync>>,
}

impl StaticFileMiddlware<'_> {
    fn handle_static_file(&self, http_context: &mut HttpContext) -> std::io::Result<()> {
        let mut path_buf = PathBuf::from(self.wwwroot);
        path_buf.push(http_context.path.trim_start_matches('/'));
        match File::open(path_buf) {
            Ok(file) => {
                crate::http::send_file(http_context, file)?;
            }
            Err(_) => {
                crate::http::send_404(http_context)?;
            }
        };
        Ok(())
    }
}

impl Middleware for StaticFileMiddlware<'_> {
    fn run(&self, context: &mut HttpContext) -> std::io::Result<()> {
        //println!("static file middleware PRE");

        if context.verb.eq("GET") {
            self.handle_static_file(context)?;
        } else {
            crate::http::send_405(context)?
        }

        if let Some(next) = &self.next {
            if context.status == 0 {
                next.run(context)?;
            }
        }
        //println!("static file middleware POST");
        Ok(())
    }
}


