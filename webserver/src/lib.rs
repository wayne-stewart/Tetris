pub mod http;
pub mod log;
pub mod middleware;
pub mod server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

