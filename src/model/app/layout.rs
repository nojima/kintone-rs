//! Form layout models.

use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::option_stringified;
use crate::model::record::FieldType;

/// A form consists of rows, tables and groups containing more rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Layout {
    Row {
        fields: Vec<LayoutField>,
    },
    Subtable {
        code: String,
        fields: Vec<LayoutField>,
    },
    Group {
        code: String,
        layout: Vec<Layout>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutField {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<LayoutSize>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSize {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub width: Option<u64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub height: Option<u64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub inner_height: Option<u64>,
}
