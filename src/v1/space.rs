//! # Kintone Space API
//!
//! This module provides functions for interacting with Kintone's space-related REST API endpoints.
//! It includes operations for managing spaces, threads, and thread comments.
//!
//! ## Available Operations
//!
//! ### Space Management
//! - [`add_space`] - Create a new space (public and single-thread)
//!
//! ### Thread Comments
//! - [`add_thread_comment`] - Add a comment to a thread within a space
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```no_run
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
//! // Create a new space
//! let space_response = kintone::v1::space::add_space("My New Space").send(&client)?;
//! println!("Created space with ID: {}", space_response.id);
//!
//! // Add a comment to a thread
//! use kintone::model::space::ThreadComment;
//! let comment = ThreadComment {
//!     text: "Hello from the thread!".to_owned(),
//!     mentions: vec![],
//! };
//! let comment_response = kintone::v1::space::add_thread_comment(space_response.id, 1, comment).send(&client)?;
//! println!("Added comment with ID: {}", comment_response.id);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: Space APIs require appropriate space permissions.

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::space::ThreadComment;

/// Creates a new space in Kintone.
///
/// This function creates a request to add a new space with the specified name.
/// The created space will be a public space and single-thread space by default.
///
/// **Important**: This API requires space creation permissions.
///
/// **Note**: This is an experimental API (API Lab) and may change in the future.
/// To use this API, you need to enable "検討中の新機能" (experimental features)
/// in your Kintone settings.
///
/// # Arguments
/// * `name` - The name of the space to create
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::space::add_space("My New Project Space").send(&client)?;
/// println!("Created space with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/api-lab/rest-api/spaces/add-space-by-name/>
pub fn add_space(name: impl Into<String>) -> AddSpaceRequest {
    AddSpaceRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/space.json"),
        body: AddSpaceRequestBody { name: name.into() },
    }
}

#[must_use]
pub struct AddSpaceRequest {
    builder: RequestBuilder,
    body: AddSpaceRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSpaceRequestBody {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSpaceResponse {
    #[serde(with = "stringified")]
    pub id: u64,
}

impl AddSpaceRequest {
    /// Sends the request to create the space.
    ///
    /// # Returns
    /// A Result containing the AddSpaceResponse with the new space ID, or an ApiError.
    ///
    /// # Authentication
    /// This API requires space creation permissions.
    pub fn send(self, client: &KintoneClient) -> Result<AddSpaceResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

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
/// ```no_run
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
