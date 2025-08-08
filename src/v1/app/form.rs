//! # Kintone App Form API
//!
//! This module provides functions for managing form fields in Kintone apps.
//! It includes operations for adding, updating, and removing fields in the preview environment.
//!
//! ## Available Operations
//!
//! ### Form Field Management
//! - [`add_form_field`] - Add a new field to an app's form in the preview environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```rust
//! # use kintone::client::{Auth, KintoneClient};
//! # use kintone::v1::app::form;
//! # use kintone::models::app::field::{FieldProperty, SingleLineTextFieldProperty};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_string(), "pass".to_string()));
//! let field = FieldProperty::SingleLineText(SingleLineTextFieldProperty {
//!     code: "my_field".to_string(),
//!     label: "My Field".to_string(),
//!     no_label: false,
//!     required: true,
//!     unique: false,
//!     max_length: Some(100),
//!     min_length: None,
//!     default_value: Some("".to_string()),
//!     expression: None,
//!     hide_expression: false,
//! });
//!
//! let response = form::add_form_field(123)
//!     .field("my_field", field)
//!     .revision(Some(5))
//!     .send(&client)?;
//! println!("Updated app with revision: {}", response.revision);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Form APIs modify the preview environment. Use the deploy API to apply changes to production.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::models::app::field::FieldProperty;

/// Adds new fields to an app's form in the preview environment.
///
/// This function creates a request to add one or more fields to a Kintone app's form.
/// The changes are made to the preview environment and need to be deployed to take effect
/// in the production environment.
///
/// **Important**: This API requires app management permissions.
///
/// **Note**: Fields added with this function exist only in the preview environment.
/// To deploy the changes to the production environment, use the deploy app API.
///
/// # Arguments
/// * `app_id` - The ID of the app to add fields to
///
/// # Example
/// ```rust
/// use kintone::models::app::field::{FieldProperty, SingleLineTextFieldProperty};
///
/// let text_field = FieldProperty::SingleLineText(SingleLineTextFieldProperty {
///     code: "customer_name".to_string(),
///     label: "Customer Name".to_string(),
///     no_label: false,
///     required: true,
///     unique: false,
///     min_length: None,
///     max_length: Some(100),
///     expression: None,
///     hide_expression: false,
///     default_value: Some("".to_string()),
/// });
///
/// let response = add_form_field(123)
///     .field("customer_name", text_field)
///     .send(&client)?;
/// println!("Added field, new revision: {}", response.revision);
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
    ///
    /// # Arguments
    /// * `field_code` - The unique field code for the new field
    /// * `field_property` - The field configuration including type, label, and other settings
    ///
    /// # Example
    /// ```rust
    /// use kintone::models::app::field::{FieldProperty, NumberFieldProperty, UnitPosition};
    /// use bigdecimal::BigDecimal;
    ///
    /// let number_field = FieldProperty::Number(NumberFieldProperty {
    ///     code: "price".to_string(),
    ///     label: "Price".to_string(),
    ///     required: true,
    ///     min_value: Some(0.into()),
    ///     digit: true,
    ///     display_scale: Some(2),
    ///     unit: Some("USD".to_string()),
    ///     unit_position: Some(UnitPosition::Before),
    ///     ..Default::default()
    /// });
    ///
    /// let request = add_form_field(123)
    ///     .field("price", number_field);
    /// ```
    pub fn field(mut self, field_code: impl Into<String>, field_property: FieldProperty) -> Self {
        self.body
            .properties
            .insert(field_code.into(), field_property);
        self
    }

    /// Sets the expected revision number for validation.
    ///
    /// If provided and the actual revision doesn't match, the request will fail.
    /// Use `None` or omit this call to skip revision validation.
    ///
    /// # Arguments
    /// * `revision` - The expected revision number, or None to skip validation
    ///
    /// # Example
    /// ```rust
    /// let request = add_form_field(123)
    ///     .revision(Some(5));  // Validate that app is at revision 5
    /// ```
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
