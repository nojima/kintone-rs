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
//! - [`add_records`] - Create multiple records at once
//! - [`update_record`] - Update an existing record
//! - [`update_records`] - Update multiple records at once
//! - [`delete_records`] - Delete multiple records at once
//! - [`bulk_request`] - Execute multiple API operations atomically
//!
//! ### Access Permissions
//! - [`evaluate_record_permissions`] - Evaluate permissions for records and fields
//!
//! ### Comment Operations
//! - [`get_comments`] - Retrieve comments for a record
//! - [`add_comment`] - Add a new comment to a record
//! - [`delete_comment`] - Delete a comment from a record
//!
//! ### Workflow Operations
//! - [`update_assignees`] - Update the assignees of a record
//! - [`update_status`] - Update the workflow status of a record
//! - [`update_statuses`] - Update the workflow statuses of multiple records
//!
//! ### Cursor-based Pagination
//! - [`create_cursor`] - Create a cursor for efficient pagination through large datasets
//! - [`get_records_by_cursor`] - Retrieve records using a cursor
//! - [`delete_cursor`] - Delete a cursor to free up resources

use bigdecimal::BigDecimal;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::{ApiError, BulkRequestError};
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::Order;
use crate::model::record::{PostedRecordComment, Record, RecordComment};

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
    pub(crate) body: AddRecordRequestBody,
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

/// Creates multiple new records in a Kintone app.
///
/// This function creates a request to add multiple records to the specified app at once.
/// This is more efficient than adding records one by one when you need to create many records.
///
/// # Arguments
/// * `app` - The ID of the Kintone app to add records to
/// * `records` - A vector of Records containing the field data for the new records
///
/// # Limits
/// - Maximum 100 records can be added in a single request
/// - If any record fails, all records in the request are rolled back
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::{Record, FieldValue};
///
/// let records = vec![
///     Record::from([
///         ("name", FieldValue::SingleLineText("Alice".to_owned())),
///         ("age", FieldValue::Number(Some(25.into()))),
///     ]),
///     Record::from([
///         ("name", FieldValue::SingleLineText("Bob".to_owned())),
///         ("age", FieldValue::Number(Some(30.into()))),
///     ]),
/// ];
///
/// let response = kintone::v1::record::add_records(123, records)
///     .send(&client)?;
/// println!("Created {} records", response.ids.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/add-records/>
pub fn add_records(app: u64, records: Vec<Record>) -> AddRecordsRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/records.json");
    AddRecordsRequest {
        builder,
        body: AddRecordsRequestBody { app, records },
    }
}

#[must_use]
pub struct AddRecordsRequest {
    builder: RequestBuilder,
    pub(crate) body: AddRecordsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRecordsRequestBody {
    app: u64,
    records: Vec<Record>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRecordsResponse {
    pub ids: Vec<String>,
    pub revisions: Vec<String>,
}

impl AddRecordsRequest {
    pub fn send(self, client: &KintoneClient) -> Result<AddRecordsResponse, ApiError> {
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

/// Value type for unique key field updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateKeyValue {
    /// String value for text fields
    String(String),
    /// Numeric value for number fields
    Number(BigDecimal),
}

impl From<String> for UpdateKeyValue {
    fn from(value: String) -> Self {
        UpdateKeyValue::String(value)
    }
}

impl From<&str> for UpdateKeyValue {
    fn from(value: &str) -> Self {
        UpdateKeyValue::String(value.to_string())
    }
}

impl From<BigDecimal> for UpdateKeyValue {
    fn from(value: BigDecimal) -> Self {
        UpdateKeyValue::Number(value)
    }
}

impl From<i64> for UpdateKeyValue {
    fn from(value: i64) -> Self {
        UpdateKeyValue::Number(BigDecimal::from(value))
    }
}

impl From<u64> for UpdateKeyValue {
    fn from(value: u64) -> Self {
        UpdateKeyValue::Number(BigDecimal::from(value))
    }
}

impl From<i32> for UpdateKeyValue {
    fn from(value: i32) -> Self {
        UpdateKeyValue::Number(BigDecimal::from(value))
    }
}

impl From<u32> for UpdateKeyValue {
    fn from(value: u32) -> Self {
        UpdateKeyValue::Number(BigDecimal::from(value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKey {
    pub field: String,
    pub value: UpdateKeyValue,
}

#[must_use]
pub struct UpdateRecordRequest {
    builder: RequestBuilder,
    pub(crate) body: UpdateRecordRequestBody,
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

    pub fn update_key(mut self, field: String, value: impl Into<UpdateKeyValue>) -> Self {
        self.body.update_key = Some(UpdateKey {
            field,
            value: value.into(),
        });
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

/// Updates multiple existing records in a Kintone app.
///
/// This function creates a request to update multiple records in the specified app at once.
/// Records can be identified either by their ID or by a unique key field. This is more efficient
/// than updating records one by one when you need to modify many records.
///
/// # Arguments
/// * `app` - The ID of the Kintone app containing the records to update
/// * `records` - A vector of UpdateRecordData containing the record update information
///
/// # Limits
/// - Maximum 100 records can be updated in a single request
/// - If any record update fails, all updates in the request are rolled back
/// - UPSERT mode can insert new records if they don't exist
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::{Record, FieldValue};
/// use kintone::v1::record::UpdateRecordData;
///
/// let updates = vec![
///     UpdateRecordData::new()
///         .id(123)
///         .record(Record::from([
///             ("status", FieldValue::SingleLineText("Completed".to_owned())),
///         ]))
///         .revision(5),
///     UpdateRecordData::new()
///         .update_key("code".to_owned(), "ABC123")  // String field
///         .record(Record::from([
///             ("priority", FieldValue::SingleLineText("High".to_owned())),
///         ])),
///     UpdateRecordData::new()
///         .update_key("employee_id".to_owned(), 12345)  // Number field
///         .record(Record::from([
///             ("department", FieldValue::SingleLineText("Engineering".to_owned())),
///         ])),
/// ];
///
/// let response = kintone::v1::record::update_records(456, updates)
///     .upsert(true)
///     .send(&client)?;
/// println!("Updated {} records", response.records.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/update-records/>
pub fn update_records(app: u64, records: Vec<UpdateRecordData>) -> UpdateRecordsRequest {
    let builder = RequestBuilder::new(http::Method::PUT, "/v1/records.json");
    UpdateRecordsRequest {
        builder,
        body: UpdateRecordsRequestBody {
            app,
            records,
            upsert: None,
        },
    }
}

/// Data for updating a single record in a bulk update operation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_key: Option<UpdateKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u64>,
}

impl UpdateRecordData {
    /// Creates a new UpdateRecordData instance.
    pub fn new() -> Self {
        Self {
            id: None,
            update_key: None,
            record: None,
            revision: None,
        }
    }

    /// Sets the record ID to update.
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the unique key to identify the record to update.
    pub fn update_key(mut self, field: String, value: impl Into<UpdateKeyValue>) -> Self {
        self.update_key = Some(UpdateKey {
            field,
            value: value.into(),
        });
        self
    }

    /// Sets the record data to update.
    pub fn record(mut self, record: Record) -> Self {
        self.record = Some(record);
        self
    }

    /// Sets the expected revision number for optimistic locking.
    pub fn revision(mut self, revision: u64) -> Self {
        self.revision = Some(revision);
        self
    }
}

impl Default for UpdateRecordData {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub struct UpdateRecordsRequest {
    builder: RequestBuilder,
    pub(crate) body: UpdateRecordsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordsRequestBody {
    app: u64,
    records: Vec<UpdateRecordData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upsert: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordsResponse {
    pub records: Vec<UpdatedRecordInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedRecordInfo {
    pub id: String,
    pub revision: String,
    /// The operation performed on the record (`INSERT` or `UPDATE`).
    /// Only returned in UPSERT mode.
    pub operation: Option<String>,
}

impl UpdateRecordsRequest {
    /// Enables UPSERT mode. When enabled, records that don't exist will be created.
    pub fn upsert(mut self, upsert: bool) -> Self {
        self.body.upsert = Some(upsert);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateRecordsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Deletes multiple records from a Kintone app.
///
/// This function creates a request to delete multiple records from the specified app at once.
/// This is more efficient than deleting records one by one when you need to remove many records.
///
/// # Arguments
/// * `app` - The ID of the Kintone app containing the records to delete
/// * `ids` - A vector of record IDs to delete
///
/// # Limits
/// - Maximum 100 records can be deleted in a single request
/// - If any record deletion fails, all deletions in the request are rolled back
/// - Optional revision numbers can be provided for optimistic locking
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let record_ids = vec![123, 456, 789];
/// let response = kintone::v1::record::delete_records(100, record_ids)
///     .send(&client)?;
/// println!("Records deleted successfully");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Example with revision numbers
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let record_ids = vec![123, 456];
/// let revisions = vec![5, 8];
/// let response = kintone::v1::record::delete_records(100, record_ids)
///     .revisions(revisions)
///     .send(&client)?;
/// println!("Records deleted successfully with revision check");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/delete-records/>
pub fn delete_records(app: u64, ids: Vec<u64>) -> DeleteRecordsRequest {
    let builder = RequestBuilder::new(http::Method::DELETE, "/v1/records.json");
    DeleteRecordsRequest {
        builder,
        body: DeleteRecordsRequestBody {
            app,
            ids,
            revisions: None,
        },
    }
}

#[must_use]
pub struct DeleteRecordsRequest {
    builder: RequestBuilder,
    pub(crate) body: DeleteRecordsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRecordsRequestBody {
    app: u64,
    ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revisions: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRecordsResponse {
    // Empty response body
}

impl DeleteRecordsRequest {
    /// Sets the expected revision numbers for optimistic locking.
    ///
    /// The length of the revisions vector should match the length of the IDs vector.
    /// Use -1 or omit to skip revision checking for specific records.
    pub fn revisions(mut self, revisions: Vec<u64>) -> Self {
        self.body.revisions = Some(revisions);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<DeleteRecordsResponse, ApiError> {
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
/// use kintone::model::record::record_comment;
///
/// let comment = record_comment("This task is now complete.").build();
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
    pub(crate) body: UpdateAssigneesRequestBody,
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
    pub(crate) body: UpdateStatusRequestBody,
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

/// Updates the statuses of multiple records in a Kintone app workflow.
///
/// This function creates a request to execute workflow actions on multiple records at once.
///
/// # Arguments
/// * `app` - The ID of the Kintone app
/// * `records` - The records to update the status for
///
/// # Limits
/// - Maximum 100 records can be updated in a single request
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::v1::record::UpdateStatusData;
///
/// let records = vec![
///     UpdateStatusData::new(123, "Submit for Review".to_owned())
///         .assignee("reviewer1".to_owned())
///         .revision(5),
///     UpdateStatusData::new(124, "Approve".to_owned()),
/// ];
/// let response = kintone::v1::record::update_statuses(456, records).send(&client)?;
/// for record in response.records {
///     println!("Record {} updated, new revision: {}", record.id, record.revision);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/update-statuses/>
pub fn update_statuses(app: u64, records: Vec<UpdateStatusData>) -> UpdateStatusesRequest {
    let builder = RequestBuilder::new(http::Method::PUT, "/v1/records/status.json");
    UpdateStatusesRequest {
        builder,
        body: UpdateStatusesRequestBody { app, records },
    }
}

/// Data for updating the status of a single record in [`update_statuses`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusData {
    pub id: u64,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u64>,
}

impl UpdateStatusData {
    /// Creates a new UpdateStatusData instance.
    ///
    /// # Arguments
    /// * `id` - The ID of the record to update the status for
    /// * `action` - The name of the workflow action to execute
    pub fn new(id: u64, action: String) -> Self {
        Self {
            id,
            action,
            assignee: None,
            revision: None,
        }
    }

    /// Sets the login name of the user to assign the record to.
    pub fn assignee(mut self, assignee: String) -> Self {
        self.assignee = Some(assignee);
        self
    }

    /// Sets the expected revision number for optimistic locking.
    pub fn revision(mut self, revision: u64) -> Self {
        self.revision = Some(revision);
        self
    }
}

#[must_use]
pub struct UpdateStatusesRequest {
    builder: RequestBuilder,
    pub(crate) body: UpdateStatusesRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusesRequestBody {
    app: u64,
    records: Vec<UpdateStatusData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusesResponse {
    pub records: Vec<UpdatedStatusInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedStatusInfo {
    #[serde(with = "stringified")]
    pub id: u64,
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl UpdateStatusesRequest {
    pub fn send(self, client: &KintoneClient) -> Result<UpdateStatusesResponse, ApiError> {
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

/// Executes multiple API requests in a single bulk operation.
///
/// This function creates a request to execute multiple API operations atomically.
/// If any operation fails, all operations are rolled back. This is useful for
/// maintaining data consistency across multiple apps or complex operations.
///
/// # Arguments
/// * `requests` - A vector of BulkRequestItem containing the operations to execute
///
/// # Limits
/// - Maximum 20 requests can be executed in a single bulk request
/// - All operations are executed atomically (all succeed or all fail)
/// - Supports record operations, status updates, and assignee updates
///
/// # Errors
/// If one of the requests fails, [`ApiError::BulkRequest`] is returned. It contains
/// the index of the failed request and the error returned by Kintone.
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::record::{Record, FieldValue};
/// use kintone::v1::record::{BulkRequestItem, BulkRequestResult, bulk_request};
///
/// let requests = vec![
///     // Add a record
///     kintone::v1::record::add_record(100)
///         .record(Record::from([
///             ("name", FieldValue::SingleLineText("New Record".to_owned())),
///         ]))
///         .try_into()?,
///     // Update a record
///     kintone::v1::record::update_record(101)
///         .id(456)
///         .record(Record::from([
///             ("status", FieldValue::SingleLineText("Updated".to_owned())),
///         ]))
///         .try_into()?,
///     // Delete records
///     kintone::v1::record::delete_records(102, vec![789, 790])
///         .try_into()?,
/// ];
///
/// let response = bulk_request(requests).send(&client)?;
/// for result in response.results {
///     match result {
///         BulkRequestResult::AddRecord(r) => println!("Added record {}", r.id),
///         BulkRequestResult::UpdateRecord(r) => println!("Updated to revision {}", r.revision),
///         BulkRequestResult::DeleteRecords(_) => println!("Deleted records"),
///         _ => {}
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/bulk-request/>
pub fn bulk_request(requests: Vec<BulkRequestItem>) -> BulkRequestRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/bulkRequest.json");
    BulkRequestRequest {
        builder,
        body: BulkRequestRequestBody { requests },
    }
}

/// Represents a single request item in a bulk operation.
///
/// A `BulkRequestItem` is created from a request of a supported API with `try_into()`.
#[derive(Debug, Clone)]
pub struct BulkRequestItem {
    /// HTTP method for the request
    method: http::Method,
    /// API endpoint path without the "/k" prefix
    api_path: &'static str,
    /// Request payload
    payload: serde_json::Value,
    /// Which API this request calls
    kind: BulkRequestKind,
}

#[derive(Debug, Clone, Copy)]
enum BulkRequestKind {
    AddRecord,
    AddRecords,
    UpdateRecord,
    UpdateRecords,
    DeleteRecords,
    UpdateAssignees,
    UpdateStatus,
    UpdateStatuses,
}

macro_rules! impl_try_from_for_bulk_request_item {
    ($request:ty, $method:ident, $api_path:literal, $kind:ident) => {
        impl TryFrom<$request> for BulkRequestItem {
            type Error = serde_json::Error;

            fn try_from(request: $request) -> Result<Self, Self::Error> {
                Ok(Self {
                    method: http::Method::$method,
                    api_path: $api_path,
                    payload: serde_json::to_value(request.body)?,
                    kind: BulkRequestKind::$kind,
                })
            }
        }
    };
}

impl_try_from_for_bulk_request_item!(AddRecordRequest, POST, "/v1/record.json", AddRecord);
impl_try_from_for_bulk_request_item!(AddRecordsRequest, POST, "/v1/records.json", AddRecords);
impl_try_from_for_bulk_request_item!(UpdateRecordRequest, PUT, "/v1/record.json", UpdateRecord);
impl_try_from_for_bulk_request_item!(UpdateRecordsRequest, PUT, "/v1/records.json", UpdateRecords);
impl_try_from_for_bulk_request_item!(
    DeleteRecordsRequest,
    DELETE,
    "/v1/records.json",
    DeleteRecords
);
impl_try_from_for_bulk_request_item!(
    UpdateAssigneesRequest,
    PUT,
    "/v1/record/assignees.json",
    UpdateAssignees
);
impl_try_from_for_bulk_request_item!(
    UpdateStatusRequest,
    PUT,
    "/v1/record/status.json",
    UpdateStatus
);
impl_try_from_for_bulk_request_item!(
    UpdateStatusesRequest,
    PUT,
    "/v1/records/status.json",
    UpdateStatuses
);

#[must_use]
pub struct BulkRequestRequest {
    builder: RequestBuilder,
    body: BulkRequestRequestBody,
}

struct BulkRequestRequestBody {
    requests: Vec<BulkRequestItem>,
}

impl BulkRequestRequestBody {
    fn to_json<'a>(&'a self, client: &KintoneClient) -> BulkRequestRequestBodyJson<'a> {
        BulkRequestRequestBodyJson {
            requests: self
                .requests
                .iter()
                .map(|r| BulkRequestItemJson {
                    method: &r.method,
                    api: client.full_api_path(r.api_path),
                    payload: &r.payload,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct BulkRequestRequestBodyJson<'a> {
    requests: Vec<BulkRequestItemJson<'a>>,
}

#[derive(Serialize)]
struct BulkRequestItemJson<'a> {
    #[serde(with = "stringified")]
    method: &'a http::Method,
    api: String,
    payload: &'a serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct BulkRequestResponse {
    /// The results of the requests, in the same order as the requests.
    pub results: Vec<BulkRequestResult>,
}

/// The result of a single request in a bulk request.
///
/// Each variant corresponds to the API of the request at the same position.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BulkRequestResult {
    AddRecord(AddRecordResponse),
    AddRecords(AddRecordsResponse),
    UpdateRecord(UpdateRecordResponse),
    UpdateRecords(UpdateRecordsResponse),
    DeleteRecords(DeleteRecordsResponse),
    UpdateAssignees(UpdateAssigneesResponse),
    UpdateStatus(UpdateStatusResponse),
    UpdateStatuses(UpdateStatusesResponse),
}

impl BulkRequestResult {
    fn from_value(kind: BulkRequestKind, value: serde_json::Value) -> serde_json::Result<Self> {
        use serde_json::from_value;
        Ok(match kind {
            BulkRequestKind::AddRecord => Self::AddRecord(from_value(value)?),
            BulkRequestKind::AddRecords => Self::AddRecords(from_value(value)?),
            BulkRequestKind::UpdateRecord => Self::UpdateRecord(from_value(value)?),
            BulkRequestKind::UpdateRecords => Self::UpdateRecords(from_value(value)?),
            BulkRequestKind::DeleteRecords => Self::DeleteRecords(from_value(value)?),
            BulkRequestKind::UpdateAssignees => Self::UpdateAssignees(from_value(value)?),
            BulkRequestKind::UpdateStatus => Self::UpdateStatus(from_value(value)?),
            BulkRequestKind::UpdateStatuses => Self::UpdateStatuses(from_value(value)?),
        })
    }
}

#[derive(Deserialize)]
struct BulkRequestResponseJson {
    results: Vec<serde_json::Value>,
}

impl BulkRequestRequest {
    pub fn send(self, client: &KintoneClient) -> Result<BulkRequestResponse, ApiError> {
        let requests = &self.body.requests;
        let body = self.body.to_json(client);
        let resp: BulkRequestResponseJson = match self.builder.send(client, body) {
            Ok(resp) => resp,
            Err(ApiError::Http(err)) => {
                return Err(match BulkRequestError::from_http_error(&err) {
                    Some(bulk_err) => bulk_err.into(),
                    None => err.into(),
                });
            }
            Err(err) => return Err(err),
        };
        // Results are matched with requests by position, so the numbers must be the same.
        if resp.results.len() != requests.len() {
            return Err(<serde_json::Error as serde::de::Error>::custom(format!(
                "the number of results ({}) does not match the number of requests ({})",
                resp.results.len(),
                requests.len()
            ))
            .into());
        }
        let results = requests
            .iter()
            .zip(resp.results)
            .map(|(r, value)| BulkRequestResult::from_value(r.kind, value))
            .collect::<Result<_, _>>()?;
        Ok(BulkRequestResponse { results })
    }
}

//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Auth;

    fn client(guest_space_id: Option<u64>) -> KintoneClient {
        let builder =
            KintoneClient::builder("https://example.cybozu.com", Auth::api_token("t".to_owned()));
        match guest_space_id {
            Some(id) => builder.guest_space_id(id).build(),
            None => builder.build(),
        }
    }

    #[test]
    fn serialize_update_statuses_request() {
        let req = update_statuses(
            4,
            vec![
                UpdateStatusData::new(1, "申請する".to_owned())
                    .assignee("user2".to_owned())
                    .revision(1),
                UpdateStatusData::new(2, "承認".to_owned()),
            ],
        );
        let actual = serde_json::to_value(&req.body).unwrap();
        // Taken from https://cybozu.dev/ja/kintone/docs/rest-api/records/update-statuses/
        let expected = serde_json::json!({
            "app": 4,
            "records": [
                { "id": 1, "action": "申請する", "assignee": "user2", "revision": 1 },
                { "id": 2, "action": "承認" }
            ]
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize_update_statuses_response() {
        let json =
            r#"{ "records": [ { "id": "1", "revision": "3" }, { "id": "2", "revision": "9" } ] }"#;
        let resp: UpdateStatusesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.records.len(), 2);
        assert_eq!((resp.records[0].id, resp.records[0].revision), (1, 3));
        assert_eq!((resp.records[1].id, resp.records[1].revision), (2, 9));
    }

    fn bulk_body() -> BulkRequestRequestBody {
        BulkRequestRequestBody {
            requests: vec![
                add_record(1).try_into().unwrap(),
                update_statuses(2, vec![UpdateStatusData::new(3, "承認".to_owned())])
                    .try_into()
                    .unwrap(),
            ],
        }
    }

    #[test]
    fn serialize_bulk_request() {
        let actual = serde_json::to_value(bulk_body().to_json(&client(None))).unwrap();
        let expected = serde_json::json!({
            "requests": [
                { "method": "POST", "api": "/k/v1/record.json", "payload": { "app": 1, "record": null } },
                {
                    "method": "PUT",
                    "api": "/k/v1/records/status.json",
                    "payload": { "app": 2, "records": [ { "id": 3, "action": "承認" } ] }
                }
            ]
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize_bulk_request_in_guest_space() {
        let body = bulk_body();
        let json = serde_json::to_value(body.to_json(&client(Some(3)))).unwrap();
        assert_eq!(json["requests"][0]["api"], "/k/guest/3/v1/record.json");
        assert_eq!(json["requests"][1]["api"], "/k/guest/3/v1/records/status.json");
    }

    #[test]
    fn deserialize_update_records_response() {
        // Without upsert, kintone does not return `operation`.
        let json = r#"{ "records": [ { "id": "1", "revision": "5" } ] }"#;
        let resp: UpdateRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.records[0].operation, None);

        let json = r#"{ "records": [ { "id": "1", "revision": "6", "operation": "UPDATE" } ] }"#;
        let resp: UpdateRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.records[0].operation.as_deref(), Some("UPDATE"));
    }

    #[test]
    fn deserialize_bulk_request_results() {
        let kinds = [BulkRequestKind::AddRecord, BulkRequestKind::UpdateRecord];
        // Taken from https://cybozu.dev/ja/kintone/docs/rest-api/records/bulk-request/
        let values = [
            serde_json::json!({ "id": "39", "revision": "1" }),
            serde_json::json!({ "revision": "3" }),
        ];
        let results: Vec<_> = kinds
            .into_iter()
            .zip(values)
            .map(|(kind, value)| BulkRequestResult::from_value(kind, value).unwrap())
            .collect();
        assert!(
            matches!(&results[0], BulkRequestResult::AddRecord(r) if r.id == 39 && r.revision == 1)
        );
        assert!(matches!(&results[1], BulkRequestResult::UpdateRecord(r) if r.revision == 3));
    }
}

/// Evaluates the current user's permissions for specific records.
///
/// This function creates a request to get the effective viewing, editing and deletion permissions
/// for records and their fields. Supply record IDs with the `ids()` method.
///
/// **Required Permissions:** This API requires record viewing or record creation permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `app_id` - The ID of the Kintone app
/// * `ids` - Record IDs to evaluate (up to 100), supplied with `ids()`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::record::evaluate_record_permissions(123)
///     .ids([1, 2])
///     .send(&client)?;
/// for right in response.rights {
///     println!("Record {} is editable: {}", right.id, right.record.editable);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/records/evaluate-record-permissions/>
pub fn evaluate_record_permissions(app_id: u64) -> EvaluateRecordPermissionsRequest {
    EvaluateRecordPermissionsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/records/acl/evaluate.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct EvaluateRecordPermissionsRequest {
    builder: RequestBuilder,
}

impl EvaluateRecordPermissionsRequest {
    /// Sets the record IDs to evaluate (up to 100 IDs).
    pub fn ids(mut self, values: impl IntoIterator<Item = u64>) -> Self {
        let values: Vec<_> = values.into_iter().collect();
        self.builder = self.builder.query_array("ids", &values);
        self
    }

    /// Sends the request to evaluate the current user's permissions for specific records.
    ///
    /// # Returns
    ///
    /// A Result containing the [`EvaluateRecordPermissionsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires record viewing or record creation permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(
        self,
        client: &KintoneClient,
    ) -> Result<EvaluateRecordPermissionsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRecordPermissionsResponse {
    pub rights: Vec<EvaluatedRecordRight>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluatedRecordRight {
    #[serde(with = "stringified")]
    pub id: u64,
    pub record: RecordPermission,
    pub fields: HashMap<String, FieldPermission>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordPermission {
    pub viewable: bool,
    pub editable: bool,
    pub deletable: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldPermission {
    pub viewable: bool,
    pub editable: bool,
}
