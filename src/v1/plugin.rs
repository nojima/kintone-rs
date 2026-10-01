//! Plugin management APIs. These APIs use the regular space client.

use serde::{Deserialize, Serialize};

use super::app::settings::EmptyResponse;
use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::model::plugin::{Plugin, PluginApp, RequiredPlugin};

/// Get plugins.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-plugins/>
pub fn get_plugins() -> GetPluginsRequest {
    GetPluginsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/plugins.json"),
    }
}

#[must_use]
pub struct GetPluginsRequest {
    builder: RequestBuilder,
}

impl GetPluginsRequest {
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    pub fn ids(mut self, values: impl IntoIterator<Item = String>) -> Self {
        let values: Vec<_> = values.into_iter().collect();
        self.builder = self.builder.query_array("ids", &values);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetPluginsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPluginsResponse {
    pub plugins: Vec<Plugin>,
}

/// Get required plugins.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-required-plugins/>
pub fn get_required_plugins() -> GetRequiredPluginsRequest {
    GetRequiredPluginsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/plugins/required.json"),
    }
}

#[must_use]
pub struct GetRequiredPluginsRequest {
    builder: RequestBuilder,
}

impl GetRequiredPluginsRequest {
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetRequiredPluginsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequiredPluginsResponse {
    pub plugins: Vec<RequiredPlugin>,
}

/// Get plugin apps.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-plugin-apps/>
pub fn get_plugin_apps(id: impl Into<String>) -> GetPluginAppsRequest {
    GetPluginAppsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/plugin/apps.json")
            .query("id", id.into()),
    }
}

#[must_use]
pub struct GetPluginAppsRequest {
    builder: RequestBuilder,
}

impl GetPluginAppsRequest {
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetPluginAppsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPluginAppsResponse {
    pub apps: Vec<PluginApp>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginResponse {
    pub id: String,
    pub version: String,
}

pub type AddPluginResponse = PluginResponse;

/// Add plugin.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/add-plugin/>
pub fn add_plugin(file_key: impl Into<String>) -> AddPluginRequest {
    AddPluginRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/plugin.json"),
        body: AddPluginRequestBody {
            file_key: file_key.into(),
        },
    }
}

#[must_use]
pub struct AddPluginRequest {
    builder: RequestBuilder,
    body: AddPluginRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddPluginRequestBody {
    file_key: String,
}

impl AddPluginRequest {
    pub fn send(self, client: &KintoneClient) -> Result<AddPluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type UpdatePluginResponse = PluginResponse;

/// Update plugin.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/update-plugin/>
pub fn update_plugin(id: impl Into<String>, file_key: impl Into<String>) -> UpdatePluginRequest {
    UpdatePluginRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/plugin.json"),
        body: UpdatePluginRequestBody {
            id: id.into(),
            file_key: file_key.into(),
        },
    }
}

#[must_use]
pub struct UpdatePluginRequest {
    builder: RequestBuilder,
    body: UpdatePluginRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePluginRequestBody {
    id: String,
    file_key: String,
}

impl UpdatePluginRequest {
    pub fn send(self, client: &KintoneClient) -> Result<UpdatePluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type DeletePluginResponse = EmptyResponse;

/// Delete plugin.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/delete-plugin/>
pub fn delete_plugin(id: impl Into<String>) -> DeletePluginRequest {
    DeletePluginRequest {
        builder: RequestBuilder::new(http::Method::DELETE, "/v1/plugin.json"),
        body: DeletePluginRequestBody { id: id.into() },
    }
}

#[must_use]
pub struct DeletePluginRequest {
    builder: RequestBuilder,
    body: DeletePluginRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeletePluginRequestBody {
    id: String,
}

impl DeletePluginRequest {
    pub fn send(self, client: &KintoneClient) -> Result<DeletePluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
