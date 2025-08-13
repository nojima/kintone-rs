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
pub struct ThreadComment {
    /// The text content of the comment
    pub text: String,
    /// List of entities mentioned in the comment
    pub mentions: Vec<Entity>,
}
