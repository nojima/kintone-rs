//! # Kintone App Report API
//!
//! This module provides functions for retrieving and updating graphs, aggregations and periodic
//! reports in Kintone apps.
//!
//! ## Available Operations
//!
//! - [`get_graph_settings`] - Retrieve graph settings
//! - [`update_graph_settings`] - Update graphs in the preview environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! let response = kintone::v1::app::report::get_graph_settings(123)
//!     .preview(true)
//!     .send(&client)?;
//! println!("Graphs: {:?}", response.reports);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Graph updates are made in the preview environment. Use [`crate::v1::app::settings::deploy_app`]
//! to apply them to production.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::app::report::Graph;

/// Retrieves the graph settings of a Kintone app.
///
/// This function creates a request to get graph types, grouping, aggregations and periodic report
/// settings. The response maps graph names to their settings.
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
/// let response = kintone::v1::app::report::get_graph_settings(123)
///     .preview(true)
///     .send(&client)?;
/// for (name, graph) in response.reports {
///     println!("{}: {:?}", name, graph.chart_type);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/report/get-graph-settings/>
pub fn get_graph_settings(app_id: u64) -> GetGraphSettingsRequest {
    GetGraphSettingsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/reports.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetGraphSettingsRequest {
    builder: RequestBuilder,
}

impl GetGraphSettingsRequest {
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

    /// Sends the request to retrieve the graph settings of a Kintone app.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetGraphSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// Viewing live settings requires record viewing or record creation permissions. Viewing preview
    /// settings requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<GetGraphSettingsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGraphSettingsResponse {
    pub reports: HashMap<String, Graph>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGraphSettingsResponse {
    pub reports: HashMap<String, SettingId>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingId {
    #[serde(with = "stringified")]
    pub id: u64,
}

/// Updates an app's graph settings in the preview environment.
///
/// This function creates a request to add, update or remove graphs and configure periodic reports.
/// Include all existing graphs to retain; omitted graphs are deleted.
///
/// **Important**: Changes are made in the preview environment. To apply them to the
/// production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// **Required Permissions:** This API requires app management permissions.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `reports` - Complete set of graphs to retain, supplied with `reports()` or `report()`
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
/// use kintone::model::app::report::{AggregationType, ChartType, Graph, GraphAggregation};
///
/// let current = kintone::v1::app::report::get_graph_settings(123)
///     .preview(true)
///     .send(&client)?;
/// let graph = Graph {
///     chart_type: ChartType::Bar,
///     index: current.reports.len() as u64,
///     aggregations: Some(vec![GraphAggregation {
///         aggregation_type: AggregationType::Count,
///         code: None,
///     }]),
///     ..Default::default()
/// };
/// let response = kintone::v1::app::report::update_graph_settings(123)
///     .reports(current.reports)
///     .report("Record count", graph)
///     .revision(Some(current.revision))
///     .send(&client)?;
/// println!("Updated graphs, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/report/update-graph-settings/>
pub fn update_graph_settings(app_id: u64) -> UpdateGraphSettingsRequest {
    UpdateGraphSettingsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/reports.json"),
        body: UpdateGraphSettingsRequestBody {
            app: app_id,
            reports: HashMap::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateGraphSettingsRequest {
    builder: RequestBuilder,
    body: UpdateGraphSettingsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateGraphSettingsRequestBody {
    app: u64,
    reports: HashMap<String, Graph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateGraphSettingsRequest {
    /// Sets the complete collection of graph settings to retain.
    pub fn reports(mut self, values: impl IntoIterator<Item = (String, Graph)>) -> Self {
        self.body.reports = values.into_iter().collect();
        self
    }

    /// Adds or replaces the settings for a single graph.
    pub fn report(mut self, name: impl Into<String>, value: Graph) -> Self {
        self.body.reports.insert(name.into(), value);
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

    /// Sends the request to update an app's graph settings in the preview environment.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateGraphSettingsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateGraphSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
