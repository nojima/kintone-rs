//! # Kintone Data Models
//!
//! This module provides data structures for representing Kintone records, fields, and other entities.
//! These models handle serialization/deserialization between Rust types and Kintone's JSON API format.
//!
//! ## Usage Examples
//!
//! ### Creating a Record
//! ```rust
//! use kintone::model::record::{Record, FieldValue};
//!
//! let mut record = Record::new();
//! record.put_field("name", FieldValue::SingleLineText("John Doe".to_owned()));
//! record.put_field("age", FieldValue::Number(Some(30.into())));
//! println!("record = {record:?}");
//! ```
//!
//! ### Reading Field Values
//! ```rust
//! # use kintone::model::record::{Record, FieldValue};
//! # let record = Record::new();
//! if let Some(FieldValue::SingleLineText(name)) = record.get("name") {
//!     println!("Name: {}", name);
//! }
//! if let Some(FieldValue::Number(age)) = record.get("age") {
//!     println!("Age: {:?}", age);
//! }
//! ```

use crate::internal::serde_helper::option_stringified;
use serde::{Deserialize, Serialize};

pub mod app;
pub mod record;
pub mod space;

/// Represents the type of entity in Kintone's user management system.
///
/// Kintone supports three types of entities for access control and assignment:
/// users, groups, and organizations. This enum is used throughout the API
/// to specify which type of entity is being referenced.
///
/// # Examples
///
/// ```rust
/// use kintone::model::EntityType;
///
/// let user_type = EntityType::USER;
/// let group_type = EntityType::GROUP;
/// let org_type = EntityType::ORGANIZATION;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// Represents an individual user in the Kintone system
    USER,
    /// Represents a group of users
    GROUP,
    /// Represents an organizational unit
    ORGANIZATION,
}

/// Represents a generic entity in Kintone's user management system.
///
/// An entity can be a user, group, or organization, identified by its type and code.
/// This structure is commonly used in permission settings, assignee specifications,
/// and other contexts where you need to reference a Kintone entity.
///
/// # Fields
///
/// * `entity_type` - The type of entity (USER, GROUP, or ORGANIZATION)
/// * `code` - The unique identifier code for the entity
///
/// # Examples
///
/// ```rust
/// use kintone::model::{Entity, EntityType};
///
/// // Create a user entity
/// let user_entity = Entity {
///     entity_type: EntityType::USER,
///     code: "john.doe".to_owned(),
/// };
///
/// // Create a group entity
/// let group_entity = Entity {
///     entity_type: EntityType::GROUP,
///     code: "development-team".to_owned(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub code: String,
}

/// Represents a user in the Kintone system.
///
/// This structure contains basic information about a Kintone user, including
/// their display name and unique code. Users can be assigned to records,
/// added to groups, and given various permissions within Kintone apps.
///
/// # Fields
///
/// * `name` - The display name of the user
/// * `code` - The unique identifier code for the user (typically their login name)
///
/// # Examples
///
/// ```rust
/// use kintone::model::User;
///
/// let user = User {
///     name: "John Doe".to_owned(),
///     code: "john.doe".to_owned(),
/// };
///
/// println!("User: {} ({})", user.name, user.code);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub code: String,
}

/// Represents a group in the Kintone system.
///
/// Groups are collections of users that can be managed together for permissions,
/// assignments, and notifications. Groups help organize users and simplify
/// access control management in Kintone applications.
///
/// # Fields
///
/// * `name` - The display name of the group
/// * `code` - The unique identifier code for the group
///
/// # Examples
///
/// ```rust
/// use kintone::model::Group;
///
/// let group = Group {
///     name: "Development Team".to_owned(),
///     code: "dev-team".to_owned(),
/// };
///
/// println!("Group: {} ({})", group.name, group.code);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub code: String,
}

/// Represents an organizational unit in the Kintone system.
///
/// Organizations represent hierarchical structures within your company or entity.
/// They can be used for access control, workflow routing, and organizational
/// reporting within Kintone applications.
///
/// # Fields
///
/// * `name` - The display name of the organization
/// * `code` - The unique identifier code for the organization
///
/// # Examples
///
/// ```rust
/// use kintone::model::Organization;
///
/// let org = Organization {
///     name: "Engineering Division".to_owned(),
///     code: "eng-div".to_owned(),
/// };
///
/// println!("Organization: {} ({})", org.name, org.code);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub name: String,
    pub code: String,
}

/// Represents metadata for a file stored in Kintone.
///
/// This structure contains information about files that have been uploaded to Kintone,
/// including their content type, unique file key, original filename, and size.
/// File bodies are typically used in attachment fields and file upload operations.
///
/// # Fields
///
/// * `file_key` - The unique identifier for the file in Kintone's storage system.
///   When you writing a record, only this field is required.
/// * `content_type` - The MIME type of the file (e.g., "image/jpeg", "application/pdf")
/// * `name` - The original filename when the file was uploaded
/// * `size` - The file size in bytes
///
/// # Examples
///
/// Using the builder pattern (recommended):
/// ```rust
/// use kintone::model::file_body;
///
/// let file_key = "abc123def456";
/// let file = file_body(file_key).build();
/// ```
///
/// Using direct struct initialization:
/// ```rust
/// use kintone::model::FileBody;
///
/// let file = FileBody {
///     file_key: "abc123def456".to_owned(),
///     content_type: Some("image/jpeg".to_owned()),
///     name: Some("photo.jpg".to_owned()),
///     size: Some(1024),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBody {
    pub file_key: String,
    pub content_type: Option<String>,
    pub name: Option<String>,
    #[serde(with = "option_stringified")]
    pub size: Option<usize>,
}

/// Creates a new file body builder.
///
/// # Arguments
/// * `file_key` - The unique identifier for the file in Kintone's storage system
///
/// # Examples
/// ```rust
/// use kintone::model::file_body;
///
/// let file = file_body("abc123def456")
///     .content_type("image/jpeg")
///     .name("photo.jpg")
///     .size(1024)
///     .build();
/// ```
pub fn file_body(file_key: impl Into<String>) -> FileBodyBuilder {
    FileBodyBuilder {
        file_body: FileBody {
            file_key: file_key.into(),
            content_type: None,
            name: None,
            size: None,
        },
    }
}

/// Builder for creating [`FileBody`].
#[derive(Clone)]
pub struct FileBodyBuilder {
    file_body: FileBody,
}

impl FileBodyBuilder {
    /// Sets the MIME type of the file.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.file_body.content_type = Some(content_type.into());
        self
    }

    /// Sets the original filename.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.file_body.name = Some(name.into());
        self
    }

    /// Sets the file size in bytes.
    pub fn size(mut self, size: usize) -> Self {
        self.file_body.size = Some(size);
        self
    }

    /// Builds the final [`FileBody`].
    pub fn build(self) -> FileBody {
        self.file_body
    }
}

impl From<FileBodyBuilder> for FileBody {
    fn from(builder: FileBodyBuilder) -> Self {
        builder.build()
    }
}

/// Represents the sort order for query results.
///
/// This enum is used to specify whether records should be sorted in ascending
/// or descending order when querying Kintone records. It's commonly used in
/// record retrieval operations and other sorted data requests.
///
/// # Variants
///
/// * `Asc` - Ascending order (A-Z, 0-9, oldest to newest)
/// * `Desc` - Descending order (Z-A, 9-0, newest to oldest)
///
/// # Examples
///
/// ```rust
/// use kintone::model::Order;
///
/// let ascending = Order::Asc;
/// let descending = Order::Desc;
///
/// // The Display trait converts to lowercase string representation
/// assert_eq!(ascending.to_string(), "asc");
/// assert_eq!(descending.to_string(), "desc");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    /// Ascending order (A-Z, 0-9, oldest to newest)
    Asc,
    /// Descending order (Z-A, 9-0, newest to oldest)
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
