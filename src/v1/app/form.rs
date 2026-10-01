//! # Kintone App Form API
//!
//! This module provides functions for managing form fields in Kintone apps.
//! It includes operations for adding, updating, and removing fields in the preview environment.
//!
//! ## Available Operations
//!
//! ### Form Field Management
//! - [`add_form_field`] - Add a new field to an app's form in the preview environment
//! - [`get_form_fields`], [`update_form_fields`], [`delete_form_fields`] - Read, update and remove fields
//! - [`get_form_layout`], [`update_form_layout`] - Read and update form layout
//! - [`get_form`] - Read legacy form design information
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! use kintone::model::app::field::single_line_text_field_property;
//!
//! let field = single_line_text_field_property("my_field")
//!     .label("My Field")
//!     .required(true)
//!     .max_length(50)
//!     .build();
//!
//! let response = kintone::v1::app::form::add_form_field(123)
//!     .field(field.into()) // Don't forget .into()
//!     .revision(Some(5))
//!     .send(&client)?;
//! println!("Updated app with revision: {}", response.revision);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Form APIs modify the preview environment. Use the deploy API to apply changes to production.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::settings::RevisionResponse;
use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::app::field::{FieldProperty, FieldPropertyUpdate};
use crate::model::app::layout::Layout;

/// Adds new fields to an app's form in the preview environment.
///
/// This function creates a request to add one or more fields to a Kintone app's form.
/// The changes are made to the preview environment and need to be deployed to take effect
/// in the production environment.
///
/// **Important**: This API requires app management permissions.
///
/// **Important**: Fields added with this function exist only in the preview environment.
/// To deploy the changes to the production environment, use [`crate::v1::app::settings::deploy_app`].
///
/// # Arguments
/// * `app_id` - The ID of the app to add fields to
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::app::field::single_line_text_field_property;
///
/// let text_field = single_line_text_field_property("customer_name")
///     .label("Customer Name")
///     .required(true)
///     .max_length(50)
///     .build();
///
/// let response = kintone::v1::app::form::add_form_field(123)
///     .field(text_field.into())
///     .send(&client)?;
/// println!("Added field, new revision: {}", response.revision);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/add-form-fields/>
pub fn add_form_field(app_id: u64) -> AddFormFieldRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/preview/app/form/fields.json");
    AddFormFieldRequest {
        builder,
        body: AddFormFieldRequestBody {
            app: app_id,
            properties: HashMap::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct AddFormFieldRequest {
    builder: RequestBuilder,
    body: AddFormFieldRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddFormFieldRequestBody {
    #[serde(with = "stringified")]
    app: u64,
    properties: HashMap<String, FieldProperty>,
    #[serde(with = "option_stringified")]
    revision: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFormFieldResponse {
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl AddFormFieldRequest {
    /// Adds a field to be created.
    pub fn field(mut self, field_property: FieldProperty) -> Self {
        self.body
            .properties
            .insert(field_property.field_code().to_owned(), field_property);
        self
    }

    /// Sets the expected revision number for validation.
    ///
    /// If provided and the actual revision doesn't match, the request will fail.
    /// Use `None` or omit this call to skip revision validation.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    /// Sends the request to add the fields.
    ///
    /// # Returns
    /// A Result containing the AddFormFieldResponse with the new revision number, or an ApiError.
    ///
    /// # Authentication
    /// This API requires app management permissions.
    pub fn send(self, client: &KintoneClient) -> Result<AddFormFieldResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get form fields.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/get-form-fields/>
pub fn get_form_fields(app_id: u64) -> GetFormFieldsRequest {
    GetFormFieldsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/form/fields.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetFormFieldsRequest {
    builder: RequestBuilder,
}

impl GetFormFieldsRequest {
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.builder = self.builder.query("lang", lang.into());
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetFormFieldsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFormFieldsResponse {
    pub properties: HashMap<String, FieldProperty>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateFormFieldsResponse = RevisionResponse;

/// Update form fields.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/update-form-fields/>
pub fn update_form_fields(app_id: u64) -> UpdateFormFieldsRequest {
    UpdateFormFieldsRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/form/fields.json"),
        body: UpdateFormFieldsRequestBody {
            app: app_id,
            properties: HashMap::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateFormFieldsRequest {
    builder: RequestBuilder,
    body: UpdateFormFieldsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateFormFieldsRequestBody {
    app: u64,
    properties: HashMap<String, FormFieldUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum FormFieldUpdate {
    Typed(Box<FieldPropertyUpdate>),
    Json(serde_json::Value),
}

impl UpdateFormFieldsRequest {
    pub fn properties(
        mut self,
        values: impl IntoIterator<Item = (String, FieldPropertyUpdate)>,
    ) -> Self {
        self.body.properties = values
            .into_iter()
            .map(|(code, value)| (code, FormFieldUpdate::Typed(Box::new(value))))
            .collect();
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: FieldPropertyUpdate) -> Self {
        self.body
            .properties
            .insert(name.into(), FormFieldUpdate::Typed(Box::new(value)));
        self
    }

    /// Sets a field update using its API JSON representation.
    /// Use this for empty string values that clear numeric limits or precision,
    /// and for field specific settings that the typed update cannot express.
    pub fn raw_field(mut self, code: impl Into<String>, value: serde_json::Value) -> Self {
        self.body.properties.insert(code.into(), FormFieldUpdate::Json(value));
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateFormFieldsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type DeleteFormFieldsResponse = RevisionResponse;

/// Delete form fields.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/delete-form-fields/>
pub fn delete_form_fields(app_id: u64) -> DeleteFormFieldsRequest {
    DeleteFormFieldsRequest {
        builder: RequestBuilder::new(http::Method::DELETE, "/v1/preview/app/form/fields.json"),
        body: DeleteFormFieldsRequestBody {
            app: app_id,
            fields: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct DeleteFormFieldsRequest {
    builder: RequestBuilder,
    body: DeleteFormFieldsRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteFormFieldsRequestBody {
    app: u64,
    fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl DeleteFormFieldsRequest {
    pub fn fields(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.fields = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<DeleteFormFieldsResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get form layout.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/get-form-layout/>
pub fn get_form_layout(app_id: u64) -> GetFormLayoutRequest {
    GetFormLayoutRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/app/form/layout.json")
            .query("app", app_id),
    }
}

#[must_use]
pub struct GetFormLayoutRequest {
    builder: RequestBuilder,
}

impl GetFormLayoutRequest {
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetFormLayoutResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFormLayoutResponse {
    pub layout: Vec<Layout>,
    #[serde(with = "stringified")]
    pub revision: u64,
}

pub type UpdateFormLayoutResponse = RevisionResponse;

/// Update form layout.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/update-form-layout/>
pub fn update_form_layout(app_id: u64) -> UpdateFormLayoutRequest {
    UpdateFormLayoutRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/preview/app/form/layout.json"),
        body: UpdateFormLayoutRequestBody {
            app: app_id,
            layout: Vec::new(),
            revision: None,
        },
    }
}

#[must_use]
pub struct UpdateFormLayoutRequest {
    builder: RequestBuilder,
    body: UpdateFormLayoutRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateFormLayoutRequestBody {
    app: u64,
    layout: Vec<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<u64>,
}

impl UpdateFormLayoutRequest {
    pub fn layout(mut self, values: impl IntoIterator<Item = Layout>) -> Self {
        self.body.layout = values.into_iter().collect();
        self
    }

    /// Sets the expected revision; None disables the revision check.
    pub fn revision(mut self, revision: Option<u64>) -> Self {
        self.body.revision = revision;
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateFormLayoutResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get form.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/apps/form/get-form/>
pub fn get_form(app_id: u64) -> GetFormRequest {
    GetFormRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/form.json").query("app", app_id),
    }
}

#[must_use]
pub struct GetFormRequest {
    builder: RequestBuilder,
}

impl GetFormRequest {
    /// Selects the preview environment when true (live by default).
    pub fn preview(mut self, preview: bool) -> Self {
        self.builder = self.builder.preview(preview);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetFormResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFormResponse {
    pub properties: Vec<serde_json::Value>,
}
