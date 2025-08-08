//! # Kintone Data Models
//!
//! This module provides data structures for representing Kintone records, fields, and other entities.
//! These models handle serialization/deserialization between Rust types and Kintone's JSON API format.
//!
//! ## Usage Examples
//!
//! ### Creating a Record
//! ```rust
//! use kintone::models::{Record, FieldValue};
//!
//! let mut record = Record::new();
//! record.put_field("name", FieldValue::SingleLineText("John Doe".to_string()));
//! record.put_field("age", FieldValue::Number(30.into()));
//! println!("record = {record:?}");
//! ```
//!
//! ### Reading Field Values
//! ```rust
//! # use kintone::models::{Record, FieldValue};
//! # let record = Record::new();
//! if let Some(FieldValue::SingleLineText(name)) = record.get_field_value("name") {
//!     println!("Name: {}", name);
//! }
//! ```

use std::collections::{HashMap, hash_map};

use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::stringified;

#[derive(Clone, Serialize, Deserialize)]
pub struct Record {
    #[serde(flatten)]
    fields: HashMap<String, FieldValue>,
}

impl Record {
    pub fn new() -> Self {
        Record {
            fields: HashMap::new(),
        }
    }

    pub fn clone_without_builtins(&self) -> Self {
        let size = self
            .field_values()
            .filter(|v| !v.field_type().is_builtin())
            .count();
        let mut fields = HashMap::with_capacity(size);
        for (code, value) in self.fields() {
            if !value.field_type().is_builtin() {
                fields.insert(code.clone(), value.clone());
            }
        }
        Record { fields }
    }

    pub fn get_field_value(&self, field_code: &str) -> Option<&FieldValue> {
        self.fields.get(field_code)
    }

    pub fn fields(&self) -> FieldIter {
        FieldIter {
            inner: self.fields.iter(),
        }
    }

    pub fn field_codes(&self) -> FieldCodeIter {
        FieldCodeIter {
            inner: self.fields.keys(),
        }
    }

    pub fn field_values(&self) -> FieldValueIter {
        FieldValueIter {
            inner: self.fields.values(),
        }
    }

    pub fn put_field(
        &mut self,
        field_code: impl Into<String>,
        value: FieldValue,
    ) -> Option<FieldValue> {
        self.fields.insert(field_code.into(), value)
    }

    pub fn remove_field(&mut self, field_code: &str) -> Option<FieldValue> {
        self.fields.remove(field_code)
    }

    pub fn id(&self) -> Option<u64> {
        let FieldValue::__ID__(value) = self.get_field_value("$id")? else {
            return None;
        };
        Some(*value)
    }

    pub fn revision(&self) -> Option<u64> {
        let FieldValue::__REVISION__(value) = self.get_field_value("$revision")? else {
            return None;
        };
        Some(*value)
    }
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Record");

        let mut keys: Vec<_> = self.fields.keys().by_ref().collect();
        keys.sort();

        for key in keys {
            let value = self.fields.get(key).unwrap();
            debug_struct.field(key, value);
        }

        debug_struct.finish()
    }
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct FieldIter<'a> {
    inner: hash_map::Iter<'a, String, FieldValue>,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = (&'a String, &'a FieldValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Clone)]
pub struct FieldCodeIter<'a> {
    inner: hash_map::Keys<'a, String, FieldValue>,
}

impl<'a> Iterator for FieldCodeIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Clone)]
pub struct FieldValueIter<'a> {
    inner: hash_map::Values<'a, String, FieldValue>,
}

impl<'a> Iterator for FieldValueIter<'a> {
    type Item = &'a FieldValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Assoc)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn is_builtin(&self) -> bool)]
#[non_exhaustive]
pub enum FieldType {
    #[assoc(is_builtin = false)]
    Calc,

    #[assoc(is_builtin = true)]
    Category,

    #[assoc(is_builtin = false)]
    CheckBox,

    #[assoc(is_builtin = true)]
    CreatedTime,

    #[assoc(is_builtin = true)]
    Creator,

    #[assoc(is_builtin = false)]
    Date,

    #[assoc(is_builtin = false)]
    Datetime,

    #[assoc(is_builtin = false)]
    DropDown,

    #[assoc(is_builtin = false)]
    File,

    #[assoc(is_builtin = false)]
    Group,

    #[assoc(is_builtin = false)]
    GroupSelect,

    #[assoc(is_builtin = false)]
    Hr,

    #[assoc(is_builtin = false)]
    Label,

    #[assoc(is_builtin = false)]
    Link,

    #[assoc(is_builtin = true)]
    Modifier,

    #[assoc(is_builtin = false)]
    MultiLineText,

    #[assoc(is_builtin = false)]
    MultiSelect,

    #[assoc(is_builtin = false)]
    Number,

    #[assoc(is_builtin = false)]
    OrganizationSelect,

    #[assoc(is_builtin = false)]
    RadioButton,

    #[assoc(is_builtin = true)]
    RecordNumber,

    #[assoc(is_builtin = false)]
    ReferenceTable,

    #[assoc(is_builtin = false)]
    RichText,

    #[assoc(is_builtin = false)]
    SingleLineText,

    #[assoc(is_builtin = false)]
    Spacer,

    #[assoc(is_builtin = true)]
    Status,

    #[assoc(is_builtin = true)]
    StatusAssignee,

    #[assoc(is_builtin = false)]
    Subtable,

    #[assoc(is_builtin = false)]
    Time,

    #[assoc(is_builtin = true)]
    UpdatedTime,

    #[assoc(is_builtin = false)]
    UserSelect,

    #[serde(rename = "__ID__")]
    #[assoc(is_builtin = true)]
    __ID__,

    #[serde(rename = "__REVISION__")]
    #[assoc(is_builtin = true)]
    __REVISION__,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Assoc)]
#[serde(tag = "type", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn field_type(&self) -> FieldType)]
#[non_exhaustive]
pub enum FieldValue {
    #[assoc(field_type = FieldType::Calc)]
    Calc(String),

    #[assoc(field_type = FieldType::Category)]
    Category(Vec<String>),

    #[assoc(field_type = FieldType::CheckBox)]
    CheckBox(Vec<String>),

    #[assoc(field_type = FieldType::CreatedTime)]
    CreatedTime(DateTime<FixedOffset>),

    #[assoc(field_type = FieldType::Creator)]
    Creator(User),

    #[assoc(field_type = FieldType::Date)]
    Date(Option<NaiveDate>),

    #[assoc(field_type = FieldType::Datetime)]
    DateTime(Option<DateTime<FixedOffset>>),

    #[assoc(field_type = FieldType::DropDown)]
    DropDown(Option<String>),

    #[assoc(field_type = FieldType::File)]
    File(Vec<FileBody>),

    #[assoc(field_type = FieldType::File)]
    GroupSelect(Vec<Group>),

    #[assoc(field_type = FieldType::Link)]
    Link(String),

    #[assoc(field_type = FieldType::Modifier)]
    Modifier(User),

    #[assoc(field_type = FieldType::MultiLineText)]
    MultiLineText(String),

    #[assoc(field_type = FieldType::MultiSelect)]
    MultiSelect(Vec<String>),

    #[assoc(field_type = FieldType::Number)]
    Number(BigDecimal),

    #[assoc(field_type = FieldType::OrganizationSelect)]
    OrganizationSelect(Vec<Organization>),

    #[assoc(field_type = FieldType::RadioButton)]
    RadioButton(Option<String>),

    #[assoc(field_type = FieldType::RecordNumber)]
    RecordNumber(String),

    #[assoc(field_type = FieldType::ReferenceTable)]
    RichText(String),

    #[assoc(field_type = FieldType::SingleLineText)]
    SingleLineText(String),

    #[assoc(field_type = FieldType::Status)]
    Status(String),

    #[assoc(field_type = FieldType::StatusAssignee)]
    StatusAssignee(Vec<User>),

    #[assoc(field_type = FieldType::Subtable)]
    Subtable(Vec<TableRow>),

    #[assoc(field_type = FieldType::Time)]
    Time(NaiveTime),

    #[assoc(field_type = FieldType::UpdatedTime)]
    UpdatedTime(DateTime<FixedOffset>),

    #[assoc(field_type = FieldType::UserSelect)]
    UserSelect(Vec<User>),

    #[serde(rename = "__ID__")]
    #[assoc(field_type = FieldType::__ID__)]
    __ID__(#[serde(with = "stringified")] u64),

    #[serde(rename = "__REVISION__")]
    #[assoc(field_type = FieldType::__REVISION__)]
    __REVISION__(#[serde(with = "stringified")] u64),
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    #[serde(flatten)]
    fields: HashMap<String, FieldValue>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn put_field(
        &mut self,
        field_code: impl Into<String>,
        value: FieldValue,
    ) -> Option<FieldValue> {
        self.fields.insert(field_code.into(), value)
    }

    pub fn get_field_value(&self, field_code: &str) -> Option<&FieldValue> {
        self.fields.get(field_code)
    }
}

impl std::fmt::Debug for TableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("TableRow");

        let mut keys: Vec<_> = self.fields.keys().by_ref().collect();
        keys.sort();

        for key in keys {
            let value = self.fields.get(key).unwrap();
            debug_struct.field(key, value);
        }

        debug_struct.finish()
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

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
pub struct ThreadComment {
    pub text: String,
    pub mentions: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordComment {
    pub text: String,
    pub mentions: Vec<Entity>,
}

impl RecordComment {
    pub fn from_text(s: impl Into<String>) -> Self {
        Self {
            text: s.into(),
            mentions: vec![],
        }
    }
}

impl From<PostedRecordComment> for RecordComment {
    fn from(c: PostedRecordComment) -> Self {
        RecordComment {
            text: c.text,
            mentions: c.mentions,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostedRecordComment {
    pub id: u64,
    pub text: String,
    pub created_at: DateTime<FixedOffset>,
    pub user: User,
    pub mentions: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalcFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub expression: String,
    //pub format: DisplayFormat,
    pub display_scale: i64,
    pub hide_expression: bool,
    pub unit: String,
    //pub unit_position: UnitPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD_JSON1: &str = include_str!("testdata/record1.json");

    fn assert_json_eq(json1: &str, json2: &str) {
        let value1: serde_json::Value = serde_json::from_str(json1).unwrap();
        let value2: serde_json::Value = serde_json::from_str(json2).unwrap();
        assert_eq!(value1, value2);
    }

    #[test]
    fn deserialize_and_serialize_record() {
        let record: Record = serde_json::from_str(RECORD_JSON1).unwrap();
        let serialized = serde_json::to_string_pretty(&record).unwrap();
        assert_json_eq(RECORD_JSON1, &serialized);
    }
}
