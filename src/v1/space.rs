//! # Kintone Space API
//!
//! This module provides functions for interacting with Kintone's space-related REST API endpoints.
//! It includes operations for managing spaces, threads, and thread comments.

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::space::ThreadComment;

/// Adds a new comment to a specific thread in a Kintone space.
///
/// This function creates a request to add a comment to a thread within a space.
/// The comment can include text and mentions of other users.
///
/// # Arguments
/// * `space` - The ID of the Kintone space
/// * `thread` - The ID of the thread to add the comment to
/// * `comment` - The comment data including text and mentions
///
/// # Example
/// ```rust
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::space::ThreadComment;
/// let comment = ThreadComment {
///     text: "This is a thread comment.".to_owned(),
///     mentions: vec![],
/// };
/// let response = kintone::v1::space::add_thread_comment(123, 456, comment).send(&client)?;
/// println!("Added thread comment with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-thread-comment/>
pub fn add_thread_comment(
    space: u64,
    thread: u64,
    comment: ThreadComment,
) -> AddThreadCommentRequest {
    AddThreadCommentRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/space/thread/comment.json"),
        body: AddThreadCommentRequestBody {
            space,
            thread,
            comment,
        },
    }
}

#[must_use]
pub struct AddThreadCommentRequest {
    builder: RequestBuilder,
    body: AddThreadCommentRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadCommentRequestBody {
    space: u64,
    thread: u64,
    comment: ThreadComment,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadCommentResponse {
    #[serde(with = "stringified")]
    pub id: u64,
}

impl AddThreadCommentRequest {
    pub fn send(self, client: &KintoneClient) -> Result<AddThreadCommentResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
