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
//! - [`get_space`], [`update_space`], [`add_space_from_template`] - Read, configure and create spaces
//! - [`get_space_members`], [`update_space_members`] - Manage memberships
//! - [`get_space_statistics`] - Read usage statistics (wide course)
//! - [`add_guest_users`], [`delete_guest_users`], [`update_guest_members`] - Manage guests
//!
//! ### Thread Management
//! - [`add_thread`] - Create a new thread in a space
//! - [`add_thread_comment`] - Add a comment to a thread within a space
//! - [`update_thread`] - Update the name and body of a thread
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

use super::app::settings::EmptyResponse;
use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;
use crate::model::User;
use crate::model::space::*;

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

/// Get space.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-space/>
pub fn get_space(id: u64) -> GetSpaceRequest {
    GetSpaceRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/space.json").query("id", id),
    }
}

#[must_use]
pub struct GetSpaceRequest {
    builder: RequestBuilder,
}

impl GetSpaceRequest {
    pub fn send(self, client: &KintoneClient) -> Result<GetSpaceResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpaceResponse {
    #[serde(with = "stringified")]
    pub id: u64,
    pub name: String,
    #[serde(with = "stringified")]
    pub default_thread: u64,
    pub is_private: bool,
    pub creator: User,
    pub modifier: User,
    #[serde(with = "stringified")]
    pub member_count: u64,
    pub cover_type: SpaceCoverType,
    pub cover_key: String,
    pub cover_url: String,
    #[serde(default)]
    pub body: Option<String>,
    pub use_multi_thread: bool,
    pub is_guest: bool,
    pub attached_apps: Vec<AttachedApp>,
    pub fixed_member: bool,
    #[serde(default)]
    pub show_announcement: Option<bool>,
    #[serde(default)]
    pub show_thread_list: Option<bool>,
    #[serde(default)]
    pub show_app_list: Option<bool>,
    #[serde(default)]
    pub show_member_list: Option<bool>,
    #[serde(default)]
    pub show_related_link_list: Option<bool>,
    pub permissions: SpacePermissions,
}

pub type UpdateSpaceResponse = EmptyResponse;

/// Update space.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space/>
pub fn update_space(id: u64) -> UpdateSpaceRequest {
    UpdateSpaceRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/space.json"),
        body: UpdateSpaceRequestBody {
            id,
            name: None,
            is_private: None,
            fixed_member: None,
            use_multi_thread: None,
            show_announcement: None,
            show_thread_list: None,
            show_app_list: None,
            show_member_list: None,
            show_related_link_list: None,
            permissions: None,
        },
    }
}

#[must_use]
pub struct UpdateSpaceRequest {
    builder: RequestBuilder,
    body: UpdateSpaceRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceRequestBody {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_multi_thread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_announcement: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_thread_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_app_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_member_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_related_link_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SpacePermissions>,
}

impl UpdateSpaceRequest {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.body.is_private = Some(is_private);
        self
    }

    pub fn fixed_member(mut self, fixed_member: bool) -> Self {
        self.body.fixed_member = Some(fixed_member);
        self
    }

    pub fn use_multi_thread(mut self, use_multi_thread: bool) -> Self {
        self.body.use_multi_thread = Some(use_multi_thread);
        self
    }

    pub fn show_announcement(mut self, show_announcement: bool) -> Self {
        self.body.show_announcement = Some(show_announcement);
        self
    }

    pub fn show_thread_list(mut self, show_thread_list: bool) -> Self {
        self.body.show_thread_list = Some(show_thread_list);
        self
    }

    pub fn show_app_list(mut self, show_app_list: bool) -> Self {
        self.body.show_app_list = Some(show_app_list);
        self
    }

    pub fn show_member_list(mut self, show_member_list: bool) -> Self {
        self.body.show_member_list = Some(show_member_list);
        self
    }

    pub fn show_related_link_list(mut self, show_related_link_list: bool) -> Self {
        self.body.show_related_link_list = Some(show_related_link_list);
        self
    }

    pub fn permissions(mut self, permissions: SpacePermissions) -> Self {
        self.body.permissions = Some(permissions);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type AddSpaceFromTemplateResponse = AddSpaceResponse;

/// Add space from template.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-space-from-template/>
pub fn add_space_from_template(id: u64, name: impl Into<String>) -> AddSpaceFromTemplateRequest {
    AddSpaceFromTemplateRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/template/space.json"),
        body: AddSpaceFromTemplateRequestBody {
            id,
            name: name.into(),
            members: Vec::new(),
            is_private: None,
            is_guest: None,
            fixed_member: None,
        },
    }
}

#[must_use]
pub struct AddSpaceFromTemplateRequest {
    builder: RequestBuilder,
    body: AddSpaceFromTemplateRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddSpaceFromTemplateRequestBody {
    id: u64,
    name: String,
    members: Vec<SpaceMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_guest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_member: Option<bool>,
}

impl AddSpaceFromTemplateRequest {
    pub fn members(mut self, values: impl IntoIterator<Item = SpaceMember>) -> Self {
        self.body.members = values.into_iter().collect();
        self
    }

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.body.is_private = Some(is_private);
        self
    }

    pub fn is_guest(mut self, is_guest: bool) -> Self {
        self.body.is_guest = Some(is_guest);
        self
    }

    pub fn fixed_member(mut self, fixed_member: bool) -> Self {
        self.body.fixed_member = Some(fixed_member);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<AddSpaceFromTemplateResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type UpdateSpaceBodyResponse = EmptyResponse;

/// Update space body.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space-body/>
pub fn update_space_body(id: u64, body: impl Into<String>) -> UpdateSpaceBodyRequest {
    UpdateSpaceBodyRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/space/body.json"),
        body: UpdateSpaceBodyRequestBody {
            id,
            body: body.into(),
        },
    }
}

#[must_use]
pub struct UpdateSpaceBodyRequest {
    builder: RequestBuilder,
    body: UpdateSpaceBodyRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceBodyRequestBody {
    id: u64,
    body: String,
}

impl UpdateSpaceBodyRequest {
    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceBodyResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get space members.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-space-members/>
pub fn get_space_members(id: u64) -> GetSpaceMembersRequest {
    GetSpaceMembersRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/space/members.json").query("id", id),
    }
}

#[must_use]
pub struct GetSpaceMembersRequest {
    builder: RequestBuilder,
}

impl GetSpaceMembersRequest {
    pub fn send(self, client: &KintoneClient) -> Result<GetSpaceMembersResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpaceMembersResponse {
    pub members: Vec<SpaceMember>,
}

pub type UpdateSpaceMembersResponse = EmptyResponse;

/// Update space members.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space-members/>
pub fn update_space_members(id: u64) -> UpdateSpaceMembersRequest {
    UpdateSpaceMembersRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/space/members.json"),
        body: UpdateSpaceMembersRequestBody {
            id,
            members: Vec::new(),
        },
    }
}

#[must_use]
pub struct UpdateSpaceMembersRequest {
    builder: RequestBuilder,
    body: UpdateSpaceMembersRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceMembersRequestBody {
    id: u64,
    members: Vec<SpaceMember>,
}

impl UpdateSpaceMembersRequest {
    pub fn members(mut self, values: impl IntoIterator<Item = SpaceMember>) -> Self {
        self.body.members = values.into_iter().collect();
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceMembersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Get space statistics.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-spaces-statistics/>
/// Available with the kintone wide course only. Use a regular space client.
pub fn get_space_statistics() -> GetSpaceStatisticsRequest {
    GetSpaceStatisticsRequest {
        builder: RequestBuilder::new(http::Method::GET, "/v1/spaces/statistics.json"),
    }
}

#[must_use]
pub struct GetSpaceStatisticsRequest {
    builder: RequestBuilder,
}

impl GetSpaceStatisticsRequest {
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<GetSpaceStatisticsResponse, ApiError> {
        self.builder.call(client)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpaceStatisticsResponse {
    pub spaces: Vec<SpaceStatistic>,
}

pub type UpdateThreadResponse = EmptyResponse;

/// Update thread.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-thread/>
pub fn update_thread(id: u64) -> UpdateThreadRequest {
    UpdateThreadRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/space/thread.json"),
        body: UpdateThreadRequestBody {
            id,
            name: None,
            body: None,
        },
    }
}

#[must_use]
pub struct UpdateThreadRequest {
    builder: RequestBuilder,
    body: UpdateThreadRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateThreadRequestBody {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

impl UpdateThreadRequest {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.body = Some(body.into());
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateThreadResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type AddGuestUsersResponse = EmptyResponse;

/// Add guest users.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-guests/>
pub fn add_guest_users() -> AddGuestUsersRequest {
    AddGuestUsersRequest {
        builder: RequestBuilder::new(http::Method::POST, "/v1/guests.json"),
        body: AddGuestUsersRequestBody { guests: Vec::new() },
    }
}

#[must_use]
pub struct AddGuestUsersRequest {
    builder: RequestBuilder,
    body: AddGuestUsersRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddGuestUsersRequestBody {
    guests: Vec<GuestUser>,
}

impl AddGuestUsersRequest {
    pub fn guests(mut self, values: impl IntoIterator<Item = GuestUser>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<AddGuestUsersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type DeleteGuestUsersResponse = EmptyResponse;

/// Delete guest users.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/delete-guests/>
pub fn delete_guest_users() -> DeleteGuestUsersRequest {
    DeleteGuestUsersRequest {
        builder: RequestBuilder::new(http::Method::DELETE, "/v1/guests.json"),
        body: DeleteGuestUsersRequestBody { guests: Vec::new() },
    }
}

#[must_use]
pub struct DeleteGuestUsersRequest {
    builder: RequestBuilder,
    body: DeleteGuestUsersRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteGuestUsersRequestBody {
    guests: Vec<String>,
}

impl DeleteGuestUsersRequest {
    pub fn guests(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<DeleteGuestUsersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Replaces the guest members. Use a client configured with this guest space ID.
/// The request body and client guest space ID must refer to the same space.
pub type UpdateGuestMembersResponse = EmptyResponse;

/// Update guest members.
///
/// Reference: <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-guest-members/>
pub fn update_guest_members(id: u64) -> UpdateGuestMembersRequest {
    UpdateGuestMembersRequest {
        builder: RequestBuilder::new(http::Method::PUT, "/v1/space/guests.json"),
        body: UpdateGuestMembersRequestBody {
            id,
            guests: Vec::new(),
        },
    }
}

#[must_use]
pub struct UpdateGuestMembersRequest {
    builder: RequestBuilder,
    body: UpdateGuestMembersRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateGuestMembersRequestBody {
    id: u64,
    guests: Vec<String>,
}

impl UpdateGuestMembersRequest {
    pub fn guests(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<UpdateGuestMembersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
