//! # Kintone App View API
//!
//! This module provides functions for retrieving and updating list, calendar and custom views in
//! Kintone apps.
//!
//! ## Available Operations
//!
//! - [`get_views`] - Retrieve view settings
//! - [`update_views`] - Update views in the preview environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! let response = kintone::v1::app::view::get_views(123)
//!     .preview(true)
//!     .send(&client)?;
//! println!("Views: {:?}", response.views);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: View updates are made in the preview environment. Use [`crate::v1::app::settings::deploy_app`]
//! to apply them to production.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::app::view::View;

/// Retrieves the views configured for a Kintone app.
///
/// This function creates a request to get list, calendar and custom view settings. The response
/// maps view names to their settings and includes the app settings revision.
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
/// let response = kintone::v1::app::view::get_views(123)
///     .lang("en")
///     .preview(true)
///     .send(&client)?;
/// for (name, view) in response.views {
///     println!("{}: {:?}", name, view.view_type);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/view/get-views/>
pub fn get_views(app_id: u64) -> GetViewsRequest {
    GetViewsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/views.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetViewsRequest {
    builder: RequestBuilder,
}

impl GetViewsRequest {
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

    /// Sends the request to retrieve the views configured for a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetViewsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// Viewing live settings requires record viewing or record creation permissions. Viewing preview
    /// settings requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetViewsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetViewsResponse {
    pub views: HashMap<String, View>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateViewsResponse {
    pub views: HashMap<String, SettingId>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingId {
    #[serde(with = "stringified")]
    pub id: u64,
}

/// Updates an app's views in the preview environment.
///
/// This function creates a request to add, update or remove views. Include all existing views to
/// retain; views omitted from the request are deleted.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** List and calendar views require app management permissions. Custom
/// views require kintone system administrator permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `views` - Complete set of views to retain, supplied with `views()` or `view()`
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
/// use kintone::model::app::view::{View, ViewType};
///
/// let current = kintone::v1::app::view::get_views(123).preview(true).send(&client)?;
/// let view = View {
///     view_type: ViewType::List,
///     index: current.views.len() as u64,
///     fields: Some(vec!["customer_name".to_owned(), "amount".to_owned()]),
///     filter_cond: Some("amount > 0".to_owned()),
///     ..Default::default()
/// };
/// let response = kintone::v1::app::view::update_views(123)
///     .views(current.views)
///     .view("Active customers", view)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated views, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/view/update-views/>
pub fn update_views(app_id: u64) -> UpdateViewsRequest {
    UpdateViewsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/views.json"),
        body: UpdateViewsRequestBody {
            app: app_id,
            views: HashMap::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateViewsRequest {
    builder: RequestBuilder,
    body: UpdateViewsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateViewsRequestBody {
    app: u64,
    views: HashMap<String, View>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateViewsRequest {
    /// Sets the complete collection of view settings to retain.
    pub fn views(mut self, values: impl IntoIterator<Item = (String, View)>) -> Self {
        self.body.views = values.into_iter().collect();
        self
    }

    /// Adds or replaces the settings for a single view.
    pub fn view(mut self, name: impl Into<String>, value: View) -> Self {
        self.body.views.insert(name.into(), value);
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

    /// Sends the request to update an app's views in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateViewsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// List and calendar views require app management permissions. Custom views require kintone system
    /// administrator permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateViewsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
