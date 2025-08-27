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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::app::field::FieldProperty;

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
