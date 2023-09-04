use std::collections::{hash_map, HashMap};

use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::as_str;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let mut fields = HashMap::new();
        for (field_code, field_value) in self.fields() {
            if !field_value.field_type().is_builtin() {
                fields.insert(field_code.clone(), field_value.clone());
            }
        }
        Record { fields }
    }

    pub fn put_field(&mut self, field_code: String, value: FieldValue) -> Option<FieldValue> {
        self.fields.insert(field_code, value)
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
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn field_type(&self) -> FieldType)]
pub enum FieldValue {
    #[assoc(field_type = FieldType::Calc)]
    Calc { value: String },

    #[assoc(field_type = FieldType::Category)]
    Category { value: Vec<String> },

    #[assoc(field_type = FieldType::CheckBox)]
    CheckBox { value: Vec<String> },

    #[assoc(field_type = FieldType::CreatedTime)]
    CreatedTime { value: DateTime<FixedOffset> },

    #[assoc(field_type = FieldType::Creator)]
    Creator { value: User },

    #[assoc(field_type = FieldType::Date)]
    Date { value: Option<NaiveDate> },

    #[assoc(field_type = FieldType::Datetime)]
    DateTime {
        value: Option<DateTime<FixedOffset>>,
    },

    #[assoc(field_type = FieldType::DropDown)]
    DropDown { value: Option<String> },

    #[assoc(field_type = FieldType::File)]
    File { value: Vec<FileBody> },

    #[assoc(field_type = FieldType::File)]
    GroupSelect { value: Vec<Group> },

    #[assoc(field_type = FieldType::Link)]
    Link { value: String },

    #[assoc(field_type = FieldType::Modifier)]
    Modifier { value: User },

    #[assoc(field_type = FieldType::MultiLineText)]
    MultiLineText { value: String },

    #[assoc(field_type = FieldType::MultiSelect)]
    MultiSelect { value: Vec<String> },

    #[assoc(field_type = FieldType::Number)]
    Number { value: BigDecimal },

    #[assoc(field_type = FieldType::OrganizationSelect)]
    OrganizationSelect { value: Vec<Organization> },

    #[assoc(field_type = FieldType::RadioButton)]
    RadioButton { value: Option<String> },

    #[assoc(field_type = FieldType::RecordNumber)]
    RecordNumber { value: String },

    #[assoc(field_type = FieldType::ReferenceTable)]
    RichText { value: String },

    #[assoc(field_type = FieldType::SingleLineText)]
    SingleLineText { value: String },

    #[assoc(field_type = FieldType::Status)]
    Status { value: String },

    #[assoc(field_type = FieldType::StatusAssignee)]
    StatusAssignee { value: Vec<User> },

    #[assoc(field_type = FieldType::Subtable)]
    Subtable { value: Vec<TableRow> },

    #[assoc(field_type = FieldType::Time)]
    Time { value: NaiveTime },

    #[assoc(field_type = FieldType::UpdatedTime)]
    UpdatedTime { value: DateTime<FixedOffset> },

    #[assoc(field_type = FieldType::UserSelect)]
    UserSelect { value: Vec<User> },

    #[serde(rename = "__ID__")]
    #[assoc(field_type = FieldType::__ID__)]
    __ID__ {
        #[serde(with = "as_str")]
        value: u64,
    },

    #[serde(rename = "__REVISION__")]
    #[assoc(field_type = FieldType::__REVISION__)]
    __REVISION__ {
        #[serde(with = "as_str")]
        value: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBody {
    pub content_type: String,
    pub file_key: String,
    pub name: String,
    #[serde(with = "as_str")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn put_field(&mut self, field_code: String, value: FieldValue) -> Option<FieldValue> {
        self.fields.insert(field_code, value)
    }

    pub fn get_field_value(&self, field_code: &str) -> Option<&FieldValue> {
        self.fields.get(field_code)
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
