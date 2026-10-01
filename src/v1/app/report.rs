//! App graphs APIs.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::app::report::Graph;

/// Get graph settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/report/get-graph-settings/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update graph settings.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/report/update-graph-settings/>
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
    pub fn reports(mut self, values: impl IntoIterator<Item = (String, Graph)>) -> Self {
        self.body.reports = values.into_iter().collect();
        self
    }

    pub fn report(mut self, name: impl Into<String>, value: Graph) -> Self {
        self.body.reports.insert(name.into(), value);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateGraphSettingsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
