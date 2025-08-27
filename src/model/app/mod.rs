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
//! use kintone::model::app::field::{single_line_text_field_property, FieldProperty};
//!
//! // Create a text field configuration using builder pattern
//! let text_field = single_line_text_field_property("employee_name")
//!     .label("Employee Name")
//!     .required(true)
//!     .max_length(50)
//!     .build();
//!
//! // Convert to the generic FieldProperty enum
//! let field_property: FieldProperty = text_field.into();
//! println!("Field type: {:?}", field_property.field_type());
//! ```

pub mod field;
