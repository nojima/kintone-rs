//! # Kintone App Models
//!
//! This module contains all the data structures and configurations related to
//! Kintone applications, including field definitions and app settings.
//!
//! # Modules
//!
//! - [`field`] - Field property definitions and configurations for different field types
//!
//! # Examples
//!
//! Working with app field configurations:
//!
//! ```rust
//! use kintone::model::app::field::{FieldProperty, SingleLineTextFieldProperty};
//!
//! // Create a text field configuration
//! let text_field = SingleLineTextFieldProperty {
//!     code: "employee_name".to_string(),
//!     label: "Employee Name".to_string(),
//!     required: true,
//!     max_length: Some(50),
//!     ..Default::default()
//! };
//!
//! // Convert to the generic FieldProperty enum
//! let field_property: FieldProperty = text_field.into();
//! println!("Field type: {:?}", field_property.field_type());
//! ```

pub mod field;
