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

/// Properties for multi-line text fields.
///
/// Multi-line text fields allow users to enter longer text content with line breaks.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiLineTextFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default value when creating new records
    pub default_value: Option<String>,
}

/// Properties for rich text fields.
///
/// Rich text fields support formatted text with styling options like bold, italic,
/// links, and other HTML formatting.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichTextFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default value when creating new records
    pub default_value: Option<String>,
}

/// Properties for number fields.
///
/// Number fields store numeric values with configurable validation rules,
/// display formats, and decimal precision settings.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldProperty {
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
    /// Maximum allowed value
    pub max_value: Option<BigDecimal>,
    /// Minimum allowed value
    pub min_value: Option<BigDecimal>,
    /// Default value when creating new records
    pub default_value: Option<BigDecimal>,
    /// Whether to display numbers with digit grouping (e.g., 1,000)
    pub digit: bool,
    /// Number of decimal places to display
    #[serde(with = "option_stringified")]
    pub display_scale: Option<u64>,
    /// Unit text to display with the value
    pub unit: Option<String>,
    /// Position of the unit text
    pub unit_position: Option<UnitPosition>,
}

/// Properties for date fields.
///
/// Date fields store date values without time information.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFieldProperty {
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
    /// Default date value when creating new records
    pub default_value: Option<NaiveDate>,
    /// Whether to use the current date as default
    pub default_now_value: bool,
}

/// Properties for time fields.
///
/// Time fields store time values without date information.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default time value when creating new records
    pub default_value: Option<NaiveTime>,
    /// Whether to use the current time as default
    pub default_now_value: bool,
}

/// Properties for date-time fields.
///
/// Date-time fields store both date and time information with timezone support.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeFieldProperty {
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
    /// Default date-time value when creating new records
    pub default_value: Option<DateTime<FixedOffset>>,
    /// Whether to use the current date-time as default
    pub default_now_value: bool,
}

/// Properties for radio button fields.
///
/// Radio button fields allow users to select a single option from a predefined list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioButtonFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected option
    pub default_value: Option<String>,
    /// Layout alignment of the options
    pub align: Option<Alignment>,
    /// Available options mapped by their values
    pub options: HashMap<String, FieldOption>,
}

/// Properties for checkbox fields.
///
/// Checkbox fields allow users to select multiple options from a predefined list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckBoxFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected options
    pub default_value: Vec<String>,
    /// Layout alignment of the options
    pub align: Option<Alignment>,
    /// Available options mapped by their values
    pub options: HashMap<String, FieldOption>,
}

/// Properties for multi-select fields.
///
/// Multi-select fields allow users to select multiple options from a dropdown list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelectFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected options
    pub default_value: Vec<String>,
    /// Available options mapped by their values
    pub options: HashMap<String, FieldOption>,
}

/// Properties for dropdown fields.
///
/// Dropdown fields allow users to select a single option from a dropdown list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropDownFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected option
    pub default_value: Option<String>,
    /// Available options mapped by their values
    pub options: HashMap<String, FieldOption>,
}

/// Properties for file attachment fields.
///
/// File fields allow users to upload and attach files to records.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Size of thumbnail images in pixels
    #[serde(with = "option_stringified")]
    pub thumbnail_size: Option<u64>,
}

/// Properties for link fields.
///
/// Link fields store URL or other link information with protocol validation.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkFieldProperty {
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
    /// Default link value when creating new records
    pub default_value: Option<String>,
    /// Maximum allowed length
    #[serde(with = "option_stringified")]
    pub max_length: Option<u64>,
    /// Minimum required length
    #[serde(with = "option_stringified")]
    pub min_length: Option<u64>,
    /// Protocol type for the link
    pub protocol: LinkProtocol,
}

/// Properties for user selection fields.
///
/// User selection fields allow selection of users from the Kintone organization.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSelectFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected users
    pub default_value: Vec<Entity>,
    /// Available users that can be selected
    pub entities: Vec<Entity>,
}

/// Properties for organization selection fields.
///
/// Organization selection fields allow selection of organizations from the Kintone system.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationSelectFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected organizations
    pub default_value: Vec<Entity>,
    /// Available organizations that can be selected
    pub entities: Vec<Entity>,
}

/// Properties for group selection fields.
///
/// Group selection fields allow selection of groups from the Kintone organization.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSelectFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Whether the field is required
    pub required: bool,
    /// Default selected groups
    pub default_value: Vec<Entity>,
    /// Available groups that can be selected
    pub entities: Vec<Entity>,
}

/// Properties for reference table fields.
///
/// Reference table fields display data from related apps based on specified conditions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceTableFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
    /// Configuration for the referenced table
    pub reference_table: ReferenceTable,
}

/// Configuration for reference table relationships.
///
/// Defines how records from a related app are filtered and displayed.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceTable {
    /// Information about the related app
    pub related_app: RelatedApp,
    /// Condition that relates records between apps
    pub condition: ReferenceCondition,
    /// Additional filter condition for the referenced records
    pub filter_cond: Option<String>,
    /// Fields to display from the referenced app
    pub display_fields: Vec<String>,
    /// Sort order for the referenced records
    pub sort: Option<String>,
    /// Maximum number of records to display
    #[serde(with = "option_stringified")]
    pub size: Option<u64>,
}

/// Information about a related app.
///
/// Specifies which app to reference either by ID or by app code.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedApp {
    /// App ID of the related app
    #[serde(with = "option_stringified")]
    pub app: Option<u64>,
    /// App code of the related app (alternative to app ID)
    pub code: Option<String>,
}

/// Condition that relates records between apps.
///
/// Defines which fields are used to match records between the current app and the related app.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceCondition {
    /// Field code in the current app
    pub field: String,
    /// Field code in the related app
    pub related_field: String,
}

/// Configuration for lookup field functionality.
///
/// Lookup settings define how field values are automatically retrieved from related apps.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupSetting {
    /// Information about the related app
    pub related_app: RelatedApp,
    /// Key field in the related app used for lookup
    pub related_key_field: String,
    /// Field mappings between current app and related app
    pub field_mappings: Vec<FieldMapping>,
    /// Fields displayed in the lookup picker
    pub lookup_picker_fields: Vec<String>,
    /// Filter condition for lookup records
    pub filter_cond: Option<String>,
    /// Sort order for lookup records
    pub sort: Option<String>,
}

/// Mapping between fields in different apps.
///
/// Defines how a field in the current app maps to a field in the related app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldMapping {
    /// Field code in the current app
    pub field: String,
    /// Field code in the related app
    pub related_field: String,
}

/// Properties for group fields.
///
/// Group fields are used to visually organize other fields together in the form layout.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label for the group
    pub label: String,
    /// Whether to hide the group label
    pub no_label: bool,
    /// Whether the group should be expanded by default
    pub open_group: bool,
}

/// Properties for subtable fields.
///
/// Subtable fields contain multiple rows of data, where each row can have multiple fields.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtableFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label for the subtable
    pub label: String,
    /// Whether to hide the subtable label
    pub no_label: bool,
    /// Fields that make up each row in the subtable
    pub fields: HashMap<String, FieldProperty>,
}

/// Properties for lookup fields.
///
/// Lookup fields automatically retrieve values from related apps based on a key field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// The underlying field type for the lookup field
    #[serde(rename = "type")]
    pub field_type: FieldType,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: Option<bool>,
    /// Whether the field is required
    pub required: Option<bool>,
    /// Lookup configuration settings
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

/// Properties for record number fields.
///
/// Record number fields automatically generate unique identifiers for each record.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordNumberFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
}

/// Properties for category fields.
///
/// Category fields are used for organizing records in views and reports.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether the category field is enabled
    pub enabled: bool,
}

/// Properties for workflow status fields.
///
/// Status fields track the current state of records in a workflow process.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether the status field is enabled
    pub enabled: bool,
}

/// Properties for status assignee fields.
///
/// Status assignee fields track who is currently assigned to handle a record in a workflow.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusAssigneeFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether the assignee field is enabled
    pub enabled: bool,
}

/// Properties for record creation time fields.
///
/// Created time fields automatically store when each record was created.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedTimeFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
}

/// Properties for record update time fields.
///
/// Updated time fields automatically store when each record was last modified.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedTimeFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
}

/// Properties for record creator fields.
///
/// Creator fields automatically store information about who created each record.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
    pub no_label: bool,
}

/// Properties for record modifier fields.
///
/// Modifier fields automatically store information about who last modified each record.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifierFieldProperty {
    /// Field code (unique identifier)
    pub code: String,
    /// Display label
    pub label: String,
    /// Whether to hide the field label
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
