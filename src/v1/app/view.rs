//! App views APIs.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::app::view::View;

/// Get views.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/view/get-views/>
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
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

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

/// Update views.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/view/update-views/>
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
    pub fn views(mut self, values: impl IntoIterator<Item = (String, View)>) -> Self {
        self.body.views = values.into_iter().collect();
        self
    }

    pub fn view(mut self, name: impl Into<String>, value: View) -> Self {
        self.body.views.insert(name.into(), value);
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateViewsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
