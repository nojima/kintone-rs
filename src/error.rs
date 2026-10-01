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

/// Error returned when one of the requests in a bulk request fails.
///
/// When a request in a bulk request fails, the whole bulk request is rolled back.
///
/// # Fields
/// * `index` - The index of the failed request in the bulk request
/// * `error` - The error returned for the failed request
#[derive(Debug, Clone, thiserror::Error)]
#[error("request #{index} failed: {error}")]
pub struct BulkRequestError {
    pub index: usize,
    pub error: KintoneError,
}

#[derive(Deserialize)]
struct BulkRequestErrorJson {
    results: Vec<serde_json::Value>,
}

impl BulkRequestError {
    /// Extracts the error of the failed request from the response body of a bulk request.
    ///
    /// Returns `None` if the body does not contain a failed request.
    pub(crate) fn from_http_error(err: &HttpError) -> Option<Self> {
        let json: BulkRequestErrorJson = serde_json::from_str(&err.body).ok()?;
        json.results.into_iter().enumerate().find_map(|(index, result)| {
            let error_json = serde_json::from_value::<KintoneErrorJson>(result).ok()?;
            Some(Self {
                index,
                error: KintoneError {
                    status: err.status,
                    code: error_json.code,
                    id: error_json.id,
                    message: error_json.message,
                },
            })
        })
    }
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
/// * `Json` - Errors in serializing requests or deserializing responses
/// * `Kintone` - Errors returned by Kintone
/// * `BulkRequest` - Errors returned by Kintone for a request in a bulk request
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("http error: {0}")]
    Http(#[from] HttpError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("kintone error: {0}")]
    Kintone(#[from] KintoneError),

    #[error("bulk request error: {0}")]
    BulkRequest(#[from] BulkRequestError),
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
    let Ok(content_type) = content_type.parse::<mime::Mime>() else {
        return false;
    };
    content_type.essence_str() == "application/json"
}

impl From<http::Response<ureq::Body>> for ApiError {
    fn from(mut response: http::Response<ureq::Body>) -> ApiError {
        const MAX_JSON_SIZE: u64 = 10 * 1024 * 1024;

        if !is_json_response(&response) {
            let status = response.status().as_u16();
            return match response.body_mut().read_to_string() {
                Ok(body) => ApiError::Http(HttpError { status, body }),
                Err(e) => ApiError::Io(e.into_io()),
            };
        };
        // If the response is JSON, attempt to parse it as KintoneError.
        let body = match response.body_mut().with_config().limit(MAX_JSON_SIZE).read_to_vec() {
            Ok(body) => body,
            Err(e) => return e.into(),
        };
        let status = response.status().as_u16();
        match serde_json::from_slice::<KintoneErrorJson>(&body) {
            Ok(error_json) => KintoneError {
                status,
                code: error_json.code,
                id: error_json.id,
                message: error_json.message,
            }
            .into(),
            // Some APIs (e.g. bulk request) return errors in a different format.
            // Keep the body so that the caller can inspect it.
            Err(_) => HttpError {
                status,
                body: String::from_utf8_lossy(&body).into_owned(),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_request_error_from_http_error() {
        // Taken from https://cybozu.dev/ja/kintone/docs/rest-api/records/bulk-request/
        let err = HttpError {
            status: 404,
            body: r#"{
                "results": [
                    {},
                    {
                        "message": "指定したレコード（id: 33）が見つかりません。",
                        "id": "1505999166-1940353231",
                        "code": "GAIA_RE01"
                    },
                    {}
                ]
            }"#
            .to_owned(),
        };
        let bulk_err = BulkRequestError::from_http_error(&err).unwrap();
        assert_eq!(bulk_err.index, 1);
        assert_eq!(bulk_err.error.status, 404);
        assert_eq!(bulk_err.error.code, "GAIA_RE01");
        assert_eq!(bulk_err.error.id, "1505999166-1940353231");
    }

    #[test]
    fn bulk_request_error_from_unrelated_http_error() {
        let err = HttpError {
            status: 500,
            body: "Internal Server Error".to_owned(),
        };
        assert!(BulkRequestError::from_http_error(&err).is_none());
    }
}
