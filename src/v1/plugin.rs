//! # Kintone Plugin API
//!
//! This module provides functions for inspecting, installing, updating and deleting plugins in
//! Kintone.
//!
//! ## Available Operations
//!
//! ### Plugin Information
//! - [`get_plugins`] - Retrieve installed plugins
//! - [`get_required_plugins`] - Retrieve required plugins
//! - [`get_plugin_apps`] - Retrieve apps that use a plugin
//!
//! ### Plugin Management
//! - [`add_plugin`] - Install an uploaded plugin
//! - [`update_plugin`] - Update an installed plugin
//! - [`delete_plugin`] - Delete an installed plugin
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! let response = kintone::v1::plugin::get_plugins()
//!     .offset(0)
//!     .limit(50)
//!     .send(&client)?;
//! for plugin in response.plugins {
//!     println!("{}: {}", plugin.id, plugin.name);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Plugin APIs use a regular space client and username/password authentication. Installing,
//! updating and deleting plugins requires kintone system administrator permissions.

use serde::{Deserialize, Serialize};

use super::app::settings::EmptyResponse;
use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::model::plugin::{Plugin, PluginApp, RequiredPlugin};

/// Retrieves the plugins installed in Kintone.
///
/// This function creates a request to get plugin IDs, names, descriptions, versions and marketplace
/// status. Results can be filtered by plugin IDs and paginated.
///
/// **Required Permissions:** This API requires no additional administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Optional Parameters
///
/// * `offset` - Sets the number of results to skip (default: 0)
/// * `limit` - Sets the maximum number of results to retrieve (1-100, default: 100)
/// * `ids` - Filters the results by plugin IDs (up to 100 IDs)
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::plugin::get_plugins()
///     .offset(0)
///     .limit(50)
///     .send(&client)?;
/// for plugin in response.plugins {
///     println!("{}: version {}", plugin.name, plugin.version);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-plugins/>
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
    /// Sets the number of results to skip (default: 0).
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    /// Sets the maximum number of results to retrieve (1-100, default: 100).
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    /// Filters the results by plugin IDs (up to 100 IDs).
    pub fn ids(mut self, values: impl IntoIterator<Item = String>) -> Self {
        let values: Vec<_> = values.into_iter().collect();
        self.builder = self.builder.query_array("ids", &values);
        self
    }

    /// Sends the request to retrieve the plugins installed in Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetPluginsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires no additional administrator permissions. Use username/password authentication;
    /// API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<GetPluginsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPluginsResponse {
    pub plugins: Vec<Plugin>,
}

/// Retrieves the plugins marked as required in Kintone.
///
/// This function creates a request to get required plugin IDs, names and marketplace status.
/// Results can be paginated.
///
/// **Required Permissions:** This API requires no additional administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Optional Parameters
///
/// * `offset` - Sets the number of results to skip (default: 0)
/// * `limit` - Sets the maximum number of results to retrieve (1-100, default: 100)
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::plugin::get_required_plugins()
///     .limit(50)
///     .send(&client)?;
/// for plugin in response.plugins {
///     println!("Required plugin: {}", plugin.name);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-required-plugins/>
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
    /// Sets the number of results to skip (default: 0).
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    /// Sets the maximum number of results to retrieve (1-100, default: 100).
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    /// Sends the request to retrieve the plugins marked as required in Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetRequiredPluginsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires no additional administrator permissions. Use username/password authentication;
    /// API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<GetRequiredPluginsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequiredPluginsResponse {
    pub plugins: Vec<RequiredPlugin>,
}

/// Retrieves the apps that use a Kintone plugin.
///
/// This function creates a request to get app IDs and names for apps using the specified plugin.
/// Results can be paginated.
///
/// **Required Permissions:** This API requires cybozu.com common administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the plugin to find apps for
///
/// # Optional Parameters
///
/// * `offset` - Sets the number of results to skip (default: 0)
/// * `limit` - Sets the maximum number of results to retrieve (1-500, default: 100)
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::plugin::get_plugin_apps("installed-plugin-id")
///     .offset(0)
///     .limit(100)
///     .send(&client)?;
/// for app in response.apps {
///     println!("{}: {}", app.id, app.name);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/get-plugin-apps/>
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
    /// Sets the number of results to skip (default: 0).
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    /// Sets the maximum number of results to retrieve (1-500, default: 100).
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    /// Sends the request to retrieve the apps that use a Kintone plugin.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetPluginAppsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires cybozu.com common administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
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

/// Installs a plugin in Kintone.
///
/// This function creates a request to install a plugin ZIP file that has already been uploaded with
/// [`crate::v1::file::upload`]. The response contains the plugin ID and version.
///
/// **Required Permissions:** This API requires kintone system administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `file_key` - The file key of the uploaded plugin ZIP file
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::plugin::add_plugin("uploaded-plugin-file-key")
///     .send(&client)?;
/// println!("Installed plugin {} (version {})", response.id, response.version);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/add-plugin/>
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
    /// Sends the request to install a plugin in Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`AddPluginResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddPluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type UpdatePluginResponse = PluginResponse;

/// Updates a plugin installed in Kintone.
///
/// This function creates a request to update the specified plugin using a ZIP file uploaded with
/// [`crate::v1::file::upload`]. The response contains the plugin ID and updated version.
///
/// **Required Permissions:** This API requires kintone system administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the installed plugin to update
/// * `file_key` - The file key of the uploaded plugin ZIP file
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::plugin::update_plugin(
///     "installed-plugin-id", "uploaded-plugin-file-key",
/// ).send(&client)?;
/// println!("Updated plugin {} to version {}", response.id, response.version);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/update-plugin/>
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
    /// Sends the request to update a plugin installed in Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdatePluginResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdatePluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type DeletePluginResponse = EmptyResponse;

/// Deletes an installed plugin from Kintone.
///
/// This function creates a request to remove the specified plugin from kintone. The API returns an
/// empty response on success.
///
/// **Required Permissions:** This API requires kintone system administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the installed plugin to delete
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::plugin::delete_plugin("installed-plugin-id").send(&client)?;
/// println!("Deleted plugin");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/plugins/delete-plugin/>
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
    /// Sends the request to delete an installed plugin from Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`DeletePluginResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<DeletePluginResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
