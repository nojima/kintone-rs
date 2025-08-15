//! # Kintone Record Models
//! 
//! This module provides the core data structures for working with records in Kintone applications.
//! Records are the fundamental data containers that hold field values, similar to rows in a database.
//!
//! # Core Types
//!
//! - [`Record`] - A collection of field values representing a single record
//! - [`FieldValue`] - Enum containing all possible field value types
//! - [`FieldType`] - Enum identifying the type of a field
//! - [`TableRow`] - Represents a row within a table field
//! - [`RecordComment`] - Comments associated with records
//!
//! # Basic Usage
//!
//! Create and manipulate records with field values:
//!
//! ```rust
//! use kintone::model::record::{Record, FieldValue};
//! use bigdecimal::BigDecimal;
//!
//! // Create a new record
//! let mut record = Record::new();
//!
//! // Add various field types
//! record.put_field("title", FieldValue::SingleLineText("Project Alpha".to_string()));
//! record.put_field("budget", FieldValue::Number(BigDecimal::from(50000)));
//! record.put_field("priority", FieldValue::RadioButton(Some("High".to_string())));
//! record.put_field("active", FieldValue::CheckBox(vec!["Yes".to_string()]));
//!
//! // Read field value
//! let Some(FieldValue::SingleLineText(title)) = record.get("title") else {
//!     panic!("Title is not set");
//! };
//! println!("Title: {}", title);
//! ```
//!
//! ## Alternative Initialization Methods
//!
//! You can also create records using the `From` trait with an array of field tuples:
//!
//! ```rust
//! use kintone::model::record::{Record, FieldValue};
//! use bigdecimal::BigDecimal;
//!
//! // Create a record with initial fields using From trait
//! let record = Record::from([
//!     ("name", FieldValue::SingleLineText("John Doe".to_string())),
//!     ("age", FieldValue::Number(BigDecimal::from(30))),
//!     ("email", FieldValue::SingleLineText("john@example.com".to_string())),
//! ]);
//!
//! // This is equivalent to creating an empty record and adding fields one by one
//! assert_eq!(record.field_codes().count(), 3);
//! ```
//!
//! # Working with Table Fields
//!
//! Table fields contain multiple rows of related data:
//!
//! ```rust
//! use kintone::model::record::{Record, FieldValue, TableRow};
//!
//! let mut record = Record::new();
//! let mut table_rows = Vec::new();
//!
//! // Create table rows
//! let mut row1 = TableRow::new();
//! row1.put_field("item", FieldValue::SingleLineText("Item 1".to_string()));
//! row1.put_field("quantity", FieldValue::Number(10.into()));
//! table_rows.push(row1);
//!
//! let mut row2 = TableRow::new();
//! row2.put_field("item", FieldValue::SingleLineText("Item 2".to_string()));
//! row2.put_field("quantity", FieldValue::Number(5.into()));
//! table_rows.push(row2);
//!
//! // Add subtable to record
//! record.put_field("items", FieldValue::Subtable(table_rows));
//! ```
//!
//! # Type-Safe Field Access
//!
//! The [`FieldValue`] enum provides type-safe access to field data while handling
//! Kintone's dynamic field system. Each variant corresponds to a specific field type
//! and ensures proper serialization/deserialization with the Kintone API.

use std::{borrow::Borrow, collections::HashMap};

use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

use crate::{
    internal::serde_helper::stringified,
    model::{Entity, FileBody, Group, Organization, User},
};

/// Represents a record in a Kintone application.
///
/// A record is a collection of field values identified by field codes (names).
/// Records are the primary data structure in Kintone applications, similar to
/// rows in a database table. Each record can contain various types of fields
/// such as text, numbers, dates, attachments, and more.
///
/// # Examples
///
/// ```rust
/// use kintone::model::record::{Record, FieldValue};
///
/// // Create a new record
/// let mut record = Record::new();
///
/// // Add fields to the record
/// record.put_field("name", FieldValue::SingleLineText("John Doe".to_owned()));
/// record.put_field("age", FieldValue::Number(30.into()));
/// record.put_field("email", FieldValue::Link("john@example.com".to_owned()));
///
/// // Read field values
/// if let Some(FieldValue::SingleLineText(name)) = record.get("name") {
///     println!("Name: {}", name);
/// }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct Record {
    #[serde(flatten)]
    fields: HashMap<String, FieldValue>,
}

impl Record {
    /// Creates a new empty record.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::Record;
    ///
    /// let record = Record::new();
    /// assert_eq!(record.fields().len(), 0);
    /// ```
    pub fn new() -> Self {
        Record {
            fields: HashMap::new(),
        }
    }

    /// Creates a new record with the specified initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Record {
            fields: HashMap::with_capacity(capacity),
        }
    }

    /// Creates a copy of the record without built-in system fields.
    ///
    /// Built-in fields are system-managed fields like record ID, creator, creation time,
    /// modifier, and modification time. This method is useful when you want to create
    /// a new record based on an existing one, excluding the system-generated fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut original = Record::new();
    /// original.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    /// // Assume the record also has built-in fields after being retrieved from Kintone
    ///
    /// let clean_copy = original.clone_without_builtins();
    /// // clean_copy only contains user-defined fields, not system fields
    /// ```
    pub fn clone_without_builtins(&self) -> Self {
        let size = self.field_values().filter(|v| !v.field_type().is_builtin()).count();
        let mut fields = HashMap::with_capacity(size);
        for (code, value) in self.fields() {
            if !value.field_type().is_builtin() {
                fields.insert(code.to_owned(), value.clone());
            }
        }
        Record { fields }
    }

    /// Gets a reference to the field value for the specified field code.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code (name) to look up
    ///
    /// # Returns
    ///
    /// `Some(&FieldValue)` if the field exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    ///
    /// if let Some(FieldValue::SingleLineText(name)) = record.get("name") {
    ///     println!("Name: {}", name);
    /// }
    /// ```
    pub fn get(&self, field_code: &str) -> Option<&FieldValue> {
        self.fields.get(field_code)
    }

    /// Gets a mutable reference to the field value for the specified field code.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code (name) to look up
    ///
    /// # Returns
    ///
    /// `Some(&mut FieldValue)` if the field exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    ///
    /// if let Some(FieldValue::SingleLineText(name)) = record.get_mut("name") {
    ///     *name = "Jane".to_owned();
    /// }
    /// ```
    pub fn get_mut(&mut self, field_code: &str) -> Option<&mut FieldValue> {
        self.fields.get_mut(field_code)
    }

    /// Returns an iterator over all field codes and values in the record.
    ///
    /// The iterator yields tuples of `(&str, &FieldValue)` representing
    /// the field code and its corresponding value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    /// record.put_field("age", FieldValue::Number(30.into()));
    ///
    /// for (field_code, field_value) in record.fields() {
    ///     println!("{}: {:?}", field_code, field_value);
    /// }
    /// ```
    pub fn fields(&self) -> impl ExactSizeIterator<Item = (&'_ str, &'_ FieldValue)> + Clone {
        self.fields.iter().map(|(k, v)| (k.borrow(), v))
    }

    /// Returns a mutable iterator over all field codes and values in the record.
    ///
    /// The iterator yields tuples of `(&str, &mut FieldValue)` representing
    /// the field code and its corresponding mutable value reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    ///
    /// for (field_code, field_value) in record.fields_mut() {
    ///     if let FieldValue::SingleLineText(text) = field_value {
    ///         *text = text.to_uppercase();
    ///     }
    /// }
    /// ```
    pub fn fields_mut(&mut self) -> impl ExactSizeIterator<Item = (&'_ str, &'_ mut FieldValue)> {
        self.fields.iter_mut().map(|(k, v)| (k.borrow(), v))
    }

    /// Returns an iterator over all field codes in the record.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    /// record.put_field("age", FieldValue::Number(30.into()));
    ///
    /// let field_codes: Vec<_> = record.field_codes().collect();
    /// assert_eq!(field_codes.len(), 2);
    /// ```
    pub fn field_codes(&self) -> impl ExactSizeIterator<Item = &'_ str> + Clone {
        self.fields.keys().map(|k| k.borrow())
    }

    /// Returns an iterator over all field values in the record.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    /// record.put_field("age", FieldValue::Number(30.into()));
    ///
    /// let field_values: Vec<_> = record.field_values().collect();
    /// assert_eq!(field_values.len(), 2);
    /// ```
    pub fn field_values(&self) -> impl ExactSizeIterator<Item = &'_ FieldValue> + Clone {
        self.fields.values()
    }

    /// Inserts a field value into the record.
    ///
    /// If the field already exists, its value is replaced and the old value is returned.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code (name) for the field
    /// * `value` - The field value to insert
    ///
    /// # Returns
    ///
    /// The previous value if the field existed, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// let old_value = record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    /// assert!(old_value.is_none());
    ///
    /// let replaced_value = record.put_field("name", FieldValue::SingleLineText("Jane".to_owned()));
    /// assert!(replaced_value.is_some());
    /// ```
    pub fn put_field(
        &mut self,
        field_code: impl Into<String>,
        value: FieldValue,
    ) -> Option<FieldValue> {
        self.fields.insert(field_code.into(), value)
    }

    /// Removes a field from the record.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code to remove
    ///
    /// # Returns
    ///
    /// The removed field value if it existed, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::{Record, FieldValue};
    ///
    /// let mut record = Record::new();
    /// record.put_field("name", FieldValue::SingleLineText("John".to_owned()));
    ///
    /// let removed = record.remove_field("name");
    /// assert!(removed.is_some());
    ///
    /// let not_found = record.remove_field("nonexistent");
    /// assert!(not_found.is_none());
    /// ```
    pub fn remove_field(&mut self, field_code: &str) -> Option<FieldValue> {
        self.fields.remove(field_code)
    }

    /// Gets the record ID if available.
    ///
    /// The record ID is a system-generated unique identifier for the record.
    /// This field is only available for records that have been saved to Kintone.
    ///
    /// # Returns
    ///
    /// `Some(id)` if the record has an ID, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::Record;
    ///
    /// let record = Record::new();
    /// assert!(record.id().is_none()); // New records don't have IDs
    ///
    /// // After saving to Kintone, the record would have an ID
    /// ```
    pub fn id(&self) -> Option<u64> {
        let Some(FieldValue::__ID__(value)) = self.get("$id") else {
            return None;
        };
        Some(*value)
    }

    /// Gets the record revision number if available.
    ///
    /// The revision number is a system-managed version counter that increments
    /// each time the record is updated. This is used for optimistic locking
    /// to prevent concurrent modification conflicts.
    ///
    /// # Returns
    ///
    /// `Some(revision)` if the record has a revision number, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kintone::model::record::Record;
    ///
    /// let record = Record::new();
    /// assert!(record.revision().is_none()); // New records don't have revisions
    ///
    /// // After saving to Kintone, the record would have a revision number
    /// ```
    pub fn revision(&self) -> Option<u64> {
        let Some(FieldValue::__REVISION__(value)) = self.get("$revision") else {
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

impl<const N: usize, S: Into<String>> From<[(S, FieldValue); N]> for Record {
    fn from(fields: [(S, FieldValue); N]) -> Self {
        let mut record = Self::with_capacity(N);
        for (key, value) in fields {
            record.put_field(key.into(), value);
        }
        record
    }
}

/// Represents the type of a field in a Kintone application.
///
/// Each field in a Kintone app has a specific type that determines what kind of data
/// it can store and how it behaves. Some field types are built-in system fields
/// (like record ID, creation time) while others are user-defined fields.
///
/// The `is_builtin()` method can be used to distinguish between system-managed
/// and user-defined fields.
///
/// # Examples
///
/// ```rust
/// use kintone::model::record::FieldType;
///
/// assert!(!FieldType::SingleLineText.is_builtin());
/// assert!(FieldType::CreatedTime.is_builtin());
/// assert!(FieldType::Creator.is_builtin());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Assoc)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn is_builtin(&self) -> bool)]
#[non_exhaustive]
pub enum FieldType {
    /// Calculated field that computes values based on other fields
    #[assoc(is_builtin = false)]
    Calc,

    /// System field for record categories (built-in)
    #[assoc(is_builtin = true)]
    Category,

    /// Checkbox field for multiple selection options
    #[assoc(is_builtin = false)]
    CheckBox,

    /// System field for record creation timestamp (built-in)
    #[assoc(is_builtin = true)]
    CreatedTime,

    /// System field for record creator information (built-in)
    #[assoc(is_builtin = true)]
    Creator,

    /// Date field for storing dates without time
    #[assoc(is_builtin = false)]
    Date,

    /// Date and time field for storing timestamps
    #[assoc(is_builtin = false)]
    Datetime,

    /// Dropdown field for single selection from predefined options
    #[assoc(is_builtin = false)]
    DropDown,

    /// File attachment field for storing uploaded files
    #[assoc(is_builtin = false)]
    File,

    /// Group field for displaying related information
    #[assoc(is_builtin = false)]
    Group,

    /// Group selection field for choosing from predefined groups
    #[assoc(is_builtin = false)]
    GroupSelect,

    /// Horizontal rule field for visual separation
    #[assoc(is_builtin = false)]
    Hr,

    /// Label field for displaying text information
    #[assoc(is_builtin = false)]
    Label,

    /// Link field for storing URLs
    #[assoc(is_builtin = false)]
    Link,

    /// System field for record modifier information (built-in)
    #[assoc(is_builtin = true)]
    Modifier,

    /// Multi-line text field for longer text content
    #[assoc(is_builtin = false)]
    MultiLineText,

    /// Multi-select field for choosing multiple options
    #[assoc(is_builtin = false)]
    MultiSelect,

    /// Number field for storing numeric values
    #[assoc(is_builtin = false)]
    Number,

    /// Organization selection field for choosing from organizational units
    #[assoc(is_builtin = false)]
    OrganizationSelect,

    /// Radio button field for single selection
    #[assoc(is_builtin = false)]
    RadioButton,

    /// System field for unique record numbers (built-in)
    #[assoc(is_builtin = true)]
    RecordNumber,

    /// Reference table field for linking to other app records
    #[assoc(is_builtin = false)]
    ReferenceTable,

    /// Rich text field for formatted text content
    #[assoc(is_builtin = false)]
    RichText,

    /// Single-line text field for short text content
    #[assoc(is_builtin = false)]
    SingleLineText,

    /// Spacer field for layout purposes
    #[assoc(is_builtin = false)]
    Spacer,

    /// System field for workflow status (built-in)
    #[assoc(is_builtin = true)]
    Status,

    /// System field for workflow status assignee (built-in)
    #[assoc(is_builtin = true)]
    StatusAssignee,

    /// Subtable field for tabular data
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

/// Represents the value of a field in a Kintone record.
///
/// Each variant corresponds to a specific field type and contains the appropriate value type.
/// The enum is marked as `#[non_exhaustive]` to allow for future field types without breaking changes.
///
/// # Examples
///
/// ```rust
/// use kintone::model::record::FieldValue;
/// use chrono::{DateTime, FixedOffset, NaiveDate};
///
/// // Text field
/// let text_value = FieldValue::SingleLineText("Hello, world!".to_string());
///
/// // Date field
/// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
/// let date_value = FieldValue::Date(Some(date));
///
/// // Number field
/// let number_value = FieldValue::Number(42.into());
/// ```
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

/// Represents a single row in a subtable field.
///
/// A `TableRow` contains a collection of fields indexed by field code,
/// similar to a record but used within subtable contexts.
///
/// # Examples
///
/// ```rust
/// use kintone::model::record::{TableRow, FieldValue};
///
/// let mut row = TableRow::new();
/// row.put_field("name", FieldValue::SingleLineText("John Doe".to_string()));
/// row.put_field("age", FieldValue::Number(25.into()));
///
/// if let Some(name_field) = row.get("name") {
///     println!("Name: {:?}", name_field);
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    #[serde(flatten)]
    fields: HashMap<String, FieldValue>,
}

impl TableRow {
    /// Creates a new empty table row.
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Creates a new table row with the specified initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            fields: HashMap::with_capacity(capacity),
        }
    }

    /// Gets a field value by field code.
    pub fn get(&self, field_code: &str) -> Option<&FieldValue> {
        self.fields.get(field_code)
    }

    /// Gets a mutable reference to a field value by field code.
    pub fn get_mut(&mut self, field_code: &str) -> Option<&mut FieldValue> {
        self.fields.get_mut(field_code)
    }

    /// Returns an iterator over all fields in the table row.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = (&'_ str, &'_ FieldValue)> + Clone {
        self.fields.iter().map(|(k, v)| (k.borrow(), v))
    }

    /// Returns a mutable iterator over all fields in the table row.
    pub fn fields_mut(&mut self) -> impl ExactSizeIterator<Item = (&'_ str, &'_ mut FieldValue)> {
        self.fields.iter_mut().map(|(k, v)| (k.borrow(), v))
    }

    /// Returns an iterator over all field codes in the table row.
    pub fn field_codes(&self) -> impl ExactSizeIterator<Item = &'_ str> + Clone {
        self.fields.keys().map(|k| k.borrow())
    }

    /// Returns an iterator over all field values in the table row.
    pub fn field_values(&self) -> impl ExactSizeIterator<Item = &'_ FieldValue> + Clone {
        self.fields.values()
    }

    /// Adds or updates a field in the table row.
    ///
    /// Returns the previous value if the field already existed.
    pub fn put_field(
        &mut self,
        field_code: impl Into<String>,
        value: FieldValue,
    ) -> Option<FieldValue> {
        self.fields.insert(field_code.into(), value)
    }

    /// Removes a field from the table row.
    pub fn remove_field(&mut self, field_code: &str) -> Option<FieldValue> {
        self.fields.remove(field_code)
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

impl<const N: usize, S: Into<String>> From<[(S, FieldValue); N]> for TableRow {
    fn from(fields: [(S, FieldValue); N]) -> Self {
        let mut row = Self::with_capacity(N);
        for (key, value) in fields {
            row.put_field(key.into(), value);
        }
        row
    }
}

/// Represents a comment to be posted to a Kintone record.
///
/// This struct is used when creating new comments on records.
/// Use `PostedRecordComment` for comments that have already been posted.
///
/// # Examples
///
/// ```rust
/// use kintone::model::{Entity, EntityType};
/// use kintone::model::record::RecordComment;
///
/// let comment = RecordComment {
///     text: "Please review this record".to_string(),
///     mentions: vec![
///         Entity {
///             entity_type: EntityType::USER,
///             code: "user1".to_string(),
///         }
///     ],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordComment {
    /// The text content of the comment
    pub text: String,
    /// List of entities mentioned in the comment
    pub mentions: Vec<Entity>,
}

impl From<PostedRecordComment> for RecordComment {
    fn from(c: PostedRecordComment) -> Self {
        RecordComment {
            text: c.text,
            mentions: c.mentions,
        }
    }
}

/// Represents a comment that has been posted to a Kintone record.
///
/// This struct contains all the metadata for an existing comment,
/// including its ID, creation time, and author information.
///
/// # Examples
///
/// ```rust
/// use kintone::model::record::{PostedRecordComment, RecordComment};
/// use kintone::model::User;
/// use chrono::{DateTime, FixedOffset};
///
/// // Convert a PostedRecordComment to RecordComment for updating
/// let posted_comment = PostedRecordComment {
///     id: 123,
///     text: "Updated comment text".to_string(),
///     created_at: DateTime::parse_from_rfc3339("2023-12-25T10:00:00+09:00").unwrap(),
///     user: User {
///         name: "John Doe".to_string(),
///         code: "john.doe".to_string(),
///     },
///     mentions: vec![],
/// };
///
/// let comment: RecordComment = posted_comment.into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostedRecordComment {
    /// Unique identifier of the comment
    pub id: u64,
    /// The text content of the comment
    pub text: String,
    /// When the comment was created
    pub created_at: DateTime<FixedOffset>,
    /// User who created the comment
    pub user: User,
    /// List of entities mentioned in the comment
    pub mentions: Vec<Entity>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD_JSON1: &str = include_str!("../testdata/record1.json");

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
