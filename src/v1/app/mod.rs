//! # Kintone App API
//!
//! This module provides functions for interacting with Kintone's app-related REST API endpoints.
//! It includes operations for retrieving app information and managing settings in the preview environment.
//!
//! ## Available Operations
//!
//! ### App Management
//! - [`add_app`] - Create a new app in the preview environment
//! - [`get_app`] - Retrieve information about a single app
//! - [`get_apps`] - Retrieve information about multiple apps
//! - [`get_app_statistics`] - Retrieve app usage statistics (wide course)
//! - [`get_app_admin_notes`] - Retrieve administrator notes
//! - [`update_app_admin_notes`] - Update administrator notes in the preview environment
//!
//! ### Settings Management
//! - [`settings::deploy_app`] - Deploy preview settings to the production environment
//! - [`settings`] - Manage general settings, workflows, notifications, permissions and plugins
//!
//! ### Form and View Management
//! - [`form`] - Manage form fields and layouts
//! - [`view`] - Manage list, calendar and custom views
//! - [`report`] - Manage graphs and periodic reports
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
pub mod report;
pub mod settings;
pub mod view;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::User;
use crate::model::app::statistics::AppStatistic;

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

/// Retrieves information about a single Kintone app.
///
/// This function creates a request to get the name, description, ownership and space information of
/// the specified app.
///
/// **Required Permissions:** This API requires record viewing or record creation permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::get_app(123).send(&client)?;
/// println!("App: {} (ID: {})", response.name, response.app_id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/get-app/>
pub fn get_app(app_id: u64) -> GetAppRequest {
    GetAppRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app.json").query("id", app_id),
    }
}

#[must_use]
pub struct GetAppRequest {
    builder: RequestBuilder,
}

impl GetAppRequest {
    /// Sends the request to retrieve information about a single Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetAppResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires record viewing or record creation permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppResponse, ApiError> {
        self.builder.call(client)
    }
}

/// Information about a single app.
pub type GetAppResponse = AppInfo;

/// Retrieves usage statistics for Kintone apps.
///
/// This function creates a request to get record counts, storage usage and other statistics for
/// apps the authenticated user can manage. Results can be paginated.
///
/// **Note**: This API is available only on the kintone wide course.
///
/// **Required Permissions:** This API requires app management permissions. Use username/password
/// authentication; API tokens cannot be used.
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
/// let response = kintone::v1::app::get_app_statistics()
///     .offset(0)
///     .limit(50)
///     .send(&client)?;
/// for app in response.apps {
///     println!("{}: {} records, {} bytes", app.name, app.record_count, app.storage_usage);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/get-apps-statistics/>
pub fn get_app_statistics() -> GetAppStatisticsRequest {
    GetAppStatisticsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/apps/statistics.json"),
    }
}

#[must_use]
pub struct GetAppStatisticsRequest {
    builder: RequestBuilder,
}

impl GetAppStatisticsRequest {
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

    /// Sends the request to retrieve usage statistics for Kintone apps.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetAppStatisticsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions. Use username/password authentication; API tokens
    /// cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppStatisticsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppStatisticsResponse {
    pub apps: Vec<AppStatistic>,
}

/// Retrieves the administrator notes for a Kintone app.
///
/// This function creates a request to get the administrator notes and whether they are included
/// when copying the app or creating a template.
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::get_app_admin_notes(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Administrator notes: {}", response.content);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/get-app-admin-notes/>
pub fn get_app_admin_notes(app_id: u64) -> GetAppAdminNotesRequest {
    GetAppAdminNotesRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/adminNotes.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetAppAdminNotesRequest {
    builder: RequestBuilder,
}

impl GetAppAdminNotesRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sends the request to retrieve the administrator notes for a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetAppAdminNotesResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppAdminNotesResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppAdminNotesResponse {
    pub content: String,
    pub include_in_template_and_duplicates: bool,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateAppAdminNotesResponse = settings::RevisionResponse;

/// Updates an app's administrator notes in the preview environment.
///
/// This function creates a request to update the notes and their inclusion in app copies and
/// templates. Only explicitly supplied properties are updated.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `content` - Sets the administrator notes as an HTML string
/// * `include_in_template_and_duplicates` - Sets whether administrator notes are included in app copies and templates
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::update_app_admin_notes(123)
///     .content("<p>Contact the sales team before editing this app.</p>")
///     .include_in_template_and_duplicates(false)
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated notes, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/update-app-admin-notes/>
pub fn update_app_admin_notes(app_id: u64) -> UpdateAppAdminNotesRequest {
    UpdateAppAdminNotesRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/adminNotes.json"),
        body: UpdateAppAdminNotesRequestBody {
            app: app_id,
            content: None,
            include_in_template_and_duplicates: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateAppAdminNotesRequest {
    builder: RequestBuilder,
    body: UpdateAppAdminNotesRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateAppAdminNotesRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_in_template_and_duplicates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateAppAdminNotesRequest {
    /// Sets the administrator notes as an HTML string.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = Some(content.into());
        self
    }

    /// Sets whether administrator notes are included in app copies and templates.
    pub fn include_in_template_and_duplicates(
        mut self,
        include_in_template_and_duplicates: bool,
    ) -> Self {
        self.body.include_in_template_and_duplicates = Some(include_in_template_and_duplicates);
        self
    }

    /// Sets the expected revision number for validation.
    ///
    /// If provided and the actual revision does not match, the request fails.
    /// Use `None` or omit this call to skip revision validation.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    /// Sends the request to update an app's administrator notes in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateAppAdminNotesResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateAppAdminNotesResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
