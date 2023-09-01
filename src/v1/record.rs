use reqwest::Method;
use serde::Deserialize;

use crate::{
    client::{KintoneClient, Request},
    models::Record,
};

#[must_use]
pub fn get_record(app: u64, id: u64) -> GetRecordRequest {
    GetRecordRequest { app, id }
}

#[derive(Clone)]
pub struct GetRecordRequest {
    app: u64,
    id: u64,
}

#[derive(Deserialize)]
struct GetRecordResponse {
    record: Record,
}

impl GetRecordRequest {
    pub fn call(self, client: &KintoneClient) -> crate::Result<Record> {
        let app_str = self.app.to_string();
        let id_str = self.id.to_string();
        let req = Request::builder(Method::GET, "/k/v1/record.json")
            .query_param("app", &app_str)
            .query_param("id", &id_str)
            .build();
        let resp: GetRecordResponse = client.call(req)?;
        Ok(resp.record)
    }
}

#[must_use]
pub fn get_records<'a>(app: u64) -> GetRecordsRequest {
    GetRecordsRequest {
        app,
        ..Default::default()
    }
}

#[derive(Clone, Default)]
pub struct GetRecordsRequest {
    app: u64,
    fields: Option<Vec<String>>,
    query: Option<String>,
    total_count: Option<bool>,
}

#[derive(Deserialize)]
struct GetRecordsResponse {
    records: Vec<Record>,
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

    pub fn call(self, client: &KintoneClient) -> crate::Result<Vec<Record>> {
        let app_str = self.app.to_string();
        let mut req =
            Request::builder(Method::GET, "/k/v1/records.json").query_param("app", &app_str);
        let fields = self.fields.unwrap_or(vec![]);
        for field in &fields {
            req = req.query_param("fields[]", &field);
        }
        let resp: GetRecordsResponse = client.call(req.build())?;
        Ok(resp.records)
    }
}
