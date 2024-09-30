pub mod client;
pub mod models;
pub mod v1;

mod internal;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxResult<T> = std::result::Result<T, BoxError>;

#[derive(Debug, thiserror::Error)]
#[error("status={status}, body={body:?}")]
pub struct HttpError {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("http error: {0}")]
    Http(#[from] HttpError),
}

impl From<ureq::Error> for ApiError {
    fn from(err: ureq::Error) -> Self {
        match err {
            ureq::Error::Status(status, response) => {
                let body = response.into_string().unwrap_or_default();
                HttpError { status, body }.into()
            }
            _ => err.into(),
        }
    }
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
