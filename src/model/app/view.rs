//! List, calendar and custom view settings.

use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::{option_stringified, stringified};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViewType {
    #[default]
    List,
    Calendar,
    Custom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViewDevice {
    #[default]
    Desktop,
    Any,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(rename = "type")]
    pub view_type: ViewType,
    #[serde(with = "stringified")]
    pub index: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified",
        skip_serializing
    )]
    pub id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none", skip_serializing)]
    pub builtin_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pager: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<ViewDevice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
