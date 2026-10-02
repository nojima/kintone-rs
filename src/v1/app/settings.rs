//! # Kintone App Settings API
//!
//! This module provides functions for retrieving and updating app settings in Kintone
//! and deploying preview settings to the production environment.
//!
//! ## Available Operations
//!
//! ### Deployment
//! - [`deploy_app`] - Deploy or revert app settings
//! - [`get_app_deploy_status`] - Check deployment status
//!
//! ### General Settings and Process Management
//! - [`get_general_settings`] - Retrieve general app settings
//! - [`update_general_settings`] - Update general settings in the preview environment
//! - [`get_process_management_settings`] - Retrieve workflow settings
//! - [`update_process_management_settings`] - Update workflows in the preview environment
//! - [`get_customization`] - Retrieve JavaScript and CSS customization settings
//! - [`update_customization`] - Update customization in the preview environment
//!
//! ### Notifications
//! - [`get_general_notification_settings`] - Retrieve event notification settings
//! - [`update_general_notification_settings`] - Update event notifications in the preview environment
//! - [`get_per_record_notification_settings`] - Retrieve per-record notification settings
//! - [`update_per_record_notification_settings`] - Update per-record notifications in the preview environment
//! - [`get_reminder_notification_settings`] - Retrieve reminder notification settings
//! - [`update_reminder_notification_settings`] - Update reminders in the preview environment
//!
//! ### Access Permissions
//! - [`get_app_permissions`] - Retrieve app access rights
//! - [`update_app_permissions`] - Update app access rights
//! - [`get_record_permissions`] - Retrieve record access rules
//! - [`update_record_permissions`] - Update record access rules
//! - [`get_field_permissions`] - Retrieve field access rules
//! - [`update_field_permissions`] - Update field access rules
//!
//! ### Actions and Plugins
//! - [`get_action_settings`] - Retrieve app action settings
//! - [`update_action_settings`] - Update actions in the preview environment
//! - [`get_app_plugins`] - Retrieve plugins configured for an app
//! - [`add_app_plugins`] - Add installed plugins in the preview environment
//! - [`move_app`] - Change the space containing an app
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! // Deploy apps
//! kintone::v1::app::settings::deploy_app()
//!     .app(123, Some(45)) // app ID with optional revision
//!     .app(124, None)     // app ID without revision check
//!     .send(&client)?;
//!
//! // Check deployment status
//! let status = kintone::v1::app::settings::get_app_deploy_status()
//!     .app(123)
//!     .app(124)
//!     .send(&client)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Most settings updates use the preview environment. Access permission updates use
//! the live environment by default and support `preview(true)` to select preview settings.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::app::settings::{
    AppAction, AppIcon, AppPlugin, AppRight, Customization, CustomizationScope, FieldRight,
    GeneralNotification, NumberPrecision, PerRecordNotification, ProcessAction, ProcessState,
    RecordRight, ReminderNotification, Theme, TitleField,
};

/// Deploys app settings from the preview environment to the production environment.
///
/// This function creates a request to deploy app settings that have been configured
/// in the preview environment to the production environment. This is equivalent to
/// clicking the "Deploy App" or "Cancel Changes" button in the app settings interface.
///
/// - This is an asynchronous API. Use the [`get_app_deploy_status`] API to check completion.
/// - Multiple apps can be deployed in a single request (max 300 apps).
/// - If any app fails to deploy, all specified apps will be reverted to their previous state.
/// - Guest space apps can only be deployed with other apps from the same guest space.
///
/// **Required Permissions:** App management permissions
///
/// # Arguments
///
/// Use the builder pattern to specify apps for deployment:
/// - `app(app_id, revision)` - Add an app to deploy with optional revision check
/// - `revert(true/false)` - Whether to cancel changes instead of deploying them
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// // Deploy multiple apps
/// let response = kintone::v1::app::settings::deploy_app()
///     .app(123, Some(45))  // Deploy app 123 with revision check
///     .app(124, None)      // Deploy app 124 without revision check
///     .revert(false)       // Deploy changes (default)
///     .send(&client)?;
///
/// // Cancel changes instead of deploying
/// let response = kintone::v1::app::settings::deploy_app()
///     .app(123, None)
///     .revert(true)        // Cancel changes
///     .send(&client)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/deploy-app-settings/>
pub fn deploy_app() -> DeployAppRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/preview/app/deploy.json");
    DeployAppRequest {
        builder,
        body: DeployAppRequestBody {
            apps: Vec::new(),
            revert: None,
        },
    }
}

#[must_use]
pub struct DeployAppRequest {
    builder: RequestBuilder,
    body: DeployAppRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeployAppRequestBody {
    apps: Vec<AppDeployInfo>,
    revert: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppDeployInfo {
    #[serde(with = "stringified")]
    app: u64,
    #[serde(with = "option_stringified")]
    revision: Option<u64>,
}

impl DeployAppRequest {
    /// Adds an app to be deployed.
    ///
    /// # Arguments
    /// * `app_id` - The ID of the app to deploy
    /// * `revision` - Optional revision number for validation. If provided and doesn't match
    ///   the actual revision, an error will be returned. Use `None` to skip validation.
    ///
    /// # Example
    /// ```no_run
    /// let request = kintone::v1::app::settings::deploy_app()
    ///     .app(123, Some(45))  // Deploy with revision check
    ///     .app(124, None);     // Deploy without revision check
    /// ```
    pub fn app(mut self, app_id: u64, revision: Option<u64>) -> Self {
        self.body.apps.push(AppDeployInfo {
            app: app_id,
            revision,
        });
        self
    }

    /// Sets whether to revert (cancel) changes instead of deploying them.
    pub fn revert(mut self, revert: bool) -> Self {
        self.body.revert = Some(revert);
        self
    }

    /// Sends the request to deploy app settings.
    ///
    /// **Note**: This is an asynchronous operation. Use the [`get_app_deploy_status`] API to check
    /// if the deployment has completed successfully.
    ///
    /// # Authentication
    /// Requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<DeployAppResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployAppResponse {}

/// Checks the deployment status of app settings.
///
/// This function creates a request to check the status of app deployments that were
/// initiated with the deploy_app API. Since deployment is an asynchronous operation,
/// this API allows you to monitor the progress and completion of the deployment process.
///
/// - Can check the status of up to 300 apps in a single request
/// - Returns the current status for each app: PROCESSING, SUCCESS, FAIL, or CANCEL
/// - Guest space apps can only be checked with other apps from the same guest space
///
/// **Required Permissions:** App management permissions
///
/// # Arguments
///
/// Use the builder pattern to specify apps to check:
/// - `app(app_id)` - Add an app ID to check deployment status
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// // Check deployment status for multiple apps
/// let status = kintone::v1::app::settings::get_app_deploy_status()
///     .app(123)
///     .app(124)
///     .app(125)
///     .send(&client)?;
///
/// use kintone::v1::app::settings::DeployStatus;
/// for app_status in status.apps {
///     match app_status.status {
///         DeployStatus::Processing => println!("App {} is still deploying", app_status.app),
///         DeployStatus::Success => println!("App {} deployed successfully", app_status.app),
///         DeployStatus::Fail => println!("App {} deployment failed", app_status.app),
///         DeployStatus::Cancel => println!("App {} deployment was cancelled", app_status.app),
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-app-deploy-status/>
pub fn get_app_deploy_status() -> GetAppDeployStatusRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/preview/app/deploy.json");
    GetAppDeployStatusRequest {
        builder,
        body: GetAppDeployStatusRequestBody { apps: Vec::new() },
    }
}

#[must_use]
pub struct GetAppDeployStatusRequest {
    builder: RequestBuilder,
    body: GetAppDeployStatusRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetAppDeployStatusRequestBody {
    apps: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppDeployStatusResponse {
    pub apps: Vec<AppDeployStatus>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDeployStatus {
    #[serde(with = "stringified")]
    pub app: u64,
    pub status: DeployStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeployStatus {
    /// Deployment is in progress
    Processing,
    /// Deployment completed successfully
    Success,
    /// Deployment failed
    Fail,
    /// Deployment was cancelled due to another app's failure
    Cancel,
}

impl GetAppDeployStatusRequest {
    /// Adds an app ID to check deployment status.
    pub fn app(mut self, app_id: u64) -> Self {
        self.body.apps.push(app_id);
        self
    }

    /// Sends the request to check app deployment status.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppDeployStatusResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RevisionResponse {
    #[serde(with = "stringified")]
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptyResponse {}

/// Retrieves the general settings of a Kintone app.
///
/// This function creates a request to get the app name, description, icon, theme, record title and
/// numeric precision settings.
///
/// **Required Permissions:** Viewing live settings requires record viewing or record creation
/// permissions. Viewing preview settings requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_general_settings(123)
///     .lang("en")
///     .preview(true)
///     .send(&client)?;
/// println!("{}: {}", response.name, response.description);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-general-settings/>
pub fn get_general_settings(app_id: u64) -> GetGeneralSettingsRequest {
    GetGeneralSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/settings.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetGeneralSettingsRequest {
    builder: RequestBuilder,
}

impl GetGeneralSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve the general settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetGeneralSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// Viewing live settings requires record viewing or record creation permissions. Viewing preview
    /// settings requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetGeneralSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGeneralSettingsResponse {
    pub name: String,
    pub description: String,
    pub icon: AppIcon,
    pub theme: Theme,
    pub title_field: TitleField,
    pub enable_thumbnails: bool,
    pub enable_bulk_deletion: bool,
    pub enable_comments: bool,
    pub enable_duplicate_record: bool,
    pub enable_inline_record_editing: bool,
    pub number_precision: NumberPrecision,
    #[serde(with = "stringified")]
    pub first_month_of_fiscal_year: u64,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateGeneralSettingsResponse = RevisionResponse;

/// Updates an app's general settings in the preview environment.
///
/// This function creates a request to update the app name, appearance and record behavior. Only
/// explicitly supplied settings are updated.
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
/// * `name` - Sets the app name
/// * `description` - Sets the app description as an HTML string
/// * `icon` - Sets the app icon to a preset icon or an uploaded file
/// * `theme` - Sets the app color or design theme
/// * `title_field` - Sets how the record title is selected and which field supplies it
/// * `enable_thumbnails` - Sets whether attachment thumbnails are displayed
/// * `enable_bulk_deletion` - Sets whether all records can be deleted together
/// * `enable_comments` - Sets whether record comments are enabled
/// * `enable_duplicate_record` - Sets whether records can be duplicated
/// * `enable_inline_record_editing` - Sets whether records can be edited directly in list views
/// * `number_precision` - Sets the numeric precision, decimal places and rounding mode
/// * `first_month_of_fiscal_year` - Sets the first month of the fiscal year (1-12)
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::Theme;
///
/// let response = kintone::v1::app::settings::update_general_settings(123)
///     .name("Customer Management")
///     .description("<p>Customer information for the sales team.</p>")
///     .theme(Theme::Blue)
///     .enable_comments(true)
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated settings, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-general-settings/>
pub fn update_general_settings(app_id: u64) -> UpdateGeneralSettingsRequest {
    UpdateGeneralSettingsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/settings.json"),
        body: UpdateGeneralSettingsRequestBody {
            app: app_id,
            name: None,
            description: None,
            icon: None,
            theme: None,
            title_field: None,
            enable_thumbnails: None,
            enable_bulk_deletion: None,
            enable_comments: None,
            enable_duplicate_record: None,
            enable_inline_record_editing: None,
            number_precision: None,
            first_month_of_fiscal_year: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateGeneralSettingsRequest {
    builder: RequestBuilder,
    body: UpdateGeneralSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateGeneralSettingsRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<AppIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<Theme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title_field: Option<TitleField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_thumbnails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_bulk_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_duplicate_record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_inline_record_editing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_precision: Option<NumberPrecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_month_of_fiscal_year: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateGeneralSettingsRequest {
    /// Sets the app name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// Sets the app description as an HTML string.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// Sets the app icon to a preset icon or an uploaded file.
    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.body.icon = Some(icon);
        self
    }

    /// Sets the app color or design theme.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.body.theme = Some(theme);
        self
    }

    /// Sets how the record title is selected and which field supplies it.
    pub fn title_field(mut self, title_field: TitleField) -> Self {
        self.body.title_field = Some(title_field);
        self
    }

    /// Sets whether attachment thumbnails are displayed.
    pub fn enable_thumbnails(mut self, enable_thumbnails: bool) -> Self {
        self.body.enable_thumbnails = Some(enable_thumbnails);
        self
    }

    /// Sets whether all records can be deleted together.
    pub fn enable_bulk_deletion(mut self, enable_bulk_deletion: bool) -> Self {
        self.body.enable_bulk_deletion = Some(enable_bulk_deletion);
        self
    }

    /// Sets whether record comments are enabled.
    pub fn enable_comments(mut self, enable_comments: bool) -> Self {
        self.body.enable_comments = Some(enable_comments);
        self
    }

    /// Sets whether records can be duplicated.
    pub fn enable_duplicate_record(mut self, enable_duplicate_record: bool) -> Self {
        self.body.enable_duplicate_record = Some(enable_duplicate_record);
        self
    }

    /// Sets whether records can be edited directly in list views.
    pub fn enable_inline_record_editing(mut self, enable_inline_record_editing: bool) -> Self {
        self.body.enable_inline_record_editing = Some(enable_inline_record_editing);
        self
    }

    /// Sets the numeric precision, decimal places and rounding mode.
    pub fn number_precision(mut self, number_precision: NumberPrecision) -> Self {
        self.body.number_precision = Some(number_precision);
        self
    }

    /// Sets the first month of the fiscal year (1-12).
    pub fn first_month_of_fiscal_year(mut self, first_month_of_fiscal_year: u64) -> Self {
        self.body.first_month_of_fiscal_year = Some(first_month_of_fiscal_year);
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

    /// Sends the request to update an app's general settings in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateGeneralSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateGeneralSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves an app's process management settings.
///
/// This function creates a request to get workflow states, assignees and actions. States and
/// actions can be absent when process management is disabled.
///
/// **Required Permissions:** Viewing live settings requires record viewing or record creation
/// permissions. Viewing preview settings requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_process_management_settings(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Process management enabled: {}", response.enable);
/// if let Some(states) = response.states {
///     for (name, state) in states {
///         println!("{}: position {}", name, state.index);
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-process-management-settings/>
pub fn get_process_management_settings(app_id: u64) -> GetProcessManagementSettingsRequest {
    GetProcessManagementSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/status.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetProcessManagementSettingsRequest {
    builder: RequestBuilder,
}

impl GetProcessManagementSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve an app's process management settings.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetProcessManagementSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// Viewing live settings requires record viewing or record creation permissions. Viewing preview
    /// settings requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<GetProcessManagementSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessManagementSettingsResponse {
    pub enable: bool,
    #[serde(default)]
    pub states: Option<HashMap<String, ProcessState>>,
    #[serde(default)]
    pub actions: Option<Vec<ProcessAction>>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateProcessManagementSettingsResponse = RevisionResponse;

/// Updates process management settings in the preview environment.
///
/// This function creates a request to enable or disable process management and update its states,
/// assignees and actions. Settings not supplied to the builder are omitted from the request.
/// Include all states and actions to retain when supplying those collections.
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
/// * `enable` - Sets whether process management is enabled
/// * `states` - Sets the workflow states, indexed by their existing names
/// * `state` - Adds or replaces a single workflow state
/// * `actions` - Sets the workflow actions
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::ProcessState;
///
/// let response = kintone::v1::app::settings::update_process_management_settings(123)
///     .enable(true)
///     .state("Open", ProcessState { index: 0, ..Default::default() })
///     .actions([])
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated workflow, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-process-management-settings/>
pub fn update_process_management_settings(app_id: u64) -> UpdateProcessManagementSettingsRequest {
    UpdateProcessManagementSettingsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/status.json"),
        body: UpdateProcessManagementSettingsRequestBody {
            app: app_id,
            enable: None,
            states: None,
            actions: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateProcessManagementSettingsRequest {
    builder: RequestBuilder,
    body: UpdateProcessManagementSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProcessManagementSettingsRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<HashMap<String, ProcessState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<ProcessAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateProcessManagementSettingsRequest {
    /// Sets whether process management is enabled.
    pub fn enable(mut self, enable: bool) -> Self {
        self.body.enable = Some(enable);
        self
    }

    /// Sets the complete collection of workflow states, indexed by their existing names.
    pub fn states(mut self, values: impl IntoIterator<Item = (String, ProcessState)>) -> Self {
        self.body.states = Some(values.into_iter().collect());
        self
    }

    /// Adds or replaces a single workflow state.
    pub fn state(mut self, name: impl Into<String>, value: ProcessState) -> Self {
        self.body.states.get_or_insert_with(HashMap::new).insert(name.into(), value);
        self
    }

    /// Sets the complete collection of workflow actions.
    pub fn actions(mut self, values: impl IntoIterator<Item = ProcessAction>) -> Self {
        self.body.actions = Some(values.into_iter().collect());
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

    /// Sends the request to update process management settings in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateProcessManagementSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateProcessManagementSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves an app's JavaScript and CSS customization settings.
///
/// This function creates a request to get the customization scope and resources configured for
/// desktop and mobile devices. Resources can be URLs or uploaded files.
///
/// **Required Permissions:** This API requires app management permissions. Use username/password
/// authentication; API tokens cannot be used.
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
/// let response = kintone::v1::app::settings::get_customization(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Customization scope: {:?}", response.scope);
/// println!("Desktop resources: {:?}", response.desktop);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-customization/>
pub fn get_customization(app_id: u64) -> GetCustomizationRequest {
    GetCustomizationRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/customize.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetCustomizationRequest {
    builder: RequestBuilder,
}

impl GetCustomizationRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sends the request to retrieve an app's JavaScript and CSS customization settings.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetCustomizationResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions. Use username/password authentication; API tokens
    /// cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<GetCustomizationResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCustomizationResponse {
    pub scope: CustomizationScope,
    pub desktop: Customization,
    pub mobile: Customization,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateCustomizationResponse = RevisionResponse;

/// Updates JavaScript and CSS customization in the preview environment.
///
/// This function creates a request to update the customization scope and desktop or mobile
/// resources. Use empty resource arrays to remove existing JavaScript or CSS files.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires kintone system administrator and app management
/// permissions. Use username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `scope` - Sets which users receive the JavaScript and CSS customizations
/// * `desktop` - Sets the JavaScript and CSS resources for desktop devices
/// * `mobile` - Sets the JavaScript and CSS resources for mobile devices
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{Customization, CustomizationResource, CustomizationScope};
///
/// let desktop = Customization {
///     js: Some(vec![CustomizationResource::Url {
///         url: "https://example.com/customize.js".to_owned(),
///     }]),
///     css: Some(vec![]),
/// };
/// let response = kintone::v1::app::settings::update_customization(123)
///     .scope(CustomizationScope::All)
///     .desktop(desktop)
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated customization, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-customization/>
pub fn update_customization(app_id: u64) -> UpdateCustomizationRequest {
    UpdateCustomizationRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/customize.json"),
        body: UpdateCustomizationRequestBody {
            app: app_id,
            scope: None,
            desktop: None,
            mobile: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateCustomizationRequest {
    builder: RequestBuilder,
    body: UpdateCustomizationRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateCustomizationRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<CustomizationScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desktop: Option<Customization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<Customization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateCustomizationRequest {
    /// Sets which users receive the JavaScript and CSS customizations.
    pub fn scope(mut self, scope: CustomizationScope) -> Self {
        self.body.scope = Some(scope);
        self
    }

    /// Sets the JavaScript and CSS resources for desktop devices.
    pub fn desktop(mut self, desktop: Customization) -> Self {
        self.body.desktop = Some(desktop);
        self
    }

    /// Sets the JavaScript and CSS resources for mobile devices.
    pub fn mobile(mut self, mobile: Customization) -> Self {
        self.body.mobile = Some(mobile);
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

    /// Sends the request to update JavaScript and CSS customization in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateCustomizationResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator and app management permissions. Use
    /// username/password authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateCustomizationResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves an app's general notification settings.
///
/// This function creates a request to get notification recipients and enabled events, including
/// record changes, comments and process status changes.
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
/// let response = kintone::v1::app::settings::get_general_notification_settings(123)
///     .send(&client)?;
/// println!("Notify commenters: {}", response.notify_to_commenter);
/// println!("Notifications: {:?}", response.notifications);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-general-notification-settings/>
pub fn get_general_notification_settings(app_id: u64) -> GetGeneralNotificationSettingsRequest {
    GetGeneralNotificationSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/notifications/general.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetGeneralNotificationSettingsRequest {
    builder: RequestBuilder,
}

impl GetGeneralNotificationSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sends the request to retrieve an app's general notification settings.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetGeneralNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<GetGeneralNotificationSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGeneralNotificationSettingsResponse {
    pub notifications: Vec<GeneralNotification>,
    pub notify_to_commenter: bool,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateGeneralNotificationSettingsResponse = RevisionResponse;

/// Updates general notification settings in the preview environment.
///
/// This function creates a request to configure event notifications and whether users who commented
/// on a record receive notifications. Supplied notifications replace the existing notification
/// list.
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
/// * `notifications` - Sets the complete notification list; an empty list removes the notifications
/// * `notify_to_commenter` - Sets whether users who commented on a record receive notifications
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{GeneralNotification, SettingsEntity, SettingsEntityType};
///
/// let notification = GeneralNotification {
///     entity: SettingsEntity {
///         entity_type: SettingsEntityType::User,
///         code: Some("user1".to_owned()),
///     },
///     record_added: Some(true),
///     ..Default::default()
/// };
/// let response = kintone::v1::app::settings::update_general_notification_settings(123)
///     .notifications([notification])
///     .notify_to_commenter(true)
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated notifications, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-general-notification-settings/>
pub fn update_general_notification_settings(
    app_id: u64,
) -> UpdateGeneralNotificationSettingsRequest {
    UpdateGeneralNotificationSettingsRequest {
        builder: RequestBuilder::new(
            http::Method::PUT,
            "/v1/preview/app/notifications/general.json",
        ),
        body: UpdateGeneralNotificationSettingsRequestBody {
            app: app_id,
            notifications: None,
            notify_to_commenter: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateGeneralNotificationSettingsRequest {
    builder: RequestBuilder,
    body: UpdateGeneralNotificationSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateGeneralNotificationSettingsRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications: Option<Vec<GeneralNotification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_to_commenter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateGeneralNotificationSettingsRequest {
    /// Sets the complete notification list; an empty list removes the notifications.
    pub fn notifications(mut self, values: impl IntoIterator<Item = GeneralNotification>) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
        self
    }

    /// Sets whether users who commented on a record receive notifications.
    pub fn notify_to_commenter(mut self, notify_to_commenter: bool) -> Self {
        self.body.notify_to_commenter = Some(notify_to_commenter);
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

    /// Sends the request to update general notification settings in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateGeneralNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateGeneralNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves an app's per-record notification settings.
///
/// This function creates a request to get notification conditions, titles and recipients for
/// notifications configured for individual records.
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
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_per_record_notification_settings(123)
///     .lang("en")
///     .send(&client)?;
/// println!("Notifications: {:?}", response.notifications);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-per-record-notification-settings/>
pub fn get_per_record_notification_settings(
    app_id: u64,
) -> GetPerRecordNotificationSettingsRequest {
    GetPerRecordNotificationSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/notifications/perRecord.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetPerRecordNotificationSettingsRequest {
    builder: RequestBuilder,
}

impl GetPerRecordNotificationSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve an app's per-record notification settings.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetPerRecordNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<GetPerRecordNotificationSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPerRecordNotificationSettingsResponse {
    pub notifications: Vec<PerRecordNotification>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdatePerRecordNotificationSettingsResponse = RevisionResponse;

/// Updates per-record notifications in the preview environment.
///
/// This function creates a request to replace the per-record notification list. Each notification
/// contains a query condition, a title and target entities.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `notifications` - Complete notification list, supplied with `notifications()`
///
/// # Optional Parameters
///
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{EntityTarget, PerRecordNotification, SettingsEntity, SettingsEntityType};
///
/// let notification = PerRecordNotification {
///     filter_cond: Some("amount > 100000".to_owned()),
///     title: Some("Large order".to_owned()),
///     targets: vec![EntityTarget {
///         entity: SettingsEntity {
///             entity_type: SettingsEntityType::User,
///             code: Some("user1".to_owned()),
///         },
///         include_subs: None,
///     }],
/// };
/// let response = kintone::v1::app::settings::update_per_record_notification_settings(123)
///     .notifications([notification])
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated notifications, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-per-record-notification-settings/>
pub fn update_per_record_notification_settings(
    app_id: u64,
) -> UpdatePerRecordNotificationSettingsRequest {
    UpdatePerRecordNotificationSettingsRequest {
        builder: RequestBuilder::new(
            http::Method::PUT,
            "/v1/preview/app/notifications/perRecord.json",
        ),
        body: UpdatePerRecordNotificationSettingsRequestBody {
            app: app_id,
            notifications: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdatePerRecordNotificationSettingsRequest {
    builder: RequestBuilder,
    body: UpdatePerRecordNotificationSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePerRecordNotificationSettingsRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications: Option<Vec<PerRecordNotification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdatePerRecordNotificationSettingsRequest {
    /// Sets the complete notification list; an empty list removes the notifications.
    pub fn notifications(
        mut self,
        values: impl IntoIterator<Item = PerRecordNotification>,
    ) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
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

    /// Sends the request to update per-record notifications in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdatePerRecordNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdatePerRecordNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves an app's reminder notification settings.
///
/// This function creates a request to get reminder timing, conditions, recipients and the timezone
/// used to send reminders.
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
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_reminder_notification_settings(123)
///     .send(&client)?;
/// println!("Timezone: {}", response.timezone);
/// println!("Reminders: {:?}", response.notifications);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-reminder-notification-settings/>
pub fn get_reminder_notification_settings(app_id: u64) -> GetReminderNotificationSettingsRequest {
    GetReminderNotificationSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/notifications/reminder.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetReminderNotificationSettingsRequest {
    builder: RequestBuilder,
}

impl GetReminderNotificationSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve an app's reminder notification settings.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetReminderNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<GetReminderNotificationSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReminderNotificationSettingsResponse {
    pub notifications: Vec<ReminderNotification>,
    pub timezone: String,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateReminderNotificationSettingsResponse = RevisionResponse;

/// Updates reminder notifications in the preview environment.
///
/// This function creates a request to configure reminders relative to date or datetime fields.
/// Negative timing offsets send reminders before the field value.
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
/// * `notifications` - Sets the complete notification list; an empty list removes the notifications
/// * `timezone` - Sets the timezone used to send reminder notifications
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{EntityTarget, ReminderNotification, ReminderTiming, SettingsEntity, SettingsEntityType};
///
/// let reminder = ReminderNotification {
///     timing: ReminderTiming {
///         code: "due_date".to_owned(),
///         days_later: -1,
///         hours_later: None,
///         time: Some("09:00".to_owned()),
///     },
///     title: Some("Due tomorrow".to_owned()),
///     targets: vec![EntityTarget {
///         entity: SettingsEntity {
///             entity_type: SettingsEntityType::User,
///             code: Some("user1".to_owned()),
///         },
///         include_subs: None,
///     }],
///     ..Default::default()
/// };
/// let response = kintone::v1::app::settings::update_reminder_notification_settings(123)
///     .notifications([reminder])
///     .timezone("Asia/Tokyo")
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Updated reminders, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-reminder-notification-settings/>
pub fn update_reminder_notification_settings(
    app_id: u64,
) -> UpdateReminderNotificationSettingsRequest {
    UpdateReminderNotificationSettingsRequest {
        builder: RequestBuilder::new(
            http::Method::PUT,
            "/v1/preview/app/notifications/reminder.json",
        ),
        body: UpdateReminderNotificationSettingsRequestBody {
            app: app_id,
            notifications: None,
            timezone: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateReminderNotificationSettingsRequest {
    builder: RequestBuilder,
    body: UpdateReminderNotificationSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReminderNotificationSettingsRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications: Option<Vec<ReminderNotification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateReminderNotificationSettingsRequest {
    /// Sets the complete notification list; an empty list removes the notifications.
    pub fn notifications(mut self, values: impl IntoIterator<Item = ReminderNotification>) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
        self
    }

    /// Sets the timezone used to send reminder notifications.
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.body.timezone = Some(timezone.into());
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

    /// Sends the request to update reminder notifications in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateReminderNotificationSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateReminderNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the app permission settings of a Kintone app.
///
/// This function creates a request to get the operations allowed for each user, group, organization
/// or app creator. The live environment is used by default.
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
/// let response = kintone::v1::app::settings::get_app_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Permission settings: {:?}", response.rights);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-app-permissions/>
pub fn get_app_permissions(app_id: u64) -> GetAppPermissionsRequest {
    GetAppPermissionsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/acl.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetAppPermissionsRequest {
    builder: RequestBuilder,
}

impl GetAppPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sends the request to retrieve the app permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetAppPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppPermissionsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppPermissionsResponse {
    pub rights: Vec<AppRight>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateAppPermissionsResponse = RevisionResponse;

/// Updates the app permission settings of a Kintone app.
///
/// This function creates a request to replace the app access rights. The live environment is
/// updated by default; use `preview(true)` to update settings before deployment.
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `rights` - Complete app access rights, supplied with `rights()`
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{AppRight, SettingsEntity, SettingsEntityType};
///
/// let current = kintone::v1::app::settings::get_app_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// let right = AppRight {
///     entity: SettingsEntity {
///         entity_type: SettingsEntityType::User,
///         code: Some("user1".to_owned()),
///     },
///     record_viewable: Some(true),
///     ..Default::default()
/// };
/// let mut rights = current.rights;
/// rights.push(right);
/// let response = kintone::v1::app::settings::update_app_permissions(123)
///     .preview(true)
///     .rights(rights)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated permissions, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-app-permissions/>
pub fn update_app_permissions(app_id: u64) -> UpdateAppPermissionsRequest {
    UpdateAppPermissionsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/app/acl.json"),
        body: UpdateAppPermissionsRequestBody {
            app: app_id,
            rights: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateAppPermissionsRequest {
    builder: RequestBuilder,
    body: UpdateAppPermissionsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateAppPermissionsRequestBody {
    app: u64,
    rights: Vec<AppRight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateAppPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the complete access rights to retain.
    pub fn rights(mut self, values: impl IntoIterator<Item = AppRight>) -> Self {
        self.body.rights = values.into_iter().collect();
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

    /// Sends the request to update the app permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateAppPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateAppPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the record permission settings of a Kintone app.
///
/// This function creates a request to get query conditions and the operations allowed for each
/// target entity. The live environment is used by default.
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
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_record_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Permission settings: {:?}", response.rights);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-record-permissions/>
pub fn get_record_permissions(app_id: u64) -> GetRecordPermissionsRequest {
    GetRecordPermissionsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/record/acl.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetRecordPermissionsRequest {
    builder: RequestBuilder,
}

impl GetRecordPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve the record permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetRecordPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetRecordPermissionsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordPermissionsResponse {
    pub rights: Vec<RecordRight>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateRecordPermissionsResponse = RevisionResponse;

/// Updates the record permission settings of a Kintone app.
///
/// This function creates a request to replace record access rules and their target entities. Rules
/// are evaluated in the specified order. The live environment is updated by default.
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `rights` - Complete record access rules, supplied with `rights()`
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{RecordRight, RecordRightEntity, SettingsEntity, SettingsEntityType};
///
/// let current = kintone::v1::app::settings::get_record_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// let mut rights = current.rights;
/// rights.push(RecordRight {
///     filter_cond: "amount > 100000".to_owned(),
///     entities: vec![RecordRightEntity {
///         entity: SettingsEntity {
///             entity_type: SettingsEntityType::User,
///             code: Some("user1".to_owned()),
///         },
///         viewable: Some(true),
///         ..Default::default()
///     }],
/// });
/// let response = kintone::v1::app::settings::update_record_permissions(123)
///     .preview(true)
///     .rights(rights)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated permissions, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-record-permissions/>
pub fn update_record_permissions(app_id: u64) -> UpdateRecordPermissionsRequest {
    UpdateRecordPermissionsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/record/acl.json"),
        body: UpdateRecordPermissionsRequestBody {
            app: app_id,
            rights: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateRecordPermissionsRequest {
    builder: RequestBuilder,
    body: UpdateRecordPermissionsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateRecordPermissionsRequestBody {
    app: u64,
    rights: Vec<RecordRight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateRecordPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the complete access rights to retain.
    pub fn rights(mut self, values: impl IntoIterator<Item = RecordRight>) -> Self {
        self.body.rights = values.into_iter().collect();
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

    /// Sends the request to update the record permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateRecordPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateRecordPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the field permission settings of a Kintone app.
///
/// This function creates a request to get viewing and editing access for each field and target
/// entity. The live environment is used by default.
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
/// let response = kintone::v1::app::settings::get_field_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// println!("Permission settings: {:?}", response.rights);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-field-permissions/>
pub fn get_field_permissions(app_id: u64) -> GetFieldPermissionsRequest {
    GetFieldPermissionsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/field/acl.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetFieldPermissionsRequest {
    builder: RequestBuilder,
}

impl GetFieldPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sends the request to retrieve the field permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetFieldPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetFieldPermissionsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFieldPermissionsResponse {
    pub rights: Vec<FieldRight>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateFieldPermissionsResponse = RevisionResponse;

/// Updates the field permission settings of a Kintone app.
///
/// This function creates a request to replace field access rules. Each rule specifies a field code
/// and the accessibility of its target entities. The live environment is updated by default.
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `rights` - Complete field access rules, supplied with `rights()`
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::settings::{Accessibility, FieldRight, FieldRightEntity, SettingsEntity, SettingsEntityType};
///
/// let current = kintone::v1::app::settings::get_field_permissions(123)
///     .preview(true)
///     .send(&client)?;
/// let mut rights = current.rights;
/// rights.push(FieldRight {
///     code: "customer_name".to_owned(),
///     entities: vec![FieldRightEntity {
///         accessibility: Accessibility::Read,
///         entity: SettingsEntity {
///             entity_type: SettingsEntityType::User,
///             code: Some("user1".to_owned()),
///         },
///         include_subs: None,
///     }],
/// });
/// let response = kintone::v1::app::settings::update_field_permissions(123)
///     .preview(true)
///     .rights(rights)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated permissions, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-field-permissions/>
pub fn update_field_permissions(app_id: u64) -> UpdateFieldPermissionsRequest {
    UpdateFieldPermissionsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/field/acl.json"),
        body: UpdateFieldPermissionsRequestBody {
            app: app_id,
            rights: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateFieldPermissionsRequest {
    builder: RequestBuilder,
    body: UpdateFieldPermissionsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateFieldPermissionsRequestBody {
    app: u64,
    rights: Vec<FieldRight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateFieldPermissionsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the complete access rights to retain.
    pub fn rights(mut self, values: impl IntoIterator<Item = FieldRight>) -> Self {
        self.body.rights = values.into_iter().collect();
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

    /// Sends the request to update the field permission settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateFieldPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateFieldPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the action settings of a Kintone app.
///
/// This function creates a request to get action names, destination apps, field mappings and
/// availability conditions. The response maps action names to their settings.
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
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_action_settings(123)
///     .preview(true)
///     .send(&client)?;
/// for (name, action) in response.actions {
///     println!("{}: {:?}", name, action.dest_app);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-action-settings/>
pub fn get_action_settings(app_id: u64) -> GetActionSettingsRequest {
    GetActionSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/actions.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetActionSettingsRequest {
    builder: RequestBuilder,
}

impl GetActionSettingsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve the action settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetActionSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetActionSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActionSettingsResponse {
    pub actions: HashMap<String, AppAction>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateActionSettingsResponse = RevisionResponse;

/// Updates an app's action settings in the preview environment.
///
/// This function creates a request to add, update or remove app actions. Include all existing
/// actions to retain; actions omitted from the request are deleted.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `actions` - Complete set of actions to retain, supplied with `actions()` or `action()`
///
/// # Optional Parameters
///
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::field::RelatedApp;
/// use kintone::model::app::settings::{ActionMapping, ActionSourceType, AppAction};
///
/// let current = kintone::v1::app::settings::get_action_settings(123)
///     .preview(true)
///     .send(&client)?;
/// let action = AppAction {
///     index: current.actions.len() as u64,
///     dest_app: Some(RelatedApp { app: Some(124), code: None }),
///     mappings: Some(vec![ActionMapping {
///         src_type: ActionSourceType::Field,
///         src_field: Some("customer_name".to_owned()),
///         dest_field: "customer_name".to_owned(),
///     }]),
///     ..Default::default()
/// };
/// let response = kintone::v1::app::settings::update_action_settings(123)
///     .actions(current.actions)
///     .action("Copy customer", action)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated actions, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-action-settings/>
pub fn update_action_settings(app_id: u64) -> UpdateActionSettingsRequest {
    UpdateActionSettingsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/actions.json"),
        body: UpdateActionSettingsRequestBody {
            app: app_id,
            actions: HashMap::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateActionSettingsRequest {
    builder: RequestBuilder,
    body: UpdateActionSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateActionSettingsRequestBody {
    app: u64,
    actions: HashMap<String, AppAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateActionSettingsRequest {
    /// Sets the complete collection of app action settings to retain.
    pub fn actions(mut self, values: impl IntoIterator<Item = (String, AppAction)>) -> Self {
        self.body.actions = values.into_iter().collect();
        self
    }

    /// Adds or replaces the settings for a single app action.
    pub fn action(mut self, name: impl Into<String>, value: AppAction) -> Self {
        self.body.actions.insert(name.into(), value);
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

    /// Sends the request to update an app's action settings in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateActionSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateActionSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the plugins configured for a Kintone app.
///
/// This function creates a request to get plugin IDs, localized names and whether each plugin is
/// enabled for the app.
///
/// **Required Permissions:** This API requires app management permissions. Use username/password
/// authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `preview` - Selects preview settings when true, or live settings when false (default: false)
/// * `lang` - Sets the language used for localized names
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::get_app_plugins(123)
///     .lang("en")
///     .send(&client)?;
/// for plugin in response.plugins {
///     println!("{}: enabled = {}", plugin.name, plugin.enabled);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-app-plugins/>
pub fn get_app_plugins(app_id: u64) -> GetAppPluginsRequest {
    GetAppPluginsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/plugins.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetAppPluginsRequest {
    builder: RequestBuilder,
}

impl GetAppPluginsRequest {
    /// Selects whether to use the preview environment.
    ///
    /// Use `true` for preview settings or `false` for live settings.
    /// The live environment is selected by default.
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    /// Sets the language used for localized names.
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    /// Sends the request to retrieve the plugins configured for a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetAppPluginsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions. Use username/password authentication; API tokens
    /// cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<GetAppPluginsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAppPluginsResponse {
    pub plugins: Vec<AppPlugin>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type AddAppPluginsResponse = RevisionResponse;

/// Adds plugins to an app in the preview environment.
///
/// This function creates a request to add plugins that have already been installed in kintone.
/// Supply the installed plugin IDs with the `ids()` method.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires app management permissions. Use username/password
/// authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `ids` - Installed plugin IDs to add, supplied with `ids()`
///
/// # Optional Parameters
///
/// * `revision` - Expected settings revision; `None` or omission skips revision validation
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::app::settings::add_app_plugins(123)
///     .ids(["installed-plugin-id".to_owned()])
///     .revision(Some(5))
///     .send(&client)?;
/// println!("Added plugins, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/add-app-plugins/>
pub fn add_app_plugins(app_id: u64) -> AddAppPluginsRequest {
    AddAppPluginsRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/preview/app/plugins.json"),
        body: AddAppPluginsRequestBody {
            app: app_id,
            ids: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct AddAppPluginsRequest {
    builder: RequestBuilder,
    body: AddAppPluginsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddAppPluginsRequestBody {
    app: u64,
    ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl AddAppPluginsRequest {
    /// Sets the installed plugin IDs to add to the app.
    pub fn ids(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.ids = values.into_iter().collect();
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

    /// Sends the request to add plugins to an app in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`AddAppPluginsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions. Use username/password authentication; API tokens
    /// cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddAppPluginsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type MoveAppResponse = EmptyResponse;

/// Changes the space that contains a Kintone app.
///
/// This function creates a request to move an app to the specified space. Omitting `space()`
/// removes the app from its current space.
///
/// **Required Permissions:** The user must satisfy the source and destination space permissions
/// documented in the API reference. Use username/password authentication; API tokens cannot be
/// used.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
///
/// # Optional Parameters
///
/// * `space` - Sets the destination space ID; omitting this call removes the app from its space
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::app::settings::move_app(123)
///     .space(10)
///     .send(&client)?;
/// println!("Moved the app to space 10");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/move-app/>
pub fn move_app(app_id: u64) -> MoveAppRequest {
    MoveAppRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/app/move.json"),
        body: MoveAppRequestBody {
            app: app_id,
            space: None,
        },
    }
}

#[must_use]
pub struct MoveAppRequest {
    builder: RequestBuilder,
    body: MoveAppRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MoveAppRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    space: Option<u64>,
}

impl MoveAppRequest {
    /// Sets the destination space ID; omitting this call removes the app from its space.
    pub fn space(mut self, space: u64) -> Self {
        self.body.space = Some(space);
        self
    }

    /// Sends the request to change the space that contains a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`MoveAppResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// The user must satisfy the source and destination space permissions documented in the API
    /// reference. Use username/password authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<MoveAppResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
