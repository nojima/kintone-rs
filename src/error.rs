//! # Error Types
//!
//! This module defines error types used throughout the kintone crate.
//! All API operations return `Result<T, ApiError>` where errors can be categorized
//! into I/O errors or HTTP-specific errors.

use serde::Deserialize;

/// HTTP-specific error containing status code and response body.
///
/// This error type is used when the HTTP request completes but returns
/// an error status code (4xx, 5xx). It includes both the status code
/// and the response body for detailed error analysis.
///
/// # Fields
/// * `status` - The HTTP status code (e.g., 404, 500)
/// * `body` - The response body as a string, which may contain error details from Kintone
#[derive(Debug, Clone, thiserror::Error)]
#[error("status={status}, body={body:?}")]
pub struct HttpError {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("status={status:?}, code={code:?}, id={id:?}, message={message:?}")]
pub struct KintoneError {
    pub status: u16,
    pub code: String,
    pub id: String,
    pub message: String,
}

#[derive(Deserialize)]
struct KintoneErrorJson {
    pub code: String,
    pub id: String,
    pub message: String,
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

    #[error("kintone error: {0}")]
    Kintone(#[from] KintoneError),
}

impl From<ureq::Error> for ApiError {
    fn from(err: ureq::Error) -> Self {
        Self::Io(err.into_io())
    }
}

impl From<http::Error> for ApiError {
    fn from(err: http::Error) -> Self {
        Self::Io(ureq::Error::from(err).into_io())
    }
}

fn is_json_response<T>(response: &http::Response<T>) -> bool {
    let Some(content_type) = response.headers().get(http::header::CONTENT_TYPE) else {
        return false;
    };
    let Ok(content_type) = content_type.to_str() else {
        return false;
    };
    // TODO: parse
    content_type == "application/json" || content_type.starts_with("application/json;")
}

impl From<http::Response<ureq::Body>> for ApiError {
    fn from(mut response: http::Response<ureq::Body>) -> ApiError {
        if !is_json_response(&response) {
            let status = response.status().as_u16();
            return match response.body_mut().read_to_string() {
                Ok(body) => ApiError::Http(HttpError { status, body }),
                Err(e) => ApiError::Io(e.into_io()),
            };
        };
        // If the response is JSON, attempt to parse it as KintoneError.
        match response.body_mut().read_json::<KintoneErrorJson>() {
            Ok(error_json) => KintoneError {
                status: response.status().as_u16(),
                code: error_json.code,
                id: error_json.id,
                message: error_json.message,
            }
            .into(),
            Err(io_error) => io_error.into(),
        }
    }
}
