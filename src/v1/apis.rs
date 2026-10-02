//! # Kintone API Discovery
//!
//! This module provides functions for discovering Kintone REST APIs and retrieving their request
//! and response schemas.
//!
//! ## Available Operations
//!
//! - [`get_apis`] - Retrieve API IDs and schema links
//! - [`get_api_schema`] - Retrieve a specific API schema
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! let response = kintone::v1::apis::get_api_schema("records/get")
//!     .send(&client)?;
//! println!("{} {}", response.http_method, response.path);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: API discovery uses a regular space client. The API specification does not require access
//! permissions or authentication.

use std::collections::HashMap;

use serde::Deserialize;

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;

/// Retrieves the available Kintone REST API definitions.
///
/// This function creates a request to discover API IDs and their schema links. An API ID from the
/// response can be passed to [`get_api_schema`].
///
/// **Required Permissions:** The API specification does not require authentication or access
/// permissions.
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::apis::get_apis().send(&client)?;
/// for (id, api) in response.apis {
///     println!("{}: {}", id, api.link);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apis/get-apis/>
pub fn get_apis() -> GetApisRequest {
    GetApisRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/apis.json"),
    }
}

#[must_use]
pub struct GetApisRequest {
    builder: RequestBuilder,
}

impl GetApisRequest {
    /// Sends the request to retrieve the available Kintone REST API definitions.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetApisResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// The API specification does not require authentication or access permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetApisResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisResponse {
    pub base_url: String,
    pub apis: HashMap<String, ApiLink>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLink {
    pub link: String,
}

/// Retrieves the schema of a Kintone REST API.
///
/// This function creates a request to get an API's HTTP method, path, request schema and response
/// schema. Use an API ID returned by [`get_apis`], such as `records/get`.
///
/// **Required Permissions:** The API specification does not require authentication or access
/// permissions.
///
/// # Arguments
///
/// * `api_id` - The API ID returned by `get_apis`, such as `records/get`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::apis::get_api_schema("records/get").send(&client)?;
/// println!("{} {}", response.http_method, response.path);
/// println!("Request schema: {}", response.request);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apis/get-api-schema/>
pub fn get_api_schema(api_id: impl AsRef<str>) -> GetApiSchemaRequest {
    let path = api_id
        .as_ref()
        .split('/')
        .map(|segment| {
            percent_encoding::utf8_percent_encode(segment, percent_encoding::NON_ALPHANUMERIC)
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("/");
    GetApiSchemaRequest {
        builder: RequestBuilder::new(http::Method::GET, format!("/v1/apis/{path}.json")),
    }
}

#[must_use]
pub struct GetApiSchemaRequest {
    builder: RequestBuilder,
}

impl GetApiSchemaRequest {
    /// Sends the request to retrieve the schema of a Kintone REST API.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetApiSchemaResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// The API specification does not require authentication or access permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetApiSchemaResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiSchemaResponse {
    pub id: String,
    pub base_url: String,
    pub path: String,
    pub http_method: String,
    pub request: serde_json::Value,
    pub response: serde_json::Value,
    pub schemas: HashMap<String, serde_json::Value>,
}
