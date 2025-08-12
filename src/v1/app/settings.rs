//! # Kintone App Settings API
//!
//! This module provides functions for managing app settings in Kintone's preview environment
//! and deploying those settings to the production environment.
//!
//! ## Available Operations
//!
//! ### Settings Deployment
//! - [`deploy_app`] - Deploy app settings from preview to production environment
//! - [`get_app_deploy_status`] - Check the deployment status of app settings
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```rust
//! # use kintone::client::{Auth, KintoneClient};
//! # use kintone::v1::app::settings;
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_string(), "pass".to_string()));
//! // Deploy apps
//! settings::deploy_app()
//!     .app(123, Some(45)) // app ID with optional revision
//!     .app(124, None)     // app ID without revision check
//!     .send(&client)?;
//!
//! // Check deployment status
//! let status = settings::get_app_deploy_status()
//!     .app(123)
//!     .app(124)
//!     .send(&client)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: App settings APIs require app management permissions.

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};

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
/// ```rust
/// // Deploy multiple apps
/// let response = deploy_app()
///     .app(123, Some(45))  // Deploy app 123 with revision check
///     .app(124, None)      // Deploy app 124 without revision check
///     .revert(false)       // Deploy changes (default)
///     .send(&client)?;
///
/// // Cancel changes instead of deploying
/// let response = deploy_app()
///     .app(123, None)
///     .revert(true)        // Cancel changes
///     .send(&client)?;
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
    /// ```rust
    /// let request = deploy_app()
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
    ///
    /// # Arguments
    /// * `revert` - `true` to cancel changes and revert the preview environment to match
    ///   the production environment, `false` to deploy changes to production (default)
    ///
    /// # Example
    /// ```rust
    /// // Cancel changes
    /// let request = deploy_app()
    ///     .app(123, None)
    ///     .revert(true);
    ///
    /// // Deploy changes (default behavior)
    /// let request = deploy_app()
    ///     .app(123, None)
    ///     .revert(false);
    /// ```
    pub fn revert(mut self, revert: bool) -> Self {
        self.body.revert = Some(revert);
        self
    }

    /// Sends the request to deploy app settings.
    ///
    /// # Returns
    /// A Result containing `()` on success, or an ApiError on failure.
    /// This API has no response body - success is indicated by the absence of an error.
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
/// ```rust
/// // Check deployment status for multiple apps
/// let status = get_app_deploy_status()
///     .app(123)
///     .app(124)
///     .app(125)
///     .send(&client)?;
///
/// for app_status in status.apps {
///     match app_status.status {
///         DeployStatus::Processing => println!("App {} is still deploying", app_status.app),
///         DeployStatus::Success => println!("App {} deployed successfully", app_status.app),
///         DeployStatus::Fail => println!("App {} deployment failed", app_status.app),
///         DeployStatus::Cancel => println!("App {} deployment was cancelled", app_status.app),
///     }
/// }
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
