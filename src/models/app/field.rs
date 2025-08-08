use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::internal::serde_helper::{option_stringified, stringified};
use crate::models::Entity;
use crate::models::record::FieldType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Assoc)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn field_type(&self) -> FieldType)]
#[non_exhaustive]
pub enum FieldProperty {
    #[assoc(field_type = FieldType::Calc)]
    Calc(CalcFieldProperty),
    #[assoc(field_type = FieldType::SingleLineText)]
    SingleLineText(SingleLineTextFieldProperty),
    #[assoc(field_type = FieldType::MultiLineText)]
    MultiLineText(MultiLineTextFieldProperty),
    #[assoc(field_type = FieldType::RichText)]
    RichText(RichTextFieldProperty),
    #[assoc(field_type = FieldType::Number)]
    Number(NumberFieldProperty),
    #[assoc(field_type = FieldType::Date)]
    Date(DateFieldProperty),
    #[assoc(field_type = FieldType::Time)]
    Time(TimeFieldProperty),
    #[assoc(field_type = FieldType::Datetime)]
    DateTime(DateTimeFieldProperty),
    #[assoc(field_type = FieldType::RadioButton)]
    RadioButton(RadioButtonFieldProperty),
    #[assoc(field_type = FieldType::CheckBox)]
    CheckBox(CheckBoxFieldProperty),
    #[assoc(field_type = FieldType::MultiSelect)]
    MultiSelect(MultiSelectFieldProperty),
    #[assoc(field_type = FieldType::DropDown)]
    DropDown(DropDownFieldProperty),
    #[assoc(field_type = FieldType::File)]
    File(FileFieldProperty),
    #[assoc(field_type = FieldType::Link)]
    Link(LinkFieldProperty),
    #[assoc(field_type = FieldType::UserSelect)]
    UserSelect(UserSelectFieldProperty),
    #[assoc(field_type = FieldType::OrganizationSelect)]
    OrganizationSelect(OrganizationSelectFieldProperty),
    #[assoc(field_type = FieldType::GroupSelect)]
    GroupSelect(GroupSelectFieldProperty),
    #[assoc(field_type = FieldType::ReferenceTable)]
    ReferenceTable(ReferenceTableFieldProperty),
    #[assoc(field_type = FieldType::Group)]
    Group(GroupFieldProperty),
    #[assoc(field_type = FieldType::Subtable)]
    Subtable(SubtableFieldProperty),
    #[assoc(field_type = FieldType::RecordNumber)]
    RecordNumber(RecordNumberFieldProperty),
    #[assoc(field_type = FieldType::Category)]
    Category(CategoryFieldProperty),
    #[assoc(field_type = FieldType::Status)]
    Status(StatusFieldProperty),
    #[assoc(field_type = FieldType::StatusAssignee)]
    StatusAssignee(StatusAssigneeFieldProperty),
    #[assoc(field_type = FieldType::CreatedTime)]
    CreatedTime(CreatedTimeFieldProperty),
    #[assoc(field_type = FieldType::UpdatedTime)]
    UpdatedTime(UpdatedTimeFieldProperty),
    #[assoc(field_type = FieldType::Creator)]
    Creator(CreatorFieldProperty),
    #[assoc(field_type = FieldType::Modifier)]
    Modifier(ModifierFieldProperty),
    // Note: Lookup is handled separately in deserialization as it can be applied to various field types
    // and is identified by the presence of a "lookup" property in the JSON
}

// Common types used across field properties
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Alignment {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitPosition {
    Before,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayFormat {
    Number,
    NumberDigit,
    DateTime,
    Date,
    Time,
    HourMinute,
    DayHourMinute,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkProtocol {
    Web,
    Call,
    Mail,
}

impl Default for LinkProtocol {
    fn default() -> Self {
        Self::Web
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldOption {
    pub label: String,
    #[serde(with = "stringified")]
    pub index: u64,
}

// Field Property structs

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalcFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub expression: String,
    pub format: Option<DisplayFormat>,
    pub display_scale: Option<i64>,
    pub hide_expression: bool,
    pub unit: Option<String>,
    pub unit_position: Option<UnitPosition>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleLineTextFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub unique: bool,
    #[serde(with = "option_stringified")]
    pub max_length: Option<u64>,
    #[serde(with = "option_stringified")]
    pub min_length: Option<u64>,
    pub default_value: Option<String>,
    pub expression: Option<String>,
    pub hide_expression: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiLineTextFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichTextFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub unique: bool,
    pub max_value: Option<BigDecimal>,
    pub min_value: Option<BigDecimal>,
    pub default_value: Option<BigDecimal>,
    pub digit: bool,
    #[serde(with = "option_stringified")]
    pub display_scale: Option<u64>,
    pub unit: Option<String>,
    pub unit_position: Option<UnitPosition>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub unique: bool,
    pub default_value: Option<NaiveDate>,
    pub default_now_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Option<NaiveTime>,
    pub default_now_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub unique: bool,
    pub default_value: Option<DateTime<FixedOffset>>,
    pub default_now_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioButtonFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Option<String>,
    pub align: Option<Alignment>,
    pub options: HashMap<String, FieldOption>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckBoxFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Vec<String>,
    pub align: Option<Alignment>,
    pub options: HashMap<String, FieldOption>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelectFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Vec<String>,
    pub options: HashMap<String, FieldOption>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropDownFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Option<String>,
    pub options: HashMap<String, FieldOption>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    #[serde(with = "option_stringified")]
    pub thumbnail_size: Option<u64>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub unique: bool,
    pub default_value: Option<String>,
    #[serde(with = "option_stringified")]
    pub max_length: Option<u64>,
    #[serde(with = "option_stringified")]
    pub min_length: Option<u64>,
    pub protocol: LinkProtocol,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSelectFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Vec<Entity>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationSelectFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Vec<Entity>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSelectFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub default_value: Vec<Entity>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceTableFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub reference_table: ReferenceTable,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceTable {
    pub related_app: RelatedApp,
    pub condition: ReferenceCondition,
    pub filter_cond: Option<String>,
    pub display_fields: Vec<String>,
    pub sort: Option<String>,
    #[serde(with = "option_stringified")]
    pub size: Option<u64>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedApp {
    #[serde(with = "option_stringified")]
    pub app: Option<u64>,
    pub code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceCondition {
    pub field: String,
    pub related_field: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupSetting {
    pub related_app: RelatedApp,
    pub related_key_field: String,
    pub field_mappings: Vec<FieldMapping>,
    pub lookup_picker_fields: Vec<String>,
    pub filter_cond: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldMapping {
    pub field: String,
    pub related_field: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub open_group: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtableFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub fields: HashMap<String, FieldProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupFieldProperty {
    pub code: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub label: String,
    pub no_label: Option<bool>,
    pub required: Option<bool>,
    pub lookup: LookupSetting,
}

impl Default for LookupFieldProperty {
    fn default() -> Self {
        Self {
            code: String::default(),
            field_type: FieldType::SingleLineText,
            label: String::default(),
            no_label: None,
            required: None,
            lookup: LookupSetting::default(),
        }
    }
}

// System field properties (read-only)

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordNumberFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryFieldProperty {
    pub code: String,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusFieldProperty {
    pub code: String,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusAssigneeFieldProperty {
    pub code: String,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedTimeFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedTimeFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifierFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
}
