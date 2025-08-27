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
/// use kintone::model::space::ThreadComment;
///
/// let comment = ThreadComment {
///     text: "This is a thread comment with a mention".to_string(),
///     mentions: vec![
///         Entity {
///             entity_type: EntityType::USER,
///             code: "user1".to_string(),
///         }
///     ],
/// };
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
