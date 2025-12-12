pub fn log_error(msg: &str) {
    println!("ERROR: {}", msg);
}

pub fn _log_debug(msg: &str) {
    println!("DEBUG: {}", msg);
}

pub fn log_request(ctx: &crate::server::HttpContext) {
    println!(
        "{} {} {} {} {}",
        ctx.client.id, ctx.client.request_count, ctx.status, ctx.verb, ctx.path_and_query
    );
}
