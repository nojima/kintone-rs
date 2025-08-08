//! # Kintone App Settings API
//!
//! This module provides functions for managing app settings in Kintone's preview environment
//! and deploying those settings to the production environment.
//!
//! ## Available Operations
//!
//! ### Settings Deployment
//! - [`deploy_app_settings`] - Deploy app settings from preview to production environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```rust
//! # use kintone::client::{Auth, KintoneClient};
//! # use kintone::v1::app::settings;
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_string(), "pass".to_string()));
//! settings::deploy_app_settings()
//!     .app(123, Some(45)) // app ID with optional revision
//!     .app(124, None)     // app ID without revision check
//!     .send(&client)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: App settings APIs require app management permissions.

use serde::Serialize;

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};

/// Deploys app settings from the preview environment to the production environment.
///
/// This function creates a request to deploy app settings that have been configured
/// in the preview environment to the production environment. This is equivalent to
/// clicking the "Deploy App" or "Cancel Changes" button in the app settings interface.
///
/// **Important Features:**
/// - This is an asynchronous API. Use the deploy status API to check completion.
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
/// let response = deploy_app_settings()
///     .app(123, Some(45))  // Deploy app 123 with revision check
///     .app(124, None)      // Deploy app 124 without revision check
///     .revert(false)       // Deploy changes (default)
///     .send(&client)?;
///
/// // Cancel changes instead of deploying
/// let response = deploy_app_settings()
///     .app(123, None)
///     .revert(true)        // Cancel changes
///     .send(&client)?;
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/settings/deploy-app-settings/>
pub fn deploy_app_settings() -> DeployAppSettingsRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/preview/app/deploy.json");
    DeployAppSettingsRequest {
        builder,
        body: DeployAppSettingsRequestBody {
            apps: Vec::new(),
            revert: None,
        },
    }
}

#[must_use]
pub struct DeployAppSettingsRequest {
    builder: RequestBuilder,
    body: DeployAppSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeployAppSettingsRequestBody {
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

impl DeployAppSettingsRequest {
    /// Adds an app to be deployed.
    ///
    /// # Arguments
    /// * `app_id` - The ID of the app to deploy
    /// * `revision` - Optional revision number for validation. If provided and doesn't match
    ///   the actual revision, an error will be returned. Use `None` to skip validation.
    ///
    /// # Example
    /// ```rust
    /// let request = deploy_app_settings()
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
    /// let request = deploy_app_settings()
    ///     .app(123, None)
    ///     .revert(true);
    ///
    /// // Deploy changes (default behavior)
    /// let request = deploy_app_settings()
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
    /// **Note**: This is an asynchronous operation. Use the deploy status API to check
    /// if the deployment has completed successfully.
    ///
    /// # Authentication
    /// Requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<(), ApiError> {
        self.builder.send(client, self.body)
    }
}
