use reqwest::Method;
use serde::Deserialize;

use crate::client::{KintoneClient, Request, RequestBuilder};
use crate::internal::serde_helper::as_str;
use crate::models::Record;

#[must_use]
pub fn get_record(app: u64, id: u64) -> GetRecordRequest {
    GetRecordRequest { app, id }
}

#[must_use]
#[derive(Clone)]
pub struct GetRecordRequest {
    app: u64,
    id: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordResponse {
    pub record: Record,
}

impl GetRecordRequest {
    pub fn send(self, client: &KintoneClient) -> crate::Result<GetRecordResponse> {
        let app_str = self.app.to_string();
        let id_str = self.id.to_string();
        let req: Request<'_, ()> = Request::builder(Method::GET, "/k/v1/record.json")
            .query_param("app", &app_str)
            .query_param("id", &id_str)
            .build();
        Ok(client.call(req)?)
    }
}

pub fn get_records<'a>(app: u64) -> GetRecordsRequest {
    GetRecordsRequest {
        app,
        ..Default::default()
    }
}

#[must_use]
#[derive(Clone, Default)]
pub struct GetRecordsRequest {
    app: u64,
    fields: Option<Vec<String>>,
    query: Option<String>,
    total_count: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordsResponse {
    pub records: Vec<Record>,

    #[serde(with = "as_str")]
    pub total_count: usize,
}

impl GetRecordsRequest {
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn total_count(mut self, total_count: bool) -> Self {
        self.total_count = Some(total_count);
        self
    }

    pub fn send(self, client: &KintoneClient) -> crate::Result<GetRecordsResponse> {
        let app_str = self.app.to_string();
        let mut req: RequestBuilder<'_, ()> =
            Request::builder(Method::GET, "/k/v1/records.json").query_param("app", &app_str);
        let fields = self.fields.unwrap_or(vec![]);
        for field in &fields {
            req = req.query_param("fields[]", &field);
        }
        Ok(client.call(req.build())?)
    }
}
