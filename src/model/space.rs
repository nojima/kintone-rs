//! # Kintone Space Models
//!
//! This module provides data structures for working with Kintone spaces.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::stringified;

use crate::model::{Entity, User};

/// Represents a comment to be posted to a thread in a Kintone space.
///
/// A thread comment can include text content and mentions of users, groups, or organizations.
///
/// # Examples
///
/// ```rust
/// use kintone::model::{Entity, EntityType};
/// use kintone::model::space::thread_comment;
///
/// // Using the builder pattern
/// let comment = thread_comment("This is a thread comment with a mention")
///     .mention(Entity {
///         entity_type: EntityType::USER,
///         code: "user1".to_string(),
///     })
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadComment {
    /// The text content of the comment
    pub text: String,
    /// List of entities mentioned in the comment
    pub mentions: Vec<Entity>,
    /// List of attachment files
    pub files: Vec<ThreadCommentFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentFile {
    /// The fileKey of the attachment file.
    pub file_key: String,
    /// Width can be specified if the attachment file is an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// Creates a new thread comment builder with the specified text.
///
/// This function creates a builder that can be used to construct a [`ThreadComment`]
/// with optional mentions and file attachments using method chaining.
///
/// # Arguments
/// * `text` - The text content of the comment
///
/// # Examples
/// ```rust
/// use kintone::model::{Entity, EntityType};
/// use kintone::model::space::{thread_comment, ThreadCommentFile};
///
/// let comment = thread_comment("Hello, this is a comment!")
///     .mention(Entity {
///         entity_type: EntityType::USER,
///         code: "user1".to_string(),
///     })
///     .file(ThreadCommentFile {
///         file_key: "file123".to_string(),
///         width: Some(300),
///     })
///     .build();
/// ```
pub fn thread_comment(text: impl Into<String>) -> ThreadCommentBuilder {
    ThreadCommentBuilder {
        text: text.into(),
        mentions: Vec::new(),
        files: Vec::new(),
    }
}

/// Builder for creating a [`ThreadComment`] with optional mentions and files.
#[derive(Clone)]
pub struct ThreadCommentBuilder {
    text: String,
    mentions: Vec<Entity>,
    files: Vec<ThreadCommentFile>,
}

impl ThreadCommentBuilder {
    /// Adds a mention to the comment.
    ///
    /// # Arguments
    /// * `entity` - The entity to mention (user, group, or organization)
    pub fn mention(mut self, entity: Entity) -> Self {
        self.mentions.push(entity);
        self
    }

    /// Adds multiple mentions to the comment.
    ///
    /// # Arguments
    /// * `entities` - The entities to mention
    pub fn mentions(mut self, entities: impl IntoIterator<Item = Entity>) -> Self {
        self.mentions.extend(entities);
        self
    }

    /// Adds a file attachment to the comment.
    ///
    /// # Arguments
    /// * `file` - The file attachment information
    pub fn file(mut self, file: ThreadCommentFile) -> Self {
        self.files.push(file);
        self
    }

    /// Adds multiple file attachments to the comment.
    ///
    /// # Arguments
    /// * `files` - The file attachments to add
    pub fn files(mut self, files: impl IntoIterator<Item = ThreadCommentFile>) -> Self {
        self.files.extend(files);
        self
    }

    /// Builds and returns the final [`ThreadComment`].
    pub fn build(self) -> ThreadComment {
        ThreadComment {
            text: self.text,
            mentions: self.mentions,
            files: self.files,
        }
    }
}

impl From<ThreadCommentBuilder> for ThreadComment {
    fn from(builder: ThreadCommentBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EntityType;

    #[test]
    fn serialize_thread_comment() {
        let comment = thread_comment("hello")
            .mention(Entity {
                entity_type: EntityType::USER,
                code: "takahashi".to_owned(),
            })
            .file(ThreadCommentFile {
                file_key: "c15b3870-7505-4ab6-9d8d-b9bdbc74f5d6".to_owned(),
                width: Some(500),
            })
            .build();
        let actual = serde_json::to_value(&comment).unwrap();
        let expected = serde_json::json!({
            "text": "hello",
            "mentions": [ { "code": "takahashi", "type": "USER" } ],
            "files": [ { "fileKey": "c15b3870-7505-4ab6-9d8d-b9bdbc74f5d6", "width": 500 } ]
        });
        assert_eq!(actual, expected);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpaceCoverType {
    #[default]
    Preset,
    Blob,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateAppPermission {
    #[default]
    Everyone,
    Admin,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacePermissions {
    pub create_app: CreateAppPermission,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMember {
    pub entity: Entity,
    pub is_admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", skip_serializing)]
    pub is_implicit: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedApp {
    #[serde(with = "stringified")]
    pub thread_id: u64,
    #[serde(with = "stringified")]
    pub app_id: u64,
    pub code: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<FixedOffset>,
    pub creator: User,
    pub modified_at: DateTime<FixedOffset>,
    pub modifier: User,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceStatistic {
    #[serde(with = "stringified")]
    pub id: u64,
    pub name: String,
    #[serde(with = "stringified")]
    pub administrator_count: u64,
    #[serde(with = "stringified")]
    pub member_count: u64,
    pub is_private: bool,
    pub is_guest: bool,
    pub creator: User,
    pub created_at: DateTime<FixedOffset>,
    pub modifier: User,
    pub modified_at: DateTime<FixedOffset>,
}

/// A guest account to create. Optional profile fields are omitted when unset.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestUser {
    pub code: String,
    pub password: String,
    pub timezone: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sur_name_reading: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name_reading: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callto: Option<String>,
}

impl std::fmt::Debug for GuestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuestUser")
            .field("code", &self.code)
            .field("password", &"<hidden>")
            .field("timezone", &self.timezone)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl GuestUser {
    pub fn new(
        code: impl Into<String>,
        password: impl Into<String>,
        timezone: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            password: password.into(),
            timezone: timezone.into(),
            name: name.into(),
            locale: None,
            image: None,
            sur_name_reading: None,
            given_name_reading: None,
            company: None,
            division: None,
            phone: None,
            callto: None,
        }
    }
}
