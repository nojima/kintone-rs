pub mod client;
pub mod models;
pub mod v1;

mod internal;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
