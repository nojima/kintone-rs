//! # Kintone Data Models
//!
//! This module provides data structures for representing Kintone records, fields, and other entities.
//! These models handle serialization/deserialization between Rust types and Kintone's JSON API format.
//!
//! ## Usage Examples
//!
//! ### Creating a Record
//! ```rust
//! use kintone::model::{Record, FieldValue};
//!
//! let mut record = Record::new();
//! record.put_field("name", FieldValue::SingleLineText("John Doe".to_owned()));
//! record.put_field("age", FieldValue::Number(30.into()));
//! println!("record = {record:?}");
//! ```
//!
//! ### Reading Field Values
//! ```rust
//! # use kintone::model::{Record, FieldValue};
//! # let record = Record::new();
//! if let Some(FieldValue::SingleLineText(name)) = record.get_field_value("name") {
//!     println!("Name: {}", name);
//! }
//! ```

use crate::internal::serde_helper::stringified;
use serde::{Deserialize, Serialize};

pub mod app;
pub mod record;
pub mod space;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    USER,
    GROUP,
    ORGANIZATION,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub type_: EntityType,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBody {
    pub content_type: String,
    pub file_key: String,
    pub name: String,
    #[serde(with = "stringified")]
    pub size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}
