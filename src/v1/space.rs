//! # Kintone Space API
//!
//! This module provides functions for interacting with Kintone's space-related REST API endpoints.
//! It includes operations for managing spaces, threads, and thread comments.
//!
//! ## Available Operations
//!
//! ### Space Management
//! - [`add_space`] - Create a new space (public and single-thread)
//! - [`delete_space`] - Delete an existing space
//!
//! ### Thread Management
//! - [`add_thread`] - Create a new thread in a space
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
//! // Create a new thread in the space
//! let thread_response = kintone::v1::space::add_thread(space_response.id, "Discussion Thread").send(&client)?;
//! println!("Created thread with ID: {}", thread_response.id);
//!
//! // Add a comment to a thread
//! use kintone::model::space::thread_comment;
//! let comment = thread_comment("Hello from the thread!").build();
//! let comment_response = kintone::v1::space::add_thread_comment(
//!     space_response.id, thread_response.id, comment,
//! ).send(&client)?;
//! println!("Added comment with ID: {}", comment_response.id);
//!
//! // Later, delete the space when no longer needed
//! kintone::v1::space::delete_space(space_response.id).send(&client)?;
//! println!("Space deleted successfully");
//! // Note: Deleted spaces can be restored within 14 days by cybozu.com common administrators
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

/// Deletes an existing space in Kintone.
///
/// This function creates a request to delete a space with the specified ID.
/// Once deleted, the space and all its content (threads, comments, etc.) will be removed from active use.
///
/// **Important**: This API requires space administrator permissions.
///
/// **Note**: Deleted spaces can be restored by cybozu.com administrators within 14 days
/// of deletion using the space recovery function in the kintone system administration.
/// After 14 days, the space becomes permanently unrecoverable.
///
/// # Arguments
/// * `id` - The ID of the space to delete
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::space::delete_space(123).send(&client)?;
/// println!("Space deleted successfully");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/delete-space/>
pub fn delete_space(id: u64) -> DeleteSpaceRequest {
    DeleteSpaceRequest {
        builder: RequestBuilder::new(http::Method::DELETE, "/v1/space.json"),
        body: DeleteSpaceRequestBody { id },
    }
}

#[must_use]
pub struct DeleteSpaceRequest {
    builder: RequestBuilder,
    body: DeleteSpaceRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSpaceRequestBody {
    id: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSpaceResponse {
    // Empty response body
}

impl DeleteSpaceRequest {
    /// Sends the request to delete the space.
    ///
    /// # Returns
    /// A Result containing the DeleteSpaceResponse (empty) on success, or an ApiError.
    ///
    /// # Authentication
    /// This API requires space administrator permissions.
    ///
    /// # Recovery Information
    /// Deleted spaces can be restored by cybozu.com common administrators within 14 days
    /// of deletion. After this period, the space becomes permanently unrecoverable.
    /// See: <https://jp.cybozu.help/k/ja/space/delete_restore/restore_space.html>
    pub fn send(self, client: &KintoneClient) -> Result<DeleteSpaceResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

/// Creates a new thread in a Kintone space.
///
/// This function creates a request to add a new thread with the specified name to a space.
/// Threads can only be created in spaces where "Use space portal and multiple threads"
/// is enabled in the space settings.
///
/// **Important**: This API requires space viewing permissions. For private spaces or guest spaces,
/// only space members can execute this operation.
///
/// **Note**: Thread creation notifications will be sent to all space members as "All" notifications.
///
/// # Arguments
/// * `space` - The ID of the space to create the thread in
/// * `name` - The name of the thread to create (1 to 128 characters)
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::space::add_thread(123, "Project Discussion").send(&client)?;
/// println!("Created thread with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-thread/>
pub fn add_thread(space: u64, name: impl Into<String>) -> AddThreadRequest {
    AddThreadRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/space/thread.json"),
        body: AddThreadRequestBody {
            space,
            name: name.into(),
        },
    }
}

#[must_use]
pub struct AddThreadRequest {
    builder: RequestBuilder,
    body: AddThreadRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadRequestBody {
    space: u64,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadResponse {
    #[serde(with = "stringified")]
    pub id: u64,
}

impl AddThreadRequest {
    /// Sends the request to create the thread.
    ///
    /// # Returns
    /// A Result containing the AddThreadResponse with the new thread ID, or an ApiError.
    ///
    /// # Authentication
    /// This API requires space viewing permissions. For private/guest spaces,
    /// only space members can execute this operation.
    pub fn send(self, client: &KintoneClient) -> Result<AddThreadResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

//-----------------------------------------------------------------------------

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
/// use kintone::model::space::{ThreadComment, thread_comment};
///
/// // Using the builder pattern (recommended)
/// let comment = thread_comment("This is a thread comment.")
///     .build();
///
/// // Or construct directly
/// let comment = ThreadComment {
///     text: "This is a thread comment.".to_owned(),
///     mentions: vec![],
///     files: vec![],
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
