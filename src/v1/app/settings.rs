//! # Kintone App Settings API
//!
//! This module provides functions for managing app settings in Kintone's preview environment
//! and deploying those settings to the production environment.
//!
//! ## Available Operations
//!
//! Configuration getters use the live environment by default and support `.preview(true)`.
//! Updates target the preview environment, except permission updates which support
//! both environments and default to live. Unset update parameters are omitted.
//! This module also provides general settings, process management, customization,
//! notifications, permissions, app actions, plugins and moving apps between spaces.
//!
//! ### Settings Deployment
//! - [`deploy_app`] - Deploy app settings from preview to production environment
//! - [`get_app_deploy_status`] - Check the deployment status of app settings
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
//! **Note**: App settings APIs require app management permissions.

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

/// Get general settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-general-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update general settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-general-settings/>
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
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.body.icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.body.theme = Some(theme);
        self
    }

    pub fn title_field(mut self, title_field: TitleField) -> Self {
        self.body.title_field = Some(title_field);
        self
    }

    pub fn enable_thumbnails(mut self, enable_thumbnails: bool) -> Self {
        self.body.enable_thumbnails = Some(enable_thumbnails);
        self
    }

    pub fn enable_bulk_deletion(mut self, enable_bulk_deletion: bool) -> Self {
        self.body.enable_bulk_deletion = Some(enable_bulk_deletion);
        self
    }

    pub fn enable_comments(mut self, enable_comments: bool) -> Self {
        self.body.enable_comments = Some(enable_comments);
        self
    }

    pub fn enable_duplicate_record(mut self, enable_duplicate_record: bool) -> Self {
        self.body.enable_duplicate_record = Some(enable_duplicate_record);
        self
    }

    pub fn enable_inline_record_editing(mut self, enable_inline_record_editing: bool) -> Self {
        self.body.enable_inline_record_editing = Some(enable_inline_record_editing);
        self
    }

    pub fn number_precision(mut self, number_precision: NumberPrecision) -> Self {
        self.body.number_precision = Some(number_precision);
        self
    }

    pub fn first_month_of_fiscal_year(mut self, first_month_of_fiscal_year: u64) -> Self {
        self.body.first_month_of_fiscal_year = Some(first_month_of_fiscal_year);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateGeneralSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get process management settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-process-management-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update process management settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-process-management-settings/>
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
    pub fn enable(mut self, enable: bool) -> Self {
        self.body.enable = Some(enable);
        self
    }

    pub fn states(mut self, values: impl IntoIterator<Item = (String, ProcessState)>) -> Self {
        self.body.states = Some(values.into_iter().collect());
        self
    }

    pub fn state(mut self, name: impl Into<String>, value: ProcessState) -> Self {
        self.body.states.get_or_insert_with(HashMap::new).insert(name.into(), value);
        self
    }

    pub fn actions(mut self, values: impl IntoIterator<Item = ProcessAction>) -> Self {
        self.body.actions = Some(values.into_iter().collect());
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateProcessManagementSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get customization.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-customization/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

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

/// Update customization.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-customization/>
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
    pub fn scope(mut self, scope: CustomizationScope) -> Self {
        self.body.scope = Some(scope);
        self
    }

    pub fn desktop(mut self, desktop: Customization) -> Self {
        self.body.desktop = Some(desktop);
        self
    }

    pub fn mobile(mut self, mobile: Customization) -> Self {
        self.body.mobile = Some(mobile);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateCustomizationResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get general notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-general-notification-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

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

/// Update general notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-general-notification-settings/>
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
    pub fn notifications(mut self, values: impl IntoIterator<Item = GeneralNotification>) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
        self
    }

    pub fn notify_to_commenter(mut self, notify_to_commenter: bool) -> Self {
        self.body.notify_to_commenter = Some(notify_to_commenter);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateGeneralNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get per record notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-per-record-notification-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update per record notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-per-record-notification-settings/>
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
    pub fn notifications(
        mut self,
        values: impl IntoIterator<Item = PerRecordNotification>,
    ) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdatePerRecordNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get reminder notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-reminder-notification-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update reminder notification settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-reminder-notification-settings/>
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
    pub fn notifications(mut self, values: impl IntoIterator<Item = ReminderNotification>) -> Self {
        self.body.notifications = Some(values.into_iter().collect());
        self
    }

    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.body.timezone = Some(timezone.into());
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<UpdateReminderNotificationSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get app permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-app-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

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

/// Update app permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-app-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn rights(mut self, values: impl IntoIterator<Item = AppRight>) -> Self {
        self.body.rights = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateAppPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get record permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-record-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update record permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-record-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn rights(mut self, values: impl IntoIterator<Item = RecordRight>) -> Self {
        self.body.rights = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateRecordPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get field permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-field-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

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

/// Update field permissions.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-field-permissions/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn rights(mut self, values: impl IntoIterator<Item = FieldRight>) -> Self {
        self.body.rights = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateFieldPermissionsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get action settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-action-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update action settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/update-action-settings/>
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
    pub fn actions(mut self, values: impl IntoIterator<Item = (String, AppAction)>) -> Self {
        self.body.actions = values.into_iter().collect();
        self
    }

    pub fn action(mut self, name: impl Into<String>, value: AppAction) -> Self {
        self.body.actions.insert(name.into(), value);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateActionSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get app plugins.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/get-app-plugins/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Add app plugins.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/add-app-plugins/>
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
    pub fn ids(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.ids = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<AddAppPluginsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type MoveAppResponse = EmptyResponse;

/// Move app.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/move-app/>
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
    pub fn space(mut self, space: u64) -> Self {
        self.body.space = Some(space);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<MoveAppResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
