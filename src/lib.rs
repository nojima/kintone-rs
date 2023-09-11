pub mod client;
pub mod models;
pub mod v1;

mod internal;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BoxError>;
