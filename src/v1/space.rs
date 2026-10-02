//! # Kintone Space API
//!
//! This module provides functions for interacting with Kintone's space-related REST API endpoints.
//! It includes operations for managing spaces, threads, and thread comments.
//!
//! ## Available Operations
//!
//! ### Space Management
//! - [`add_space`] - Create a new space (public and single-thread)
//! - [`get_space`] - Retrieve space information
//! - [`update_space`] - Update space settings
//! - [`delete_space`] - Delete an existing space
//! - [`add_space_from_template`] - Create a space from a template
//! - [`update_space_body`] - Update the space portal body
//! - [`get_space_members`] - Retrieve space memberships
//! - [`update_space_members`] - Update space memberships
//! - [`get_space_statistics`] - Retrieve space usage statistics (wide course)
//!
//! ### Thread Management
//! - [`add_thread`] - Create a thread in a space
//! - [`update_thread`] - Update a thread's name and body
//! - [`add_thread_comment`] - Add a comment to a thread
//!
//! ### Guest Management
//! - [`add_guest_users`] - Create guest accounts
//! - [`delete_guest_users`] - Delete guest accounts
//! - [`update_guest_members`] - Update guest space memberships
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

/// Retrieves information about a Kintone space.
///
/// This function creates a request to get a space's name, portal body, members count, attached apps
/// and visibility settings.
///
/// **Required Permissions:** This API requires space viewing permissions. Private spaces can be
/// accessed only by their members. Use username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::space::get_space(123).send(&client)?;
/// println!("{}: {} members", response.name, response.member_count);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-space/>
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
    /// Sends the request to retrieve information about a Kintone space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetSpaceResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space viewing permissions. Private spaces can be accessed only by their
    /// members. Use username/password authentication; API tokens cannot be used.
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

/// Updates the settings of a Kintone space.
///
/// This function creates a request to update a space's name, visibility, portal components and app
/// creation permissions. Properties not supplied to the builder are omitted from the request.
///
/// **Required Permissions:** This API requires space administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space
///
/// # Optional Parameters
///
/// * `name` - Sets the space name
/// * `is_private` - Sets whether the space is private
/// * `fixed_member` - Sets whether members are prevented from leaving or unfollowing the space
/// * `use_multi_thread` - Enables multiple threads when true; false leaves the current setting unchanged
/// * `show_announcement` - Sets whether announcements are displayed in the space portal
/// * `show_thread_list` - Sets whether the thread list is displayed in the space portal
/// * `show_app_list` - Sets whether the app list is displayed in the space portal
/// * `show_member_list` - Sets whether the member list is displayed in the space portal
/// * `show_related_link_list` - Sets whether related links are displayed in the space portal
/// * `permissions` - Sets who can create apps in the space
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::space::{CreateAppPermission, SpacePermissions};
///
/// kintone::v1::space::update_space(123)
///     .name("Project Team")
///     .is_private(true)
///     .permissions(SpacePermissions { create_app: CreateAppPermission::Admin })
///     .send(&client)?;
/// println!("Updated space settings");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space/>
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
    /// Sets the space name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// Sets whether the space is private.
    pub fn is_private(mut self, is_private: bool) -> Self {
        self.body.is_private = Some(is_private);
        self
    }

    /// Sets whether members are prevented from leaving or unfollowing the space.
    pub fn fixed_member(mut self, fixed_member: bool) -> Self {
        self.body.fixed_member = Some(fixed_member);
        self
    }

    /// Enables multiple threads when true; false leaves the current setting unchanged.
    pub fn use_multi_thread(mut self, use_multi_thread: bool) -> Self {
        self.body.use_multi_thread = Some(use_multi_thread);
        self
    }

    /// Sets whether announcements are displayed in the space portal.
    pub fn show_announcement(mut self, show_announcement: bool) -> Self {
        self.body.show_announcement = Some(show_announcement);
        self
    }

    /// Sets whether the thread list is displayed in the space portal.
    pub fn show_thread_list(mut self, show_thread_list: bool) -> Self {
        self.body.show_thread_list = Some(show_thread_list);
        self
    }

    /// Sets whether the app list is displayed in the space portal.
    pub fn show_app_list(mut self, show_app_list: bool) -> Self {
        self.body.show_app_list = Some(show_app_list);
        self
    }

    /// Sets whether the member list is displayed in the space portal.
    pub fn show_member_list(mut self, show_member_list: bool) -> Self {
        self.body.show_member_list = Some(show_member_list);
        self
    }

    /// Sets whether related links are displayed in the space portal.
    pub fn show_related_link_list(mut self, show_related_link_list: bool) -> Self {
        self.body.show_related_link_list = Some(show_related_link_list);
        self
    }

    /// Sets who can create apps in the space.
    pub fn permissions(mut self, permissions: SpacePermissions) -> Self {
        self.body.permissions = Some(permissions);
        self
    }

    /// Sends the request to update the settings of a Kintone space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateSpaceResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space administrator permissions. Use username/password authentication; API
    /// tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type AddSpaceFromTemplateResponse = AddSpaceResponse;

/// Creates a Kintone space from a space template.
///
/// This function creates a request to create a space from the specified template, with a name,
/// members and optional privacy or guest space settings.
///
/// **Required Permissions:** This API requires space creation permissions. Creating a guest space
/// requires guest space creation permissions. Use username/password authentication; API tokens
/// cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space template
/// * `name` - The name of the new space
/// * `members` - Members of the new space, supplied with `members()`
///
/// # Optional Parameters
///
/// * `is_private` - Sets whether the space is private
/// * `is_guest` - Sets whether the new space is a guest space
/// * `fixed_member` - Sets whether members are prevented from leaving or unfollowing the space
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::{Entity, EntityType};
/// use kintone::model::space::SpaceMember;
///
/// let member = SpaceMember {
///     entity: Entity { entity_type: EntityType::USER, code: "user1".to_owned() },
///     is_admin: true,
///     include_subs: None,
///     is_implicit: None,
/// };
/// let response = kintone::v1::space::add_space_from_template(10, "Project Team")
///     .members([member])
///     .is_private(true)
///     .send(&client)?;
/// println!("Created space with ID: {}", response.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-space-from-template/>
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
    /// Sets the complete space member list, including administrator assignments.
    pub fn members(mut self, values: impl IntoIterator<Item = SpaceMember>) -> Self {
        self.body.members = values.into_iter().collect();
        self
    }

    /// Sets whether the space is private.
    pub fn is_private(mut self, is_private: bool) -> Self {
        self.body.is_private = Some(is_private);
        self
    }

    /// Sets whether the new space is a guest space.
    pub fn is_guest(mut self, is_guest: bool) -> Self {
        self.body.is_guest = Some(is_guest);
        self
    }

    /// Sets whether members are prevented from leaving or unfollowing the space.
    pub fn fixed_member(mut self, fixed_member: bool) -> Self {
        self.body.fixed_member = Some(fixed_member);
        self
    }

    /// Sends the request to create a Kintone space from a space template.
    ///
    /// # Returns
    ///
    /// A Result containing the [`AddSpaceFromTemplateResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space creation permissions. Creating a guest space requires guest space
    /// creation permissions. Use username/password authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddSpaceFromTemplateResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type UpdateSpaceBodyResponse = EmptyResponse;

/// Updates the portal body of a Kintone space.
///
/// This function creates a request to replace the space portal body with the supplied HTML. The
/// server removes unsupported HTML tags and attributes.
///
/// **Required Permissions:** This API requires space administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space
/// * `body` - The new space portal body as an HTML string
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::space::update_space_body(123, "<p>Welcome to the project space.</p>")
///     .send(&client)?;
/// println!("Updated space portal body");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space-body/>
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
    /// Sends the request to update the portal body of a Kintone space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateSpaceBodyResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space administrator permissions. Use username/password authentication; API
    /// tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceBodyResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves the members of a Kintone space.
///
/// This function creates a request to get user, group and organization memberships, including space
/// administrator status and organization inheritance settings.
///
/// **Required Permissions:** This API requires space viewing permissions. Private spaces can be
/// accessed only by their members. Use username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::space::get_space_members(123).send(&client)?;
/// for member in response.members {
///     println!("{}: administrator = {}", member.entity.code, member.is_admin);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-space-members/>
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
    /// Sends the request to retrieve the members of a Kintone space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetSpaceMembersResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space viewing permissions. Private spaces can be accessed only by their
    /// members. Use username/password authentication; API tokens cannot be used.
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

/// Updates the members of a Kintone space.
///
/// This function creates a request to replace space memberships and administrator assignments.
/// Supply the complete member list with at least one space administrator.
///
/// **Required Permissions:** This API requires space administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the space
/// * `members` - Complete member list, supplied with `members()`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::{Entity, EntityType};
/// use kintone::model::space::SpaceMember;
///
/// let current = kintone::v1::space::get_space_members(123).send(&client)?;
/// let mut members = current.members;
/// members.push(SpaceMember {
///     entity: Entity { entity_type: EntityType::USER, code: "user2".to_owned() },
///     is_admin: false,
///     include_subs: None,
///     is_implicit: None,
/// });
/// kintone::v1::space::update_space_members(123)
///     .members(members)
///     .send(&client)?;
/// println!("Updated space members");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-space-members/>
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
    /// Sets the complete space member list, including administrator assignments.
    pub fn members(mut self, values: impl IntoIterator<Item = SpaceMember>) -> Self {
        self.body.members = values.into_iter().collect();
        self
    }

    /// Sends the request to update the members of a Kintone space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateSpaceMembersResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space administrator permissions. Use username/password authentication; API
    /// tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateSpaceMembersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Retrieves usage statistics for Kintone spaces.
///
/// This function creates a request to get space names, administrator and member counts, visibility
/// and ownership information. Results can be paginated.
///
/// **Note**: This API is available only on the kintone wide course.
///
/// **Required Permissions:** This API requires no additional administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Optional Parameters
///
/// * `offset` - Sets the number of results to skip (default: 0)
/// * `limit` - Sets the maximum number of results to retrieve (1-100, default: 100)
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// let response = kintone::v1::space::get_space_statistics()
///     .offset(0)
///     .limit(50)
///     .send(&client)?;
/// for space in response.spaces {
///     println!("{}: {} members", space.name, space.member_count);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/get-spaces-statistics/>
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
    /// Sets the number of results to skip (default: 0).
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder = self.builder.query("offset", offset);
        self
    }

    /// Sets the maximum number of results to retrieve (1-100, default: 100).
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder = self.builder.query("limit", limit);
        self
    }

    /// Sends the request to retrieve usage statistics for Kintone spaces.
    ///
    /// # Returns
    ///
    /// A Result containing the [`GetSpaceStatisticsResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires no additional administrator permissions. Use username/password authentication;
    /// API tokens cannot be used.
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

/// Updates the name and body of a Kintone space thread.
///
/// This function creates a request to update a thread's name or HTML body. Properties not supplied
/// to the builder are omitted from the request. Thread names can be changed only in spaces with
/// multiple threads.
///
/// **Required Permissions:** This API requires space administrator permissions or ownership of the
/// thread. Use username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the thread to update
///
/// # Optional Parameters
///
/// * `name` - Sets the thread name
/// * `body` - Sets the thread body as an HTML string
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::space::update_thread(456)
///     .name("Project discussion")
///     .body("<p>Share project updates here.</p>")
///     .send(&client)?;
/// println!("Updated thread");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-thread/>
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
    /// Sets the thread name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// Sets the thread body as an HTML string.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.body = Some(body.into());
        self
    }

    /// Sends the request to update the name and body of a Kintone space thread.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateThreadResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires space administrator permissions or ownership of the thread. Use
    /// username/password authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateThreadResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type AddGuestUsersResponse = EmptyResponse;

/// Creates guest user accounts in Kintone.
///
/// This function creates a request to register guest accounts with an email address, initial
/// password, timezone and display name. Use [`update_guest_members`] to add the accounts to a guest
/// space.
///
/// **Required Permissions:** This API requires kintone system administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `guests` - Guest account profiles to create, supplied with `guests()`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use kintone::model::space::GuestUser;
///
/// let guest = GuestUser::new(
///     "guest@example.com", "initial-password", "Asia/Tokyo", "Guest User",
/// );
/// kintone::v1::space::add_guest_users()
///     .guests([guest])
///     .send(&client)?;
/// println!("Created guest user");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/add-guests/>
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
    /// Sets the guest account profiles to create.
    pub fn guests(mut self, values: impl IntoIterator<Item = GuestUser>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    /// Sends the request to create guest user accounts in Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`AddGuestUsersResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddGuestUsersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

pub type DeleteGuestUsersResponse = EmptyResponse;

/// Deletes guest user accounts from Kintone.
///
/// This function creates a request to delete guest accounts identified by their email addresses.
/// These accounts are removed from kintone, including their guest space memberships.
///
/// **Required Permissions:** This API requires kintone system administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `guests` - Email addresses of guest accounts to delete, supplied with `guests()`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// kintone::v1::space::delete_guest_users()
///     .guests(["guest@example.com".to_owned()])
///     .send(&client)?;
/// println!("Deleted guest user");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/delete-guests/>
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
    /// Sets the email addresses of guest accounts to delete.
    pub fn guests(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    /// Sends the request to delete guest user accounts from Kintone.
    ///
    /// # Returns
    ///
    /// A Result containing the [`DeleteGuestUsersResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires kintone system administrator permissions. Use username/password
    /// authentication; API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<DeleteGuestUsersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}

/// Replaces the guest members. Use a client configured with this guest space ID.
/// The request body and client guest space ID must refer to the same space.
pub type UpdateGuestMembersResponse = EmptyResponse;

/// Updates the guest members of a Kintone guest space.
///
/// This function creates a request to replace a guest space's guest member list. Guest accounts
/// must already exist in kintone. Configure the client with the same guest space ID as the request.
///
/// **Required Permissions:** This API requires guest space administrator permissions. Use
/// username/password authentication; API tokens cannot be used.
///
/// # Arguments
///
/// * `id` - The ID of the guest space to update
/// * `guests` - Complete list of guest email addresses, supplied with `guests()`
///
/// # Example
///
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// let client = KintoneClient::builder(
///     "https://example.cybozu.com",
///     Auth::password("user".to_owned(), "pass".to_owned()),
/// ).guest_space_id(123).build();
/// kintone::v1::space::update_guest_members(123)
///     .guests(["guest@example.com".to_owned()])
///     .send(&client)?;
/// println!("Updated guest space members");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
///
/// <https://cybozu.dev/ja/kintone/docs/rest-api/spaces/update-guest-members/>
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
    /// Sets the complete list of guest email addresses belonging to the space.
    pub fn guests(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.body.guests = values.into_iter().collect();
        self
    }

    /// Sends the request to update the guest members of a Kintone guest space.
    ///
    /// # Returns
    ///
    /// A Result containing the [`UpdateGuestMembersResponse`], or an [`ApiError`].
    ///
    /// # Authentication
    ///
    /// This API requires guest space administrator permissions. Use username/password authentication;
    /// API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<UpdateGuestMembersResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
