//! # Kintone App API
//!
//! This module provides functions for interacting with Kintone's app-related REST API endpoints.
//! It includes operations for creating and managing apps in the preview environment.
//!
//! ## Available Operations
//!
//! ### App Management
//! - [`add_app`] - Create a new app in the preview environment
//! - [`get_apps`] - Retrieve information about multiple apps
//!
//! ### Settings Management
//! - [`settings::deploy_app`] - Deploy app settings from preview to production environment
//!
//! ### Form Management
//! - [`form::add_form_field`] - Add fields to an app's form in the preview environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! // Create a new app (requires username/password auth)
//! let response = kintone::v1::app::add_app("My App").send(&client)?;
//! println!("Created app with ID: {}", response.app);
//!
//! // Get app information (can use API tokens)
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::api_token("token".to_owned()));
//! let response = kintone::v1::app::get_apps()
//!     .codes(["PROJECT", "TASK"])
//!     .send(&client)?;
//! for app in response.apps {
//!     println!("App: {} (ID: {})", app.name, app.app_id);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Some app APIs like [`add_app`] require username/password authentication and cannot use API tokens.

pub mod form;
pub mod settings;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::User;

/// Creates a new app in the preview environment.
///
/// This function creates a request to add a new app to Kintone's preview environment.
/// The preview environment is a temporary location where app information is stored
/// before being deployed to the production environment.
///
/// **Important**: This API requires username/password authentication and cannot use API tokens.
///
/// **Important**: Apps created with this function exist only in the preview environment.
/// To deploy the app to the production environment, use [`settings::deploy_app`].
///
/// # Arguments
/// * `name` - The name of the app (up to 64 characters)
/// * `space` (optional) - The space ID where the app should be created
/// * `thread` (optional) - The thread ID within the space where the app should be created
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::add_app("Project Management App")
///     .space(10) // optional
///     .thread(11) // optional
///     .send(&client)?;
/// println!("Created app with ID: {}", response.app);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/add-app/>
pub fn add_app(name: impl Into<String>) -> AddAppRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/preview/app.json");
    AddAppRequest {
        builder,
        body: AddAppRequestBody {
            name: name.into(),
            space: None,
            thread: None,
        },
    }
}

/// Retrieves information about multiple apps.
///
/// This function creates a request to get information about apps that match the specified criteria.
/// You can filter apps by IDs, codes, names, or space IDs. A maximum of 100 apps can be retrieved per request.
///
/// # Optional Parameters
/// * `ids` - Array of app IDs (up to 100 IDs)
/// * `codes` - Array of app codes (up to 100 codes)  
/// * `name` - App name or partial name (case-insensitive partial match)
/// * `space_ids` - Array of space IDs (up to 100 IDs)
/// * `offset` - Number of apps to skip from the beginning (default: 0)
/// * `limit` - Number of apps to retrieve (1-100, default: 100)
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::api_token("token".to_owned()));
/// // Get all apps
/// let response = kintone::v1::app::get_apps()
///     .send(&client)?;
///
/// // Get apps by codes
/// let response = kintone::v1::app::get_apps()
///     .codes(["PROJECT", "TASK"])
///     .send(&client)?;
///
/// // Get apps by name with pagination  
/// let response = kintone::v1::app::get_apps()
///     .name("Management")
///     .offset(0)
///     .limit(50)
///     .send(&client)?;
///
/// for app in response.apps {
///     println!("App: {} (ID: {})", app.name, app.app_id);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/get-apps/>
pub fn get_apps() -> GetAppsRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/apps.json");
    GetAppsRequest { builder }
}

#[must_use]
pub struct AddAppRequest {
    builder: RequestBuilder,
    body: AddAppRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddAppRequestBody {
    name: String,
    space: Option<u64>,
    thread: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAppResponse {
    #[serde(with = "stringified")]
    pub app: u64,
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl AddAppRequest {
    /// Sets the space ID where the app should be created.
    ///
    /// This is used when creating an app within a specific space.
    /// Both `space` and `thread` should be specified together.
    pub fn space(mut self, space: u64) -> Self {
        self.body.space = Some(space);
        self
    }

    /// Sets the thread ID within the space where the app should be created.
    ///
    /// This is used when creating an app within a specific thread in a space.
    /// Both `space` and `thread` should be specified together.
    pub fn thread(mut self, thread: u64) -> Self {
        self.body.thread = Some(thread);
        self
    }

    /// Sends the request to create the app.
    ///
    /// # Returns
    /// A Result containing the AddAppResponse with the app ID and revision, or an ApiError.
    ///
    /// # Authentication
    /// This API requires username/password authentication. API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddAppResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

#[must_use]
pub struct GetAppsRequest {
    builder: RequestBuilder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppsResponse {
    pub apps: Vec<AppInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    #[serde(with = "stringified")]
    pub app_id: u64,
    pub code: String,
    pub name: String,
    pub description: String,
    #[serde(with = "option_stringified")]
    pub space_id: Option<u64>,
    #[serde(with = "option_stringified")]
    pub thread_id: Option<u64>,
    pub created_at: DateTime<FixedOffset>,
    pub creator: User,
    pub modified_at: DateTime<FixedOffset>,
    pub modifier: User,
}

impl GetAppsRequest {
    /// Sets the app IDs to filter by.
    ///
    /// Maximum of 100 app IDs can be specified.
    pub fn ids<I, T>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        let id_strings: Vec<String> = ids.into_iter().map(|id| id.into().to_string()).collect();
        self.builder = self.builder.query_array("ids", &id_strings);
        self
    }

    /// Sets the app codes to filter by.
    ///
    /// Maximum of 100 app codes can be specified.
    pub fn codes<I, T>(mut self, codes: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let code_strings: Vec<String> = codes.into_iter().map(Into::into).collect();
        self.builder = self.builder.query_array("codes", &code_strings);
        self
    }

    /// Sets the app name to search for (partial match, case-insensitive).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.builder = self.builder.query("name", name.into());
        self
    }

    /// Sets the space IDs to filter by.
    ///
    /// Maximum of 100 space IDs can be specified.
    pub fn space_ids<I, T>(mut self, space_ids: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        let space_id_strings: Vec<String> =
            space_ids.into_iter().map(|id| id.into().to_string()).collect();
        self.builder = self.builder.query_array("spaceIds", &space_id_strings);
        self
    }

    /// Sets the number of apps to skip from the beginning.
    ///
    /// Default is 0 if not specified.
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset.to_string());
        self
    }

    /// Sets the maximum number of apps to retrieve.
    ///
    /// Must be between 1 and 100. Default is 100 if not specified.
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit.to_string());
        self
    }

    /// Sends the request to get the apps.
    ///
    /// # Returns
    /// A Result containing the GetAppsResponse with app information, or an ApiError.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppsResponse, ApiError> {
        self.builder.call(client)
    }
}
