use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::Entity;
use crate::model::record::FieldType;

/// Represents the configuration properties of a field in a Kintone app.
///
/// Each variant corresponds to a specific field type and contains the complete
/// configuration for that field, including validation rules, display options,
/// and default values.
///
/// This enum is marked as `#[non_exhaustive]` to allow for future field types
/// without breaking changes.
///
/// # Examples
///
/// ```rust
/// use kintone::model::app::field::{FieldProperty, SingleLineTextFieldProperty};
///
/// let text_field = FieldProperty::SingleLineText(SingleLineTextFieldProperty {
///     code: "name".to_string(),
///     label: "Full Name".to_string(),
///     required: true,
///     max_length: Some(100),
///     ..Default::default()
/// });
///
/// // Get the field code
/// assert_eq!(text_field.field_code(), "name");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Assoc)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn field_type(&self) -> FieldType)]
#[func(pub fn field_code(&self) -> &str)]
#[non_exhaustive]
pub enum FieldProperty {
    #[assoc(field_type = FieldType::Calc)]
    #[assoc(field_code = &_0.code)]
    Calc(CalcFieldProperty),
    #[assoc(field_type = FieldType::SingleLineText)]
    #[assoc(field_code = &_0.code)]
    SingleLineText(SingleLineTextFieldProperty),
    #[assoc(field_type = FieldType::MultiLineText)]
    #[assoc(field_code = &_0.code)]
    MultiLineText(MultiLineTextFieldProperty),
    #[assoc(field_type = FieldType::RichText)]
    #[assoc(field_code = &_0.code)]
    RichText(RichTextFieldProperty),
    #[assoc(field_type = FieldType::Number)]
    #[assoc(field_code = &_0.code)]
    Number(NumberFieldProperty),
    #[assoc(field_type = FieldType::Date)]
    #[assoc(field_code = &_0.code)]
    Date(DateFieldProperty),
    #[assoc(field_type = FieldType::Time)]
    #[assoc(field_code = &_0.code)]
    Time(TimeFieldProperty),
    #[assoc(field_type = FieldType::Datetime)]
    #[assoc(field_code = &_0.code)]
    DateTime(DateTimeFieldProperty),
    #[assoc(field_type = FieldType::RadioButton)]
    #[assoc(field_code = &_0.code)]
    RadioButton(RadioButtonFieldProperty),
    #[assoc(field_type = FieldType::CheckBox)]
    #[assoc(field_code = &_0.code)]
    CheckBox(CheckBoxFieldProperty),
    #[assoc(field_type = FieldType::MultiSelect)]
    #[assoc(field_code = &_0.code)]
    MultiSelect(MultiSelectFieldProperty),
    #[assoc(field_type = FieldType::DropDown)]
    #[assoc(field_code = &_0.code)]
    DropDown(DropDownFieldProperty),
    #[assoc(field_type = FieldType::File)]
    #[assoc(field_code = &_0.code)]
    File(FileFieldProperty),
    #[assoc(field_type = FieldType::Link)]
    #[assoc(field_code = &_0.code)]
    Link(LinkFieldProperty),
    #[assoc(field_type = FieldType::UserSelect)]
    #[assoc(field_code = &_0.code)]
    UserSelect(UserSelectFieldProperty),
    #[assoc(field_type = FieldType::OrganizationSelect)]
    #[assoc(field_code = &_0.code)]
    OrganizationSelect(OrganizationSelectFieldProperty),
    #[assoc(field_type = FieldType::GroupSelect)]
    #[assoc(field_code = &_0.code)]
    GroupSelect(GroupSelectFieldProperty),
    #[assoc(field_type = FieldType::ReferenceTable)]
    #[assoc(field_code = &_0.code)]
    ReferenceTable(ReferenceTableFieldProperty),
    #[assoc(field_type = FieldType::Group)]
    #[assoc(field_code = &_0.code)]
    Group(GroupFieldProperty),
    #[assoc(field_type = FieldType::Subtable)]
    #[assoc(field_code = &_0.code)]
    Subtable(SubtableFieldProperty),
    #[assoc(field_type = FieldType::RecordNumber)]
    #[assoc(field_code = &_0.code)]
    RecordNumber(RecordNumberFieldProperty),
    #[assoc(field_type = FieldType::Category)]
    #[assoc(field_code = &_0.code)]
    Category(CategoryFieldProperty),
    #[assoc(field_type = FieldType::Status)]
    #[assoc(field_code = &_0.code)]
    Status(StatusFieldProperty),
    #[assoc(field_type = FieldType::StatusAssignee)]
    #[assoc(field_code = &_0.code)]
    StatusAssignee(StatusAssigneeFieldProperty),
    #[assoc(field_type = FieldType::CreatedTime)]
    #[assoc(field_code = &_0.code)]
    CreatedTime(CreatedTimeFieldProperty),
    #[assoc(field_type = FieldType::UpdatedTime)]
    #[assoc(field_code = &_0.code)]
    UpdatedTime(UpdatedTimeFieldProperty),
    #[assoc(field_type = FieldType::Creator)]
    #[assoc(field_code = &_0.code)]
    Creator(CreatorFieldProperty),
    #[assoc(field_type = FieldType::Modifier)]
    #[assoc(field_code = &_0.code)]
    Modifier(ModifierFieldProperty),
    // Note: Lookup is handled separately in deserialization as it can be applied to various field types
    // and is identified by the presence of a "lookup" property in the JSON
}

// Common types used across field properties

/// Alignment options for field layouts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Alignment {
    /// Horizontal alignment
    Horizontal,
    /// Vertical alignment
    Vertical,
}

/// Position of unit text relative to the input field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitPosition {
    /// Display unit before the value
    Before,
    /// Display unit after the value
    After,
}

/// Display format options for calculated fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayFormat {
    /// Display as a number
    Number,
    /// Display as a number with digit grouping
    NumberDigit,
    /// Display as date and time
    DateTime,
    /// Display as date only
    Date,
    /// Display as time only
    Time,
    /// Display as hour and minute
    HourMinute,
    /// Display as day, hour, and minute
    DayHourMinute,
}

/// Protocol type for link fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkProtocol {
    /// Web URL (http/https)
    Web,
    /// Phone call (tel:)
    Call,
    /// Email address (mailto:)
    Mail,
}

impl Default for LinkProtocol {
    fn default() -> Self {
        Self::Web
    }
}

/// Represents an option in a choice field (radio button, checkbox, dropdown, multi-select).
///
/// Each option has a display label and an index that determines its position
/// in the list of options.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldOption {
    /// Display label for the option
    pub label: String,
    /// Position index of the option
    #[serde(with = "stringified")]
    pub index: u64,
}

// Field Property structs

/// Properties for calculated fields.
///
/// Calculated fields automatically compute values based on an expression
/// that can reference other fields in the same record.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalcFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Calculation expression
    pub expression: String,
    /// Display format for the calculated result
    pub format: Option<DisplayFormat>,
    /// Number of decimal places to display
    pub display_scale: Option<i64>,
    /// Whether to hide the expression from users
    pub hide_expression: bool,
    /// Unit text to display with the value
    pub unit: Option<String>,
    /// Position of the unit text
    pub unit_position: Option<UnitPosition>,
}

/// Properties for single line text fields.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleLineTextFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Whether values must be unique across records
    pub unique: bool,
    /// Maximum allowed length
    #[serde(with = "option_stringified")]
    pub max_length: Option<u64>,
    /// Minimum required length
    #[serde(with = "option_stringified")]
    pub min_length: Option<u64>,
    /// Default value when creating new records
    pub default_value: Option<String>,
    /// Auto-calculation expression
    pub expression: Option<String>,
    /// Whether to hide the expression from users
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

// From trait implementations for ergonomic FieldProperty creation

impl From<CalcFieldProperty> for FieldProperty {
    fn from(property: CalcFieldProperty) -> Self {
        FieldProperty::Calc(property)
    }
}

impl From<SingleLineTextFieldProperty> for FieldProperty {
    fn from(property: SingleLineTextFieldProperty) -> Self {
        FieldProperty::SingleLineText(property)
    }
}

impl From<MultiLineTextFieldProperty> for FieldProperty {
    fn from(property: MultiLineTextFieldProperty) -> Self {
        FieldProperty::MultiLineText(property)
    }
}

impl From<RichTextFieldProperty> for FieldProperty {
    fn from(property: RichTextFieldProperty) -> Self {
        FieldProperty::RichText(property)
    }
}

impl From<NumberFieldProperty> for FieldProperty {
    fn from(property: NumberFieldProperty) -> Self {
        FieldProperty::Number(property)
    }
}

impl From<DateFieldProperty> for FieldProperty {
    fn from(property: DateFieldProperty) -> Self {
        FieldProperty::Date(property)
    }
}

impl From<TimeFieldProperty> for FieldProperty {
    fn from(property: TimeFieldProperty) -> Self {
        FieldProperty::Time(property)
    }
}

impl From<DateTimeFieldProperty> for FieldProperty {
    fn from(property: DateTimeFieldProperty) -> Self {
        FieldProperty::DateTime(property)
    }
}

impl From<RadioButtonFieldProperty> for FieldProperty {
    fn from(property: RadioButtonFieldProperty) -> Self {
        FieldProperty::RadioButton(property)
    }
}

impl From<CheckBoxFieldProperty> for FieldProperty {
    fn from(property: CheckBoxFieldProperty) -> Self {
        FieldProperty::CheckBox(property)
    }
}

impl From<MultiSelectFieldProperty> for FieldProperty {
    fn from(property: MultiSelectFieldProperty) -> Self {
        FieldProperty::MultiSelect(property)
    }
}

impl From<DropDownFieldProperty> for FieldProperty {
    fn from(property: DropDownFieldProperty) -> Self {
        FieldProperty::DropDown(property)
    }
}

impl From<FileFieldProperty> for FieldProperty {
    fn from(property: FileFieldProperty) -> Self {
        FieldProperty::File(property)
    }
}

impl From<LinkFieldProperty> for FieldProperty {
    fn from(property: LinkFieldProperty) -> Self {
        FieldProperty::Link(property)
    }
}

impl From<UserSelectFieldProperty> for FieldProperty {
    fn from(property: UserSelectFieldProperty) -> Self {
        FieldProperty::UserSelect(property)
    }
}

impl From<OrganizationSelectFieldProperty> for FieldProperty {
    fn from(property: OrganizationSelectFieldProperty) -> Self {
        FieldProperty::OrganizationSelect(property)
    }
}

impl From<GroupSelectFieldProperty> for FieldProperty {
    fn from(property: GroupSelectFieldProperty) -> Self {
        FieldProperty::GroupSelect(property)
    }
}

impl From<ReferenceTableFieldProperty> for FieldProperty {
    fn from(property: ReferenceTableFieldProperty) -> Self {
        FieldProperty::ReferenceTable(property)
    }
}

impl From<GroupFieldProperty> for FieldProperty {
    fn from(property: GroupFieldProperty) -> Self {
        FieldProperty::Group(property)
    }
}

impl From<SubtableFieldProperty> for FieldProperty {
    fn from(property: SubtableFieldProperty) -> Self {
        FieldProperty::Subtable(property)
    }
}

impl From<RecordNumberFieldProperty> for FieldProperty {
    fn from(property: RecordNumberFieldProperty) -> Self {
        FieldProperty::RecordNumber(property)
    }
}

impl From<CategoryFieldProperty> for FieldProperty {
    fn from(property: CategoryFieldProperty) -> Self {
        FieldProperty::Category(property)
    }
}

impl From<StatusFieldProperty> for FieldProperty {
    fn from(property: StatusFieldProperty) -> Self {
        FieldProperty::Status(property)
    }
}

impl From<StatusAssigneeFieldProperty> for FieldProperty {
    fn from(property: StatusAssigneeFieldProperty) -> Self {
        FieldProperty::StatusAssignee(property)
    }
}

impl From<CreatedTimeFieldProperty> for FieldProperty {
    fn from(property: CreatedTimeFieldProperty) -> Self {
        FieldProperty::CreatedTime(property)
    }
}

impl From<UpdatedTimeFieldProperty> for FieldProperty {
    fn from(property: UpdatedTimeFieldProperty) -> Self {
        FieldProperty::UpdatedTime(property)
    }
}

impl From<CreatorFieldProperty> for FieldProperty {
    fn from(property: CreatorFieldProperty) -> Self {
        FieldProperty::Creator(property)
    }
}

impl From<ModifierFieldProperty> for FieldProperty {
    fn from(property: ModifierFieldProperty) -> Self {
        FieldProperty::Modifier(property)
    }
}
