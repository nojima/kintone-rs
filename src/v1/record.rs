use serde::{Deserialize, Serialize};

use crate::ApiResult;
use crate::client::{KintoneClient, RequestBuilder};
use crate::internal::serde_helper::stringified;
use crate::models::{Order, PostedRecordComment, Record, RecordComment};

// https://cybozu.dev/ja/kintone/docs/rest-api/records/get-record/
pub fn get_record(app: u64, id: u64) -> GetRecordRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/record.json")
        .query("app", app)
        .query("id", id);
    GetRecordRequest { builder }
}

#[must_use]
pub struct GetRecordRequest {
    builder: RequestBuilder,
}

impl GetRecordRequest {
    pub fn send(self, client: &KintoneClient) -> ApiResult<GetRecordResponse> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordResponse {
    pub record: Record,
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/get-records/
pub fn get_records(app: u64) -> GetRecordsRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/records.json").query("app", app);
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

    pub fn send(self, client: &KintoneClient) -> ApiResult<GetRecordsResponse> {
        self.builder.call(client)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/add-record/
pub fn add_record(app: u64) -> AddRecordRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/record.json");
    AddRecordRequest {
        builder,
        body: AddRecordRequestBody { app, record: None },
    }
}

#[must_use]
pub struct AddRecordRequest {
    builder: RequestBuilder,
    body: AddRecordRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRecordRequestBody {
    app: u64,
    record: Option<Record>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRecordResponse {
    #[serde(with = "stringified")]
    pub id: u64,
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl AddRecordRequest {
    pub fn record(mut self, record: Record) -> Self {
        self.body.record = Some(record);
        self
    }

    pub fn send(self, client: &KintoneClient) -> ApiResult<AddRecordResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/update-record/
pub fn update_record(app: u64) -> UpdateRecordRequest {
    let builder = RequestBuilder::new(http::Method::PUT, "/v1/record.json");
    UpdateRecordRequest {
        builder,
        body: UpdateRecordRequestBody {
            app,
            id: None,
            update_key: None,
            record: None,
            revision: None,
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKey {
    pub field: String,
    pub value: String,
}

#[must_use]
pub struct UpdateRecordRequest {
    builder: RequestBuilder,
    body: UpdateRecordRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordRequestBody {
    app: u64,
    id: Option<u64>,
    update_key: Option<UpdateKey>,
    record: Option<Record>,
    revision: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordResponse {
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl UpdateRecordRequest {
    pub fn id(mut self, id: u64) -> Self {
        self.body.id = Some(id);
        self
    }

    pub fn update_key(mut self, field: String, value: String) -> Self {
        self.body.update_key = Some(UpdateKey { field, value });
        self
    }

    pub fn record(mut self, record: Record) -> Self {
        self.body.record = Some(record);
        self
    }

    pub fn revision(mut self, revision: u64) -> Self {
        self.body.revision = Some(revision);
        self
    }

    pub fn send(self, client: &KintoneClient) -> ApiResult<UpdateRecordResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/get-comments/
pub fn get_comments(app: u64, record: u64) -> GetCommentsRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/record/comments.json")
        .query("app", app)
        .query("record", record);
    GetCommentsRequest { builder }
}

#[must_use]
pub struct GetCommentsRequest {
    builder: RequestBuilder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentsResponse {
    pub comments: Vec<PostedRecordComment>,
    pub older: bool,
    pub newer: bool,
}

impl GetCommentsRequest {
    pub fn order(mut self, order: Order) -> Self {
        self.builder = self.builder.query("order", order);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    pub fn send(self, client: &KintoneClient) -> ApiResult<GetCommentsResponse> {
        self.builder.call(client)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/add-comment/
pub fn add_comment(app: u64, record: u64, comment: RecordComment) -> AddCommentRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/record/comment.json");
    AddCommentRequest {
        builder,
        body: AddCommentRequestBody {
            app,
            record,
            comment,
        },
    }
}

#[must_use]
pub struct AddCommentRequest {
    builder: RequestBuilder,
    body: AddCommentRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentRequestBody {
    app: u64,
    record: u64,
    comment: RecordComment,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentResponse {
    #[serde(with = "stringified")]
    pub id: u64,
}

impl AddCommentRequest {
    pub fn send(self, client: &KintoneClient) -> ApiResult<AddCommentResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/delete-comment/
pub fn delete_comment(app: u64, record: u64, comment: u64) -> DeleteCommentRequest {
    let builder = RequestBuilder::new(http::Method::DELETE, "/v1/record/comment.json");
    DeleteCommentRequest {
        builder,
        body: DeleteCommentRequestBody {
            app,
            record,
            comment,
        },
    }
}

#[must_use]
pub struct DeleteCommentRequest {
    builder: RequestBuilder,
    body: DeleteCommentRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequestBody {
    app: u64,
    record: u64,
    comment: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCommentResponse {
    // Empty response body
}

impl DeleteCommentRequest {
    pub fn send(self, client: &KintoneClient) -> ApiResult<DeleteCommentResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/update-assignees/
pub fn update_assignees(app: u64, id: u64, assignees: Vec<String>) -> UpdateAssigneesRequest {
    let builder = RequestBuilder::new(http::Method::PUT, "/v1/record/assignees.json");
    UpdateAssigneesRequest {
        builder,
        body: UpdateAssigneesRequestBody {
            app,
            id,
            assignees,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateAssigneesRequest {
    builder: RequestBuilder,
    body: UpdateAssigneesRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssigneesRequestBody {
    app: u64,
    id: u64,
    assignees: Vec<String>,
    revision: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssigneesResponse {
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl UpdateAssigneesRequest {
    pub fn revision(mut self, revision: u64) -> Self {
        self.body.revision = Some(revision);
        self
    }

    pub fn send(self, client: &KintoneClient) -> ApiResult<UpdateAssigneesResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/records/update-status/
pub fn update_status(app: u64, id: u64, action: String) -> UpdateStatusRequest {
    let builder = RequestBuilder::new(http::Method::PUT, "/v1/record/status.json");
    UpdateStatusRequest {
        builder,
        body: UpdateStatusRequestBody {
            app,
            id,
            action,
            assignee: None,
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateStatusRequest {
    builder: RequestBuilder,
    body: UpdateStatusRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusRequestBody {
    app: u64,
    id: u64,
    action: String,
    assignee: Option<String>,
    revision: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusResponse {
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl UpdateStatusRequest {
    pub fn assignee(mut self, assignee: String) -> Self {
        self.body.assignee = Some(assignee);
        self
    }

    pub fn revision(mut self, revision: u64) -> Self {
        self.body.revision = Some(revision);
        self
    }

    pub fn send(self, client: &KintoneClient) -> ApiResult<UpdateStatusResponse> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------
