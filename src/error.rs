//! # Error Types
//!
//! This module defines error types used throughout the kintone crate.
//! All API operations return `Result<T, ApiError>` where errors can be categorized
//! into I/O errors or HTTP-specific errors.

/// HTTP-specific error containing status code and response body.
///
/// This error type is used when the HTTP request completes but returns
/// an error status code (4xx, 5xx). It includes both the status code
/// and the response body for detailed error analysis.
///
/// # Fields
/// * `status` - The HTTP status code (e.g., 404, 500)
/// * `body` - The response body as a string, which may contain error details from Kintone
#[derive(Debug, thiserror::Error)]
#[error("status={status}, body={body:?}")]
pub struct HttpError {
    pub status: u16,
    pub body: String,
}

/// The main error type for all Kintone API operations.
///
/// This enum represents all possible errors that can occur when interacting
/// with the Kintone API. It categorizes errors into I/O errors (network issues,
/// connection problems) and HTTP errors (API-specific error responses).
///
/// # Variants
/// * `Io` - I/O related errors such as network connectivity issues
/// * `Http` - HTTP-specific errors with status codes and response bodies
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
