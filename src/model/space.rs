//! # Kintone Space Models
//!
//! This module provides data structures for working with Kintone spaces.

use serde::{Deserialize, Serialize};

use crate::model::Entity;

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
#[serde(rename = "camelCase")]
pub struct ThreadComment {
    /// The text content of the comment
    pub text: String,
    /// List of entities mentioned in the comment
    pub mentions: Vec<Entity>,
    /// List of attachment files
    pub files: Vec<ThreadCommentFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
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
#[derive(Debug, Clone)]
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
