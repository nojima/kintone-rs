use serde::Deserialize;

use crate::ApiResult;
use crate::client::{KintoneClient, RequestBuilder};
use crate::internal::serde_helper::stringified;
use crate::models::Record;

pub fn get_record(client: &KintoneClient, app: u64, id: u64) -> GetRecordRequest {
    let builder = client
        .request(http::Method::GET, "/k/v1/record.json")
        .query("app", app)
        .query("id", id);
    GetRecordRequest { builder }
}

#[must_use]
pub struct GetRecordRequest {
    builder: RequestBuilder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordResponse {
    pub record: Record,
}

impl GetRecordRequest {
    pub fn send(self) -> ApiResult<GetRecordResponse> {
        self.builder.call()
    }
}

pub fn get_records(client: &KintoneClient, app: u64) -> GetRecordsRequest {
    let builder = client
        .request(http::Method::GET, "/k/v1/records.json")
        .query("app", app);
    GetRecordsRequest { builder }
}

#[must_use]
pub struct GetRecordsRequest {
    builder: RequestBuilder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordsResponse {
    pub records: Vec<Record>,

    #[serde(with = "stringified")]
    pub total_count: usize,
}

impl GetRecordsRequest {
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.builder = self.builder.query_array("fields", fields);
        self
    }

    pub fn query(mut self, query: &str) -> Self {
        self.builder = self.builder.query("query", query);
        self
    }

    pub fn total_count(mut self, total_count: bool) -> Self {
        self.builder = self.builder.query("totalCount", total_count);
        self
    }

    pub fn send(self) -> ApiResult<GetRecordsResponse> {
        self.builder.call()
    }
}
