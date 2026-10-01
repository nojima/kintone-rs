//! API discovery and schema information. Use a regular space client.

use std::collections::HashMap;

use serde::Deserialize;

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
/// Get apis.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apis/get-apis/>
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

/// Retrieves a schema by API ID, e.g. `record/get` from `get_apis`.
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apis/get-api-schema/>
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
