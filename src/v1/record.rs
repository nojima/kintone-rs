//! # Kintone Record API
//!
//! This module provides functions for interacting with Kintone's record-related REST API endpoints.
//! It includes operations for managing records, comments, assignees, workflow statuses, and cursor-based pagination.
//!
//! ## Available Functions
//!
//! ### Record Operations
//! - [`get_record`] - Retrieve a single record by ID
//! - [`get_records`] - Retrieve multiple records with filtering and pagination
//! - [`add_record`] - Create a new record
//! - [`update_record`] - Update an existing record
//!
//! ### Comment Operations
//! - [`get_comments`] - Retrieve comments for a record
//! - [`add_comment`] - Add a new comment to a record
//! - [`delete_comment`] - Delete a comment from a record
//!
//! ### Workflow Operations
//! - [`update_assignees`] - Update the assignees of a record
//! - [`update_status`] - Update the workflow status of a record
//!
//! ### Cursor-based Pagination
//! - [`create_cursor`] - Create a cursor for efficient pagination through large datasets
//! - [`get_records_by_cursor`] - Retrieve records using a cursor
//! - [`delete_cursor`] - Delete a cursor to free up resources

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::{
    Order,
    record::{PostedRecordComment, Record, RecordComment},
};

/// Retrieves a single record from a Kintone app by its ID.
///
/// This function creates a request to get a specific record from the specified app.
/// The record is identified by its unique ID within the app.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `id` - The ID of the record to retrieve
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::get_record(123, 456).send(&client)?;
/// println!("Record: {:?}", response.record);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/get-record/>
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
    pub fn send(self, client: &KintoneClient) -> Result<GetRecordResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordResponse {
    pub record: Record,
}

//-----------------------------------------------------------------------------

/// Retrieves multiple records from a Kintone app with optional filtering and pagination.
///
/// This function creates a request to get records from the specified app. The request
/// can be configured with query conditions, field selection, and pagination options.
///
/// # Arguments
/// * `app` - The ID of the Kintone app to retrieve records from
/// * `fields` (optional) - An array of field codes to include in the response
/// * `query` (optional) - A query string following Kintone's query syntax (e.g., "status = \"Active\" and priority > 3")
/// * `total_count` (optional) - If true, includes the total count; if false, excludes it for better performance
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::get_records(123)
///     .query("status = \"Active\"")
///     .fields(&["name", "email", "status"])
///     .send(&client)?;
/// println!("Found {} records", response.records.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/get-records/>
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

    #[serde(with = "option_stringified")]
    pub total_count: Option<usize>,
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

    pub fn send(self, client: &KintoneClient) -> Result<GetRecordsResponse, ApiError> {
        self.builder.call(client)
    }
}

//-----------------------------------------------------------------------------

/// Creates a new record in a Kintone app.
///
/// This function creates a request to add a new record to the specified app.
/// The record data can be provided using the `record()` method on the returned request.
///
/// # Arguments
/// * `app` - The ID of the Kintone app to add the record to
/// * `record` (optional) - A Record containing the field data for the new record
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::{Record, FieldValue};
/// use bigdecimal::BigDecimal;
///
/// let record = Record::from([
///     ("name", FieldValue::SingleLineText("John Doe".to_owned())),
///     ("age", FieldValue::Number(Some(30.into()))),
/// ]);
///
/// let response = kintone::v1::record::add_record(123)
///     .record(record)
///     .send(&client)?;
/// println!("Created record with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/add-record/>
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

    pub fn send(self, client: &KintoneClient) -> Result<AddRecordResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Updates an existing record in a Kintone app.
///
/// This function creates a request to update a record in the specified app.
/// The record can be identified either by its ID or by a unique key field.
/// Only the fields specified in the record data will be updated.
///
/// # Arguments
/// * `app` - The ID of the Kintone app containing the record to update
/// * `id` (optional) - The ID of the record to update
/// * `update_key` (optional) - A unique key field and value to identify the record to update
/// * `record` (optional) - A Record containing the field data to update (only specified fields will be updated)
/// * `revision` (optional) - The expected revision number of the record to prevent conflicts
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::{Record, FieldValue};
/// use chrono::NaiveDate;
///
/// let record = Record::from([
///     ("status", FieldValue::SingleLineText("Completed".to_owned())),
///     ("completion_date", FieldValue::Date(Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()))),
/// ]);
///
/// let response = kintone::v1::record::update_record(123)
///     .id(456)
///     .record(record)
///     .revision(10)
///     .send(&client)?;
/// println!("Updated to revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/update-record/>
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

    pub fn send(self, client: &KintoneClient) -> Result<UpdateRecordResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Retrieves comments for a specific record in a Kintone app.
///
/// This function creates a request to get all comments associated with a specific record.
/// The comments can be ordered, paginated, and filtered using the available methods.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `record` - The ID of the record to get comments for
/// * `order` (optional) - The order to sort comments
/// * `offset` (optional) - The number of comments to skip
/// * `limit` (optional) - The maximum number of comments to return
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::Order;
///
/// let response = kintone::v1::record::get_comments(123, 456)
///     .order(Order::Desc)
///     .limit(50)
///     .send(&client)?;
/// println!("Found {} comments", response.comments.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/get-comments/>
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

    pub fn send(self, client: &KintoneClient) -> Result<GetCommentsResponse, ApiError> {
        self.builder.call(client)
    }
}

//-----------------------------------------------------------------------------

/// Adds a new comment to a specific record in a Kintone app.
///
/// This function creates a request to add a comment to a record. The comment
/// can include text and mentions of other users.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `record` - The ID of the record to add the comment to
/// * `comment` - The comment data including text and mentions
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::RecordComment;
///
/// let comment = RecordComment {
///     text: "This task is now complete.".to_owned(),
///     mentions: vec![],
/// };
/// let response = kintone::v1::record::add_comment(123, 456, comment).send(&client)?;
/// println!("Added comment with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/add-comment/>
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
    pub fn send(self, client: &KintoneClient) -> Result<AddCommentResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Deletes a specific comment from a record in a Kintone app.
///
/// This function creates a request to delete a comment from a record. Only the
/// comment author or users with appropriate permissions can delete comments.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `record` - The ID of the record containing the comment
/// * `comment` - The ID of the comment to delete
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::delete_comment(123, 456, 789).send(&client)?;
/// println!("Comment deleted successfully");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/delete-comment/>
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
    pub fn send(self, client: &KintoneClient) -> Result<DeleteCommentResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Updates the assignees of a record in a Kintone app.
///
/// This function creates a request to update the list of users assigned to a record.
/// This is typically used in workflow processes where tasks need to be reassigned.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `id` - The ID of the record to update assignees for
/// * `assignees` - A vector of user login names to assign to the record
/// * `revision` (optional) - The expected revision number of the record to prevent conflicts
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let assignees = vec!["user1".to_owned(), "user2".to_owned()];
/// let response = kintone::v1::record::update_assignees(123, 456, assignees)
///     .revision(10)
///     .send(&client)?;
/// println!("Updated assignees, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/update-assignees/>
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

    pub fn send(self, client: &KintoneClient) -> Result<UpdateAssigneesResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Updates the status of a record in a Kintone app workflow.
///
/// This function creates a request to change the status of a record by executing
/// a workflow action. The action moves the record from its current status to the next
/// status in the workflow.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `id` - The ID of the record to update the status for
/// * `action` - The name of the workflow action to execute
/// * `assignee` (optional) - The login name or code of the user to assign the record to
/// * `revision` (optional) - The expected revision number of the record to prevent conflicts
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::update_status(123, 456, "Submit for Review".to_owned())
///     .assignee("reviewer1".to_owned())
///     .revision(5)
///     .send(&client)?;
/// println!("Status updated, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/update-status/>
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

    pub fn send(self, client: &KintoneClient) -> Result<UpdateStatusResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Creates a cursor for paginating through large result sets efficiently.
///
/// This function creates a request to generate a cursor that can be used to retrieve
/// records in chunks. This is more efficient than using offset-based pagination for
/// large datasets as it provides consistent results even when records are being
/// added or modified during iteration.
///
/// # Arguments
/// * `app` - The ID of the Kintone app to create a cursor for
/// * `fields` (optional) - An array of field codes to include in the response
/// * `query` (optional) - A query string following Kintone's query syntax
/// * `size` (optional) - The number of records to retrieve per page (default: 100, max: 500)
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::create_cursor(123)
///     .query("status = \"Active\"")
///     .fields(&["name", "email", "status"])
///     .size(100)
///     .send(&client)?;
/// println!("Created cursor: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/create-cursor/>
pub fn create_cursor(app: u64) -> CreateCursorRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/records/cursor.json");
    CreateCursorRequest {
        builder,
        body: CreateCursorRequestBody {
            app,
            fields: None,
            query: None,
            size: None,
        },
    }
}

#[must_use]
pub struct CreateCursorRequest {
    builder: RequestBuilder,
    body: CreateCursorRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCursorRequestBody {
    app: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCursorResponse {
    pub id: String,
    #[serde(with = "stringified")]
    pub total_count: u64,
}

impl CreateCursorRequest {
    /// Specifies which fields to include in the response.
    ///
    /// # Arguments
    /// * `fields` - An array of field codes to retrieve
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.body.fields = Some(fields.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Sets a query to filter the records.
    ///
    /// # Arguments
    /// * `query` - A query string following Kintone's query syntax
    pub fn query(mut self, query: &str) -> Self {
        self.body.query = Some(query.to_string());
        self
    }

    /// Sets the number of records to retrieve per page.
    ///
    /// # Arguments
    /// * `size` - The page size (default: 100, max: 500)
    pub fn size(mut self, size: u64) -> Self {
        self.body.size = Some(size);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<CreateCursorResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Retrieves records using a previously created cursor.
///
/// This function creates a request to fetch records using a cursor ID obtained
/// from `create_cursor`. The cursor maintains state to efficiently paginate
/// through large result sets.
///
/// # Arguments
/// * `id` - The cursor ID returned from `create_cursor`
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// // First create a cursor
/// let cursor_response = kintone::v1::record::create_cursor(123)
///     .query("status = \"Active\"")
///     .size(100)
///     .send(&client)?;
///
/// // Then retrieve records using the cursor
/// let response = kintone::v1::record::get_records_by_cursor(&cursor_response.id)
///     .send(&client)?;
/// println!("Retrieved {} records", response.records.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/get-records-with-cursor/>
pub fn get_records_by_cursor(id: &str) -> GetRecordsByCursorRequest {
    let builder = RequestBuilder::new(http::Method::GET, "/v1/records/cursor.json").query("id", id);
    GetRecordsByCursorRequest { builder }
}

#[must_use]
pub struct GetRecordsByCursorRequest {
    builder: RequestBuilder,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordsByCursorResponse {
    pub records: Vec<Record>,
    pub next: bool,
}

impl GetRecordsByCursorRequest {
    pub fn send(self, client: &KintoneClient) -> Result<GetRecordsByCursorResponse, ApiError> {
        self.builder.call(client)
    }
}

//-----------------------------------------------------------------------------

/// Deletes a cursor to free up resources.
///
/// This function creates a request to delete a cursor when you're done using it.
/// While cursors automatically expire after a certain period, it's good practice
/// to explicitly delete them when no longer needed.
///
/// # Arguments
/// * `id` - The cursor ID to delete
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// # let cursor_id = "example-cursor-id";
/// let response = kintone::v1::record::delete_cursor(cursor_id).send(&client)?;
/// println!("Cursor deleted successfully");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/delete-cursor/>
pub fn delete_cursor(id: &str) -> DeleteCursorRequest {
    let builder = RequestBuilder::new(http::Method::DELETE, "/v1/records/cursor.json");
    DeleteCursorRequest {
        builder,
        body: DeleteCursorRequestBody { id: id.to_string() },
    }
}

#[must_use]
pub struct DeleteCursorRequest {
    builder: RequestBuilder,
    body: DeleteCursorRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCursorRequestBody {
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCursorResponse {
    // Empty response body
}

impl DeleteCursorRequest {
    pub fn send(self, client: &KintoneClient) -> Result<DeleteCursorResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------
