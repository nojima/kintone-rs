use serde::{Deserialize, Serialize};

use crate::models::Entity;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadComment {
    pub text: String,
    pub mentions: Vec<Entity>,
}
