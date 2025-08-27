//! # Kintone Field Models
//!
//! This module provides comprehensive type definitions for all supported Kintone field types,
//! including their configuration properties, validation rules, and display options.
//!
//! # Basic Usage
//!
//! Create field configurations using the builder pattern (recommended):
//!
//! ```rust
//! use kintone::model::app::field::{
//!     single_line_text_field_property, number_field_property, UnitPosition
//! };
//!
//! // Text field with validation
//! let name_field = single_line_text_field_property("name")
//!     .label("Full Name")
//!     .required(true)
//!     .max_length(50)
//!     .build();
//!
//! // Number field with currency formatting
//! let price_field = number_field_property("price")
//!     .label("Price")
//!     .required(true)
//!     .unit("€")
//!     .unit_position(UnitPosition::After)
//!     .display_scale(2)
//!     .build();
//! ```
//!
//! # Alternative: Direct Struct Initialization
//!
//! You can also create fields using direct struct initialization:
//!
//! ```rust
//! use kintone::model::app::field::{
//!     SingleLineTextFieldProperty, NumberFieldProperty, UnitPosition
//! };
//!
//! // Text field with validation
//! let name_field = SingleLineTextFieldProperty {
//!     code: "name".to_string(),
//!     label: "Full Name".to_string(),
//!     required: true,
//!     max_length: Some(50),
//!     ..Default::default()
//! };
//!
//! // Number field with currency formatting
//! let price_field = NumberFieldProperty {
//!     code: "price".to_string(),
//!     label: "Price".to_string(),
//!     required: true,
//!     unit: Some("€".to_string()),
//!     unit_position: Some(UnitPosition::After),
//!     display_scale: Some(2),
//!     ..Default::default()
//! };
//! ```
//!
//! # Builder Pattern Usage
//!
//! For more ergonomic field creation, use the provided builder functions:
//!
//! ```rust
//! use kintone::model::app::field::{
//!     single_line_text_field_property, number_field_property,
//!     date_field_property, radio_button_field_property,
//!     created_time_field_property, status_field_property
//! };
//! use kintone::model::app::field::{FieldOption, Alignment, UnitPosition};
//! use bigdecimal::BigDecimal;
//! use std::collections::HashMap;
//!
//! // Text field with builder pattern
//! let name_field = single_line_text_field_property("name")
//!     .label("Full Name")
//!     .required(true)
//!     .max_length(100)
//!     .min_length(1)
//!     .build();
//!
//! // Number field with currency formatting
//! let price_field = number_field_property("price")
//!     .label("Price")
//!     .required(true)
//!     .unit("USD")
//!     .unit_position(UnitPosition::After)
//!     .display_scale(2)
//!     .digit(true)
//!     .build();
//!
//! // Date field with default
//! let start_date = date_field_property("start_date")
//!     .label("Start Date")
//!     .required(true)
//!     .default_now_value(true)
//!     .build();
//!
//! // Radio button field with options
//! let mut options = HashMap::new();
//! options.insert("high".to_string(), FieldOption { label: "High".to_string(), index: 0 });
//! options.insert("medium".to_string(), FieldOption { label: "Medium".to_string(), index: 1 });
//! options.insert("low".to_string(), FieldOption { label: "Low".to_string(), index: 2 });
//!
//! let priority_field = radio_button_field_property("priority")
//!     .label("Priority")
//!     .required(true)
//!     .default_value("medium")
//!     .align(Alignment::Horizontal)
//!     .options(options)
//!     .build();
//!
//! // System fields
//! let created_time = created_time_field_property("created_time")
//!     .label("Created Time")
//!     .build();
//!
//! let status = status_field_property("status")
//!     .label("Status")
//!     .enabled(true)
//!     .build();
//! ```
//!
//! # Type Conversions with `.into()`
//!
//! Instead of writing verbose enum variant names, you can use `.into()` to convert
//! specific field property types to the generic [`FieldProperty`] enum:
//!
//! ```rust
//! use kintone::model::app::field::{single_line_text_field_property, number_field_property, FieldProperty};
//!
//! let text_field = single_line_text_field_property("description")
//!     .label("Description")
//!     .build();
//!
//! // Concise conversion to FieldProperty
//! let field_property: FieldProperty = text_field.into();
//!
//! // Use in collections
//! let fields: Vec<FieldProperty> = vec![
//!     single_line_text_field_property("title")
//!         .label("Title")
//!         .build()
//!         .into(),
//!     number_field_property("amount")
//!         .label("Amount")
//!         .build()
//!         .into(),
//! ];
//! ```
//!
//! # Field Properties Access
//!
//! All field types implement common methods to access their properties:
//!
//! ```rust
//! use kintone::model::app::field::{single_line_text_field_property, FieldProperty};
//!
//! let field: FieldProperty = single_line_text_field_property("sample")
//!     .label("Sample Field")
//!     .build()
//!     .into();
//!
//! // Access common properties
//! assert_eq!(field.field_code(), "sample");
//! println!("Field type: {:?}", field.field_type());
//! ```

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
/// use kintone::model::app::field::{single_line_text_field_property, FieldProperty};
///
/// let text_field: FieldProperty = single_line_text_field_property("name")
///     .label("Full Name")
///     .required(true)
///     .max_length(100)
///     .build()
///     .into();
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

// Builder functions for field properties

/// Creates a new calculated field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{calc_field_property, DisplayFormat, UnitPosition};
///
/// let field = calc_field_property("total_price")
///     .label("Total Price")
///     .required(true)
///     .expression("price * quantity")
///     .format(DisplayFormat::Number)
///     .display_scale(2)
///     .unit("USD")
///     .unit_position(UnitPosition::After)
///     .build();
/// ```
pub fn calc_field_property(code: impl Into<String>) -> CalcFieldPropertyBuilder {
    CalcFieldPropertyBuilder {
        property: CalcFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            expression: String::new(),
            format: None,
            display_scale: None,
            hide_expression: false,
            unit: None,
            unit_position: None,
        },
    }
}

/// Builder for creating [`CalcFieldProperty`].
#[derive(Debug, Clone)]
pub struct CalcFieldPropertyBuilder {
    property: CalcFieldProperty,
}

impl CalcFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the calculation expression.
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.property.expression = expression.into();
        self
    }

    /// Sets the display format for the calculated result.
    pub fn format(mut self, format: DisplayFormat) -> Self {
        self.property.format = Some(format);
        self
    }

    /// Sets the number of decimal places to display.
    pub fn display_scale(mut self, display_scale: i64) -> Self {
        self.property.display_scale = Some(display_scale);
        self
    }

    /// Sets whether to hide the expression from users.
    pub fn hide_expression(mut self, hide_expression: bool) -> Self {
        self.property.hide_expression = hide_expression;
        self
    }

    /// Sets the unit text to display with the value.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.property.unit = Some(unit.into());
        self
    }

    /// Sets the position of the unit text.
    pub fn unit_position(mut self, unit_position: UnitPosition) -> Self {
        self.property.unit_position = Some(unit_position);
        self
    }

    /// Builds the final [`CalcFieldProperty`].
    pub fn build(self) -> CalcFieldProperty {
        self.property
    }
}

impl From<CalcFieldPropertyBuilder> for CalcFieldProperty {
    fn from(builder: CalcFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new single line text field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::single_line_text_field_property;
///
/// let field = single_line_text_field_property("name")
///     .label("Full Name")
///     .required(true)
///     .unique(true)
///     .max_length(100)
///     .min_length(1)
///     .default_value("John Doe")
///     .build();
/// ```
pub fn single_line_text_field_property(
    code: impl Into<String>,
) -> SingleLineTextFieldPropertyBuilder {
    SingleLineTextFieldPropertyBuilder {
        property: SingleLineTextFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            unique: false,
            max_length: None,
            min_length: None,
            default_value: None,
            expression: None,
            hide_expression: false,
        },
    }
}

/// Builder for creating [`SingleLineTextFieldProperty`].
#[derive(Debug, Clone)]
pub struct SingleLineTextFieldPropertyBuilder {
    property: SingleLineTextFieldProperty,
}

impl SingleLineTextFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets whether values must be unique across records.
    pub fn unique(mut self, unique: bool) -> Self {
        self.property.unique = unique;
        self
    }

    /// Sets the maximum allowed length.
    pub fn max_length(mut self, max_length: u64) -> Self {
        self.property.max_length = Some(max_length);
        self
    }

    /// Sets the minimum required length.
    pub fn min_length(mut self, min_length: u64) -> Self {
        self.property.min_length = Some(min_length);
        self
    }

    /// Sets the default value when creating new records.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Sets the auto-calculation expression.
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.property.expression = Some(expression.into());
        self
    }

    /// Sets whether to hide the expression from users.
    pub fn hide_expression(mut self, hide_expression: bool) -> Self {
        self.property.hide_expression = hide_expression;
        self
    }

    /// Builds the final [`SingleLineTextFieldProperty`].
    pub fn build(self) -> SingleLineTextFieldProperty {
        self.property
    }
}

impl From<SingleLineTextFieldPropertyBuilder> for SingleLineTextFieldProperty {
    fn from(builder: SingleLineTextFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new number field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{number_field_property, UnitPosition};
/// use bigdecimal::BigDecimal;
///
/// let field = number_field_property("price")
///     .label("Price")
///     .required(true)
///     .unique(false)
///     .max_value(BigDecimal::from(10000))
///     .min_value(BigDecimal::from(0))
///     .default_value(BigDecimal::from(0))
///     .digit(true)
///     .display_scale(2)
///     .unit("USD")
///     .unit_position(UnitPosition::After)
///     .build();
/// ```
pub fn number_field_property(code: impl Into<String>) -> NumberFieldPropertyBuilder {
    NumberFieldPropertyBuilder {
        property: NumberFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            unique: false,
            max_value: None,
            min_value: None,
            default_value: None,
            digit: false,
            display_scale: None,
            unit: None,
            unit_position: None,
        },
    }
}

/// Builder for creating [`NumberFieldProperty`].
#[derive(Debug, Clone)]
pub struct NumberFieldPropertyBuilder {
    property: NumberFieldProperty,
}

impl NumberFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets whether values must be unique across records.
    pub fn unique(mut self, unique: bool) -> Self {
        self.property.unique = unique;
        self
    }

    /// Sets the maximum allowed value.
    pub fn max_value(mut self, max_value: BigDecimal) -> Self {
        self.property.max_value = Some(max_value);
        self
    }

    /// Sets the minimum allowed value.
    pub fn min_value(mut self, min_value: BigDecimal) -> Self {
        self.property.min_value = Some(min_value);
        self
    }

    /// Sets the default value when creating new records.
    pub fn default_value(mut self, default_value: BigDecimal) -> Self {
        self.property.default_value = Some(default_value);
        self
    }

    /// Sets whether to display numbers with digit grouping (e.g., 1,000).
    pub fn digit(mut self, digit: bool) -> Self {
        self.property.digit = digit;
        self
    }

    /// Sets the number of decimal places to display.
    pub fn display_scale(mut self, display_scale: u64) -> Self {
        self.property.display_scale = Some(display_scale);
        self
    }

    /// Sets the unit text to display with the value.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.property.unit = Some(unit.into());
        self
    }

    /// Sets the position of the unit text.
    pub fn unit_position(mut self, unit_position: UnitPosition) -> Self {
        self.property.unit_position = Some(unit_position);
        self
    }

    /// Builds the final [`NumberFieldProperty`].
    pub fn build(self) -> NumberFieldProperty {
        self.property
    }
}

impl From<NumberFieldPropertyBuilder> for NumberFieldProperty {
    fn from(builder: NumberFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new multi-line text field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::multi_line_text_field_property;
///
/// let field = multi_line_text_field_property("description")
///     .label("Description")
///     .required(true)
///     .default_value("Enter description here...")
///     .build();
/// ```
pub fn multi_line_text_field_property(
    code: impl Into<String>,
) -> MultiLineTextFieldPropertyBuilder {
    MultiLineTextFieldPropertyBuilder {
        property: MultiLineTextFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: None,
        },
    }
}

/// Builder for creating [`MultiLineTextFieldProperty`].
#[derive(Debug, Clone)]
pub struct MultiLineTextFieldPropertyBuilder {
    property: MultiLineTextFieldProperty,
}

impl MultiLineTextFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default value when creating new records.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Builds the final [`MultiLineTextFieldProperty`].
    pub fn build(self) -> MultiLineTextFieldProperty {
        self.property
    }
}

impl From<MultiLineTextFieldPropertyBuilder> for MultiLineTextFieldProperty {
    fn from(builder: MultiLineTextFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new date field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::date_field_property;
/// use chrono::NaiveDate;
///
/// let field = date_field_property("start_date")
///     .label("Start Date")
///     .required(true)
///     .unique(false)
///     .default_value(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
///     .build();
/// ```
pub fn date_field_property(code: impl Into<String>) -> DateFieldPropertyBuilder {
    DateFieldPropertyBuilder {
        property: DateFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            unique: false,
            default_value: None,
            default_now_value: false,
        },
    }
}

/// Builder for creating [`DateFieldProperty`].
#[derive(Debug, Clone)]
pub struct DateFieldPropertyBuilder {
    property: DateFieldProperty,
}

impl DateFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets whether values must be unique across records.
    pub fn unique(mut self, unique: bool) -> Self {
        self.property.unique = unique;
        self
    }

    /// Sets the default date value when creating new records.
    pub fn default_value(mut self, default_value: NaiveDate) -> Self {
        self.property.default_value = Some(default_value);
        self
    }

    /// Sets whether to use the current date as default.
    pub fn default_now_value(mut self, default_now_value: bool) -> Self {
        self.property.default_now_value = default_now_value;
        self
    }

    /// Builds the final [`DateFieldProperty`].
    pub fn build(self) -> DateFieldProperty {
        self.property
    }
}

impl From<DateFieldPropertyBuilder> for DateFieldProperty {
    fn from(builder: DateFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new rich text field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::rich_text_field_property;
///
/// let field = rich_text_field_property("content")
///     .label("Content")
///     .required(true)
///     .default_value("<p>Enter content here...</p>")
///     .build();
/// ```
pub fn rich_text_field_property(code: impl Into<String>) -> RichTextFieldPropertyBuilder {
    RichTextFieldPropertyBuilder {
        property: RichTextFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: None,
        },
    }
}

/// Builder for creating [`RichTextFieldProperty`].
#[derive(Debug, Clone)]
pub struct RichTextFieldPropertyBuilder {
    property: RichTextFieldProperty,
}

impl RichTextFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default value when creating new records.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Builds the final [`RichTextFieldProperty`].
    pub fn build(self) -> RichTextFieldProperty {
        self.property
    }
}

impl From<RichTextFieldPropertyBuilder> for RichTextFieldProperty {
    fn from(builder: RichTextFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new time field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::time_field_property;
/// use chrono::NaiveTime;
///
/// let field = time_field_property("meeting_time")
///     .label("Meeting Time")
///     .required(true)
///     .default_value(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
///     .build();
/// ```
pub fn time_field_property(code: impl Into<String>) -> TimeFieldPropertyBuilder {
    TimeFieldPropertyBuilder {
        property: TimeFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: None,
            default_now_value: false,
        },
    }
}

/// Builder for creating [`TimeFieldProperty`].
#[derive(Debug, Clone)]
pub struct TimeFieldPropertyBuilder {
    property: TimeFieldProperty,
}

impl TimeFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default time value when creating new records.
    pub fn default_value(mut self, default_value: NaiveTime) -> Self {
        self.property.default_value = Some(default_value);
        self
    }

    /// Sets whether to use the current time as default.
    pub fn default_now_value(mut self, default_now_value: bool) -> Self {
        self.property.default_now_value = default_now_value;
        self
    }

    /// Builds the final [`TimeFieldProperty`].
    pub fn build(self) -> TimeFieldProperty {
        self.property
    }
}

impl From<TimeFieldPropertyBuilder> for TimeFieldProperty {
    fn from(builder: TimeFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new date-time field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::date_time_field_property;
/// use chrono::{DateTime, FixedOffset, NaiveDateTime};
///
/// let field = date_time_field_property("created_at")
///     .label("Created At")
///     .required(true)
///     .unique(false)
///     .default_now_value(true)
///     .build();
/// ```
pub fn date_time_field_property(code: impl Into<String>) -> DateTimeFieldPropertyBuilder {
    DateTimeFieldPropertyBuilder {
        property: DateTimeFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            unique: false,
            default_value: None,
            default_now_value: false,
        },
    }
}

/// Builder for creating [`DateTimeFieldProperty`].
#[derive(Debug, Clone)]
pub struct DateTimeFieldPropertyBuilder {
    property: DateTimeFieldProperty,
}

impl DateTimeFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets whether values must be unique across records.
    pub fn unique(mut self, unique: bool) -> Self {
        self.property.unique = unique;
        self
    }

    /// Sets the default date-time value when creating new records.
    pub fn default_value(mut self, default_value: DateTime<FixedOffset>) -> Self {
        self.property.default_value = Some(default_value);
        self
    }

    /// Sets whether to use the current date-time as default.
    pub fn default_now_value(mut self, default_now_value: bool) -> Self {
        self.property.default_now_value = default_now_value;
        self
    }

    /// Builds the final [`DateTimeFieldProperty`].
    pub fn build(self) -> DateTimeFieldProperty {
        self.property
    }
}

impl From<DateTimeFieldPropertyBuilder> for DateTimeFieldProperty {
    fn from(builder: DateTimeFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new radio button field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{radio_button_field_property, FieldOption, Alignment};
/// use std::collections::HashMap;
///
/// let mut options = HashMap::new();
/// options.insert("small".to_string(), FieldOption {
///     label: "Small".to_string(),
///     index: 0,
/// });
/// options.insert("medium".to_string(), FieldOption {
///     label: "Medium".to_string(),
///     index: 1,
/// });
///
/// let field = radio_button_field_property("size")
///     .label("Size")
///     .required(true)
///     .default_value("medium")
///     .align(Alignment::Horizontal)
///     .options(options)
///     .build();
/// ```
pub fn radio_button_field_property(code: impl Into<String>) -> RadioButtonFieldPropertyBuilder {
    RadioButtonFieldPropertyBuilder {
        property: RadioButtonFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: None,
            align: None,
            options: HashMap::new(),
        },
    }
}

/// Builder for creating [`RadioButtonFieldProperty`].
#[derive(Debug, Clone)]
pub struct RadioButtonFieldPropertyBuilder {
    property: RadioButtonFieldProperty,
}

impl RadioButtonFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected option.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Sets the layout alignment of the options.
    pub fn align(mut self, align: Alignment) -> Self {
        self.property.align = Some(align);
        self
    }

    /// Sets the available options.
    pub fn options(mut self, options: HashMap<String, FieldOption>) -> Self {
        self.property.options = options;
        self
    }

    /// Adds a single option to the field.
    pub fn add_option(
        mut self,
        value: impl Into<String>,
        label: impl Into<String>,
        index: u64,
    ) -> Self {
        let option = FieldOption {
            label: label.into(),
            index,
        };
        self.property.options.insert(value.into(), option);
        self
    }

    /// Builds the final [`RadioButtonFieldProperty`].
    pub fn build(self) -> RadioButtonFieldProperty {
        self.property
    }
}

impl From<RadioButtonFieldPropertyBuilder> for RadioButtonFieldProperty {
    fn from(builder: RadioButtonFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new checkbox field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{checkbox_field_property, FieldOption, Alignment};
/// use std::collections::HashMap;
///
/// let mut options = HashMap::new();
/// options.insert("red".to_string(), FieldOption {
///     label: "Red".to_string(),
///     index: 0,
/// });
/// options.insert("green".to_string(), FieldOption {
///     label: "Green".to_string(),
///     index: 1,
/// });
///
/// let field = checkbox_field_property("colors")
///     .label("Colors")
///     .required(true)
///     .default_value(vec!["red".to_string()])
///     .align(Alignment::Vertical)
///     .options(options)
///     .build();
/// ```
pub fn checkbox_field_property(code: impl Into<String>) -> CheckBoxFieldPropertyBuilder {
    CheckBoxFieldPropertyBuilder {
        property: CheckBoxFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: Vec::new(),
            align: None,
            options: HashMap::new(),
        },
    }
}

/// Builder for creating [`CheckBoxFieldProperty`].
#[derive(Debug, Clone)]
pub struct CheckBoxFieldPropertyBuilder {
    property: CheckBoxFieldProperty,
}

impl CheckBoxFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected options.
    pub fn default_value(mut self, default_value: Vec<String>) -> Self {
        self.property.default_value = default_value;
        self
    }

    /// Sets the layout alignment of the options.
    pub fn align(mut self, align: Alignment) -> Self {
        self.property.align = Some(align);
        self
    }

    /// Sets the available options.
    pub fn options(mut self, options: HashMap<String, FieldOption>) -> Self {
        self.property.options = options;
        self
    }

    /// Adds a single option to the field.
    pub fn add_option(
        mut self,
        value: impl Into<String>,
        label: impl Into<String>,
        index: u64,
    ) -> Self {
        let option = FieldOption {
            label: label.into(),
            index,
        };
        self.property.options.insert(value.into(), option);
        self
    }

    /// Builds the final [`CheckBoxFieldProperty`].
    pub fn build(self) -> CheckBoxFieldProperty {
        self.property
    }
}

impl From<CheckBoxFieldPropertyBuilder> for CheckBoxFieldProperty {
    fn from(builder: CheckBoxFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new multi-select field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{multi_select_field_property, FieldOption};
/// use std::collections::HashMap;
///
/// let mut options = HashMap::new();
/// options.insert("red".to_string(), FieldOption {
///     label: "Red".to_string(),
///     index: 0,
/// });
/// options.insert("green".to_string(), FieldOption {
///     label: "Green".to_string(),
///     index: 1,
/// });
///
/// let field = multi_select_field_property("colors")
///     .label("Colors")
///     .required(true)
///     .default_value(vec!["red".to_string()])
///     .options(options)
///     .build();
/// ```
pub fn multi_select_field_property(code: impl Into<String>) -> MultiSelectFieldPropertyBuilder {
    MultiSelectFieldPropertyBuilder {
        property: MultiSelectFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: Vec::new(),
            options: HashMap::new(),
        },
    }
}

/// Builder for creating [`MultiSelectFieldProperty`].
#[derive(Debug, Clone)]
pub struct MultiSelectFieldPropertyBuilder {
    property: MultiSelectFieldProperty,
}

impl MultiSelectFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected options.
    pub fn default_value(mut self, default_value: Vec<String>) -> Self {
        self.property.default_value = default_value;
        self
    }

    /// Sets the available options.
    pub fn options(mut self, options: HashMap<String, FieldOption>) -> Self {
        self.property.options = options;
        self
    }

    /// Adds a single option to the field.
    pub fn add_option(
        mut self,
        value: impl Into<String>,
        label: impl Into<String>,
        index: u64,
    ) -> Self {
        let option = FieldOption {
            label: label.into(),
            index,
        };
        self.property.options.insert(value.into(), option);
        self
    }

    /// Builds the final [`MultiSelectFieldProperty`].
    pub fn build(self) -> MultiSelectFieldProperty {
        self.property
    }
}

impl From<MultiSelectFieldPropertyBuilder> for MultiSelectFieldProperty {
    fn from(builder: MultiSelectFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new dropdown field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{dropdown_field_property, FieldOption};
/// use std::collections::HashMap;
///
/// let mut options = HashMap::new();
/// options.insert("high".to_string(), FieldOption {
///     label: "High".to_string(),
///     index: 0,
/// });
/// options.insert("medium".to_string(), FieldOption {
///     label: "Medium".to_string(),
///     index: 1,
/// });
///
/// let field = dropdown_field_property("priority")
///     .label("Priority")
///     .required(true)
///     .default_value("medium")
///     .options(options)
///     .build();
/// ```
pub fn dropdown_field_property(code: impl Into<String>) -> DropDownFieldPropertyBuilder {
    DropDownFieldPropertyBuilder {
        property: DropDownFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: None,
            options: HashMap::new(),
        },
    }
}

/// Builder for creating [`DropDownFieldProperty`].
#[derive(Debug, Clone)]
pub struct DropDownFieldPropertyBuilder {
    property: DropDownFieldProperty,
}

impl DropDownFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected option.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Sets the available options.
    pub fn options(mut self, options: HashMap<String, FieldOption>) -> Self {
        self.property.options = options;
        self
    }

    /// Adds a single option to the field.
    pub fn add_option(
        mut self,
        value: impl Into<String>,
        label: impl Into<String>,
        index: u64,
    ) -> Self {
        let option = FieldOption {
            label: label.into(),
            index,
        };
        self.property.options.insert(value.into(), option);
        self
    }

    /// Builds the final [`DropDownFieldProperty`].
    pub fn build(self) -> DropDownFieldProperty {
        self.property
    }
}

impl From<DropDownFieldPropertyBuilder> for DropDownFieldProperty {
    fn from(builder: DropDownFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new file field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::file_field_property;
///
/// let field = file_field_property("attachments")
///     .label("Attachments")
///     .required(false)
///     .thumbnail_size(150)
///     .build();
/// ```
pub fn file_field_property(code: impl Into<String>) -> FileFieldPropertyBuilder {
    FileFieldPropertyBuilder {
        property: FileFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            thumbnail_size: None,
        },
    }
}

/// Builder for creating [`FileFieldProperty`].
#[derive(Debug, Clone)]
pub struct FileFieldPropertyBuilder {
    property: FileFieldProperty,
}

impl FileFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the size of thumbnail images in pixels.
    pub fn thumbnail_size(mut self, thumbnail_size: u64) -> Self {
        self.property.thumbnail_size = Some(thumbnail_size);
        self
    }

    /// Builds the final [`FileFieldProperty`].
    pub fn build(self) -> FileFieldProperty {
        self.property
    }
}

impl From<FileFieldPropertyBuilder> for FileFieldProperty {
    fn from(builder: FileFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new link field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{link_field_property, LinkProtocol};
///
/// let field = link_field_property("website")
///     .label("Website")
///     .required(true)
///     .unique(false)
///     .default_value("https://example.com")
///     .max_length(200)
///     .min_length(10)
///     .protocol(LinkProtocol::Web)
///     .build();
/// ```
pub fn link_field_property(code: impl Into<String>) -> LinkFieldPropertyBuilder {
    LinkFieldPropertyBuilder {
        property: LinkFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            unique: false,
            default_value: None,
            max_length: None,
            min_length: None,
            protocol: LinkProtocol::default(),
        },
    }
}

/// Builder for creating [`LinkFieldProperty`].
#[derive(Debug, Clone)]
pub struct LinkFieldPropertyBuilder {
    property: LinkFieldProperty,
}

impl LinkFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets whether values must be unique across records.
    pub fn unique(mut self, unique: bool) -> Self {
        self.property.unique = unique;
        self
    }

    /// Sets the default link value when creating new records.
    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.property.default_value = Some(default_value.into());
        self
    }

    /// Sets the maximum allowed length.
    pub fn max_length(mut self, max_length: u64) -> Self {
        self.property.max_length = Some(max_length);
        self
    }

    /// Sets the minimum required length.
    pub fn min_length(mut self, min_length: u64) -> Self {
        self.property.min_length = Some(min_length);
        self
    }

    /// Sets the protocol type for the link.
    pub fn protocol(mut self, protocol: LinkProtocol) -> Self {
        self.property.protocol = protocol;
        self
    }

    /// Builds the final [`LinkFieldProperty`].
    pub fn build(self) -> LinkFieldProperty {
        self.property
    }
}

impl From<LinkFieldPropertyBuilder> for LinkFieldProperty {
    fn from(builder: LinkFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new user select field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::user_select_field_property;
/// use kintone::model::{Entity, EntityType};
///
/// let field = user_select_field_property("assignee")
///     .label("Assignee")
///     .required(true)
///     .default_value(vec![
///         Entity { code: "user1".to_string(), entity_type: EntityType::USER }
///     ])
///     .entities(vec![
///         Entity { code: "user1".to_string(), entity_type: EntityType::USER },
///         Entity { code: "user2".to_string(), entity_type: EntityType::USER },
///     ])
///     .build();
/// ```
pub fn user_select_field_property(code: impl Into<String>) -> UserSelectFieldPropertyBuilder {
    UserSelectFieldPropertyBuilder {
        property: UserSelectFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: Vec::new(),
            entities: Vec::new(),
        },
    }
}

/// Builder for creating [`UserSelectFieldProperty`].
#[derive(Debug, Clone)]
pub struct UserSelectFieldPropertyBuilder {
    property: UserSelectFieldProperty,
}

impl UserSelectFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected users.
    pub fn default_value(mut self, default_value: Vec<Entity>) -> Self {
        self.property.default_value = default_value;
        self
    }

    /// Sets the available users that can be selected.
    pub fn entities(mut self, entities: Vec<Entity>) -> Self {
        self.property.entities = entities;
        self
    }

    /// Adds a single user to the available entities.
    pub fn add_entity(
        mut self,
        code: impl Into<String>,
        entity_type: crate::model::EntityType,
    ) -> Self {
        let entity = Entity {
            code: code.into(),
            entity_type,
        };
        self.property.entities.push(entity);
        self
    }

    /// Builds the final [`UserSelectFieldProperty`].
    pub fn build(self) -> UserSelectFieldProperty {
        self.property
    }
}

impl From<UserSelectFieldPropertyBuilder> for UserSelectFieldProperty {
    fn from(builder: UserSelectFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new organization select field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::organization_select_field_property;
/// use kintone::model::{Entity, EntityType};
///
/// let field = organization_select_field_property("department")
///     .label("Department")
///     .required(true)
///     .default_value(vec![
///         Entity { code: "eng".to_string(), entity_type: EntityType::ORGANIZATION }
///     ])
///     .entities(vec![
///         Entity { code: "eng".to_string(), entity_type: EntityType::ORGANIZATION },
///         Entity { code: "sales".to_string(), entity_type: EntityType::ORGANIZATION },
///     ])
///     .build();
/// ```
pub fn organization_select_field_property(
    code: impl Into<String>,
) -> OrganizationSelectFieldPropertyBuilder {
    OrganizationSelectFieldPropertyBuilder {
        property: OrganizationSelectFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: Vec::new(),
            entities: Vec::new(),
        },
    }
}

/// Builder for creating [`OrganizationSelectFieldProperty`].
#[derive(Debug, Clone)]
pub struct OrganizationSelectFieldPropertyBuilder {
    property: OrganizationSelectFieldProperty,
}

impl OrganizationSelectFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected organizations.
    pub fn default_value(mut self, default_value: Vec<Entity>) -> Self {
        self.property.default_value = default_value;
        self
    }

    /// Sets the available organizations that can be selected.
    pub fn entities(mut self, entities: Vec<Entity>) -> Self {
        self.property.entities = entities;
        self
    }

    /// Adds a single organization to the available entities.
    pub fn add_entity(
        mut self,
        code: impl Into<String>,
        entity_type: crate::model::EntityType,
    ) -> Self {
        let entity = Entity {
            code: code.into(),
            entity_type,
        };
        self.property.entities.push(entity);
        self
    }

    /// Builds the final [`OrganizationSelectFieldProperty`].
    pub fn build(self) -> OrganizationSelectFieldProperty {
        self.property
    }
}

impl From<OrganizationSelectFieldPropertyBuilder> for OrganizationSelectFieldProperty {
    fn from(builder: OrganizationSelectFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new group select field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::group_select_field_property;
/// use kintone::model::{Entity, EntityType};
///
/// let field = group_select_field_property("team")
///     .label("Team")
///     .required(true)
///     .default_value(vec![
///         Entity { code: "backend".to_string(), entity_type: EntityType::GROUP }
///     ])
///     .entities(vec![
///         Entity { code: "backend".to_string(), entity_type: EntityType::GROUP },
///         Entity { code: "frontend".to_string(), entity_type: EntityType::GROUP },
///     ])
///     .build();
/// ```
pub fn group_select_field_property(code: impl Into<String>) -> GroupSelectFieldPropertyBuilder {
    GroupSelectFieldPropertyBuilder {
        property: GroupSelectFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            required: false,
            default_value: Vec::new(),
            entities: Vec::new(),
        },
    }
}

/// Builder for creating [`GroupSelectFieldProperty`].
#[derive(Debug, Clone)]
pub struct GroupSelectFieldPropertyBuilder {
    property: GroupSelectFieldProperty,
}

impl GroupSelectFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the field is required.
    pub fn required(mut self, required: bool) -> Self {
        self.property.required = required;
        self
    }

    /// Sets the default selected groups.
    pub fn default_value(mut self, default_value: Vec<Entity>) -> Self {
        self.property.default_value = default_value;
        self
    }

    /// Sets the available groups that can be selected.
    pub fn entities(mut self, entities: Vec<Entity>) -> Self {
        self.property.entities = entities;
        self
    }

    /// Adds a single group to the available entities.
    pub fn add_entity(
        mut self,
        code: impl Into<String>,
        entity_type: crate::model::EntityType,
    ) -> Self {
        let entity = Entity {
            code: code.into(),
            entity_type,
        };
        self.property.entities.push(entity);
        self
    }

    /// Builds the final [`GroupSelectFieldProperty`].
    pub fn build(self) -> GroupSelectFieldProperty {
        self.property
    }
}

impl From<GroupSelectFieldPropertyBuilder> for GroupSelectFieldProperty {
    fn from(builder: GroupSelectFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new reference table field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{reference_table_field_property, ReferenceTable, RelatedApp, ReferenceCondition};
///
/// let reference_table = ReferenceTable {
///     related_app: RelatedApp {
///         app: Some(123),
///         code: None,
///     },
///     condition: ReferenceCondition {
///         field: "customer_id".to_string(),
///         related_field: "id".to_string(),
///     },
///     filter_cond: Some("status = \"active\"".to_string()),
///     display_fields: vec!["name".to_string(), "email".to_string()],
///     sort: Some("name asc".to_string()),
///     size: Some(10),
/// };
///
/// let field = reference_table_field_property("related_customers")
///     .label("Related Customers")
///     .reference_table(reference_table)
///     .build();
/// ```
pub fn reference_table_field_property(
    code: impl Into<String>,
) -> ReferenceTableFieldPropertyBuilder {
    ReferenceTableFieldPropertyBuilder {
        property: ReferenceTableFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            reference_table: ReferenceTable::default(),
        },
    }
}

/// Builder for creating [`ReferenceTableFieldProperty`].
#[derive(Debug, Clone)]
pub struct ReferenceTableFieldPropertyBuilder {
    property: ReferenceTableFieldProperty,
}

impl ReferenceTableFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets the configuration for the referenced table.
    pub fn reference_table(mut self, reference_table: ReferenceTable) -> Self {
        self.property.reference_table = reference_table;
        self
    }

    /// Builds the final [`ReferenceTableFieldProperty`].
    pub fn build(self) -> ReferenceTableFieldProperty {
        self.property
    }
}

impl From<ReferenceTableFieldPropertyBuilder> for ReferenceTableFieldProperty {
    fn from(builder: ReferenceTableFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new group field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::group_field_property;
///
/// let field = group_field_property("contact_info")
///     .label("Contact Information")
///     .open_group(true)
///     .build();
/// ```
pub fn group_field_property(code: impl Into<String>) -> GroupFieldPropertyBuilder {
    GroupFieldPropertyBuilder {
        property: GroupFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            open_group: false,
        },
    }
}

/// Builder for creating [`GroupFieldProperty`].
#[derive(Debug, Clone)]
pub struct GroupFieldPropertyBuilder {
    property: GroupFieldProperty,
}

impl GroupFieldPropertyBuilder {
    /// Sets the display label for the group.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the group label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets whether the group should be expanded by default.
    pub fn open_group(mut self, open_group: bool) -> Self {
        self.property.open_group = open_group;
        self
    }

    /// Builds the final [`GroupFieldProperty`].
    pub fn build(self) -> GroupFieldProperty {
        self.property
    }
}

impl From<GroupFieldPropertyBuilder> for GroupFieldProperty {
    fn from(builder: GroupFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new subtable field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::{subtable_field_property, single_line_text_field_property};
/// use std::collections::HashMap;
///
/// let mut fields = HashMap::new();
/// fields.insert("name".to_string(), single_line_text_field_property("name")
///     .label("Name")
///     .required(true)
///     .build()
///     .into());
///
/// let field = subtable_field_property("items")
///     .label("Items")
///     .fields(fields)
///     .build();
/// ```
pub fn subtable_field_property(code: impl Into<String>) -> SubtableFieldPropertyBuilder {
    SubtableFieldPropertyBuilder {
        property: SubtableFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
            fields: HashMap::new(),
        },
    }
}

/// Builder for creating [`SubtableFieldProperty`].
#[derive(Debug, Clone)]
pub struct SubtableFieldPropertyBuilder {
    property: SubtableFieldProperty,
}

impl SubtableFieldPropertyBuilder {
    /// Sets the display label for the subtable.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the subtable label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Sets the fields that make up each row in the subtable.
    pub fn fields(mut self, fields: HashMap<String, FieldProperty>) -> Self {
        self.property.fields = fields;
        self
    }

    /// Adds a single field to the subtable.
    pub fn add_field(mut self, code: impl Into<String>, field: FieldProperty) -> Self {
        self.property.fields.insert(code.into(), field);
        self
    }

    /// Builds the final [`SubtableFieldProperty`].
    pub fn build(self) -> SubtableFieldProperty {
        self.property
    }
}

impl From<SubtableFieldPropertyBuilder> for SubtableFieldProperty {
    fn from(builder: SubtableFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new record number field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::record_number_field_property;
///
/// let field = record_number_field_property("record_number")
///     .label("Record Number")
///     .no_label(false)
///     .build();
/// ```
pub fn record_number_field_property(code: impl Into<String>) -> RecordNumberFieldPropertyBuilder {
    RecordNumberFieldPropertyBuilder {
        property: RecordNumberFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
        },
    }
}

/// Builder for creating [`RecordNumberFieldProperty`].
#[derive(Debug, Clone)]
pub struct RecordNumberFieldPropertyBuilder {
    property: RecordNumberFieldProperty,
}

impl RecordNumberFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Builds the final [`RecordNumberFieldProperty`].
    pub fn build(self) -> RecordNumberFieldProperty {
        self.property
    }
}

impl From<RecordNumberFieldPropertyBuilder> for RecordNumberFieldProperty {
    fn from(builder: RecordNumberFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new category field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::category_field_property;
///
/// let field = category_field_property("category")
///     .label("Category")
///     .enabled(true)
///     .build();
/// ```
pub fn category_field_property(code: impl Into<String>) -> CategoryFieldPropertyBuilder {
    CategoryFieldPropertyBuilder {
        property: CategoryFieldProperty {
            code: code.into(),
            label: String::new(),
            enabled: true,
        },
    }
}

/// Builder for creating [`CategoryFieldProperty`].
#[derive(Debug, Clone)]
pub struct CategoryFieldPropertyBuilder {
    property: CategoryFieldProperty,
}

impl CategoryFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether the category field is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.property.enabled = enabled;
        self
    }

    /// Builds the final [`CategoryFieldProperty`].
    pub fn build(self) -> CategoryFieldProperty {
        self.property
    }
}

impl From<CategoryFieldPropertyBuilder> for CategoryFieldProperty {
    fn from(builder: CategoryFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new status field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::status_field_property;
///
/// let field = status_field_property("status")
///     .label("Status")
///     .enabled(true)
///     .build();
/// ```
pub fn status_field_property(code: impl Into<String>) -> StatusFieldPropertyBuilder {
    StatusFieldPropertyBuilder {
        property: StatusFieldProperty {
            code: code.into(),
            label: String::new(),
            enabled: true,
        },
    }
}

/// Builder for creating [`StatusFieldProperty`].
#[derive(Debug, Clone)]
pub struct StatusFieldPropertyBuilder {
    property: StatusFieldProperty,
}

impl StatusFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether the status field is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.property.enabled = enabled;
        self
    }

    /// Builds the final [`StatusFieldProperty`].
    pub fn build(self) -> StatusFieldProperty {
        self.property
    }
}

impl From<StatusFieldPropertyBuilder> for StatusFieldProperty {
    fn from(builder: StatusFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new status assignee field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::status_assignee_field_property;
///
/// let field = status_assignee_field_property("assignee")
///     .label("Assignee")
///     .enabled(true)
///     .build();
/// ```
pub fn status_assignee_field_property(
    code: impl Into<String>,
) -> StatusAssigneeFieldPropertyBuilder {
    StatusAssigneeFieldPropertyBuilder {
        property: StatusAssigneeFieldProperty {
            code: code.into(),
            label: String::new(),
            enabled: true,
        },
    }
}

/// Builder for creating [`StatusAssigneeFieldProperty`].
#[derive(Debug, Clone)]
pub struct StatusAssigneeFieldPropertyBuilder {
    property: StatusAssigneeFieldProperty,
}

impl StatusAssigneeFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether the assignee field is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.property.enabled = enabled;
        self
    }

    /// Builds the final [`StatusAssigneeFieldProperty`].
    pub fn build(self) -> StatusAssigneeFieldProperty {
        self.property
    }
}

impl From<StatusAssigneeFieldPropertyBuilder> for StatusAssigneeFieldProperty {
    fn from(builder: StatusAssigneeFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new created time field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::created_time_field_property;
///
/// let field = created_time_field_property("created_time")
///     .label("Created Time")
///     .no_label(false)
///     .build();
/// ```
pub fn created_time_field_property(code: impl Into<String>) -> CreatedTimeFieldPropertyBuilder {
    CreatedTimeFieldPropertyBuilder {
        property: CreatedTimeFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
        },
    }
}

/// Builder for creating [`CreatedTimeFieldProperty`].
#[derive(Debug, Clone)]
pub struct CreatedTimeFieldPropertyBuilder {
    property: CreatedTimeFieldProperty,
}

impl CreatedTimeFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Builds the final [`CreatedTimeFieldProperty`].
    pub fn build(self) -> CreatedTimeFieldProperty {
        self.property
    }
}

impl From<CreatedTimeFieldPropertyBuilder> for CreatedTimeFieldProperty {
    fn from(builder: CreatedTimeFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new updated time field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::updated_time_field_property;
///
/// let field = updated_time_field_property("updated_time")
///     .label("Updated Time")
///     .no_label(false)
///     .build();
/// ```
pub fn updated_time_field_property(code: impl Into<String>) -> UpdatedTimeFieldPropertyBuilder {
    UpdatedTimeFieldPropertyBuilder {
        property: UpdatedTimeFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
        },
    }
}

/// Builder for creating [`UpdatedTimeFieldProperty`].
#[derive(Debug, Clone)]
pub struct UpdatedTimeFieldPropertyBuilder {
    property: UpdatedTimeFieldProperty,
}

impl UpdatedTimeFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Builds the final [`UpdatedTimeFieldProperty`].
    pub fn build(self) -> UpdatedTimeFieldProperty {
        self.property
    }
}

impl From<UpdatedTimeFieldPropertyBuilder> for UpdatedTimeFieldProperty {
    fn from(builder: UpdatedTimeFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new creator field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::creator_field_property;
///
/// let field = creator_field_property("creator")
///     .label("Creator")
///     .no_label(false)
///     .build();
/// ```
pub fn creator_field_property(code: impl Into<String>) -> CreatorFieldPropertyBuilder {
    CreatorFieldPropertyBuilder {
        property: CreatorFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
        },
    }
}

/// Builder for creating [`CreatorFieldProperty`].
#[derive(Debug, Clone)]
pub struct CreatorFieldPropertyBuilder {
    property: CreatorFieldProperty,
}

impl CreatorFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Builds the final [`CreatorFieldProperty`].
    pub fn build(self) -> CreatorFieldProperty {
        self.property
    }
}

impl From<CreatorFieldPropertyBuilder> for CreatorFieldProperty {
    fn from(builder: CreatorFieldPropertyBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new modifier field builder.
///
/// # Arguments
/// * `code` - Field code (unique identifier)
///
/// # Examples
/// ```rust
/// use kintone::model::app::field::modifier_field_property;
///
/// let field = modifier_field_property("modifier")
///     .label("Modifier")
///     .no_label(false)
///     .build();
/// ```
pub fn modifier_field_property(code: impl Into<String>) -> ModifierFieldPropertyBuilder {
    ModifierFieldPropertyBuilder {
        property: ModifierFieldProperty {
            code: code.into(),
            label: String::new(),
            no_label: false,
        },
    }
}

/// Builder for creating [`ModifierFieldProperty`].
#[derive(Debug, Clone)]
pub struct ModifierFieldPropertyBuilder {
    property: ModifierFieldProperty,
}

impl ModifierFieldPropertyBuilder {
    /// Sets the display label for the field.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.property.label = label.into();
        self
    }

    /// Sets whether to hide the field label.
    pub fn no_label(mut self, no_label: bool) -> Self {
        self.property.no_label = no_label;
        self
    }

    /// Builds the final [`ModifierFieldProperty`].
    pub fn build(self) -> ModifierFieldProperty {
        self.property
    }
}

impl From<ModifierFieldPropertyBuilder> for ModifierFieldProperty {
    fn from(builder: ModifierFieldPropertyBuilder) -> Self {
        builder.build()
    }
}
