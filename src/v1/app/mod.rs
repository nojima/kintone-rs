//! # Kintone App API
//!
//! This module provides functions for interacting with Kintone's app-related REST API endpoints.
//! It includes operations for creating and managing apps in the preview environment.
//!
//! ## Available Operations
//!
//! ### App Management
//! - [`add_app`] - Create a new app in the preview environment
//!
//! ## Usage Pattern
//!
//! All functions in this module follow the builder pattern:
//!
//! ```rust
//! # use kintone::client::{Auth, KintoneClient};
//! # use kintone::v1::app;
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_string(), "pass".to_string()));
//! let response = app::add_app("My App")
//!     .space(123)
//!     .thread(456)
//!     .send(&client)?;
//! println!("Created app with ID: {}", response.app);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! **Note**: App APIs require username/password authentication and cannot use API tokens.

use serde::{Deserialize, Serialize};

use crate::client::{KintoneClient, RequestBuilder};
use crate::error::ApiError;
use crate::internal::serde_helper::stringified;

/// Creates a new app in the preview environment.
///
/// This function creates a request to add a new app to Kintone's preview environment.
/// The preview environment is a temporary location where app information is stored 
/// before being deployed to the production environment.
///
/// **Important**: This API requires username/password authentication and cannot use API tokens.
///
/// # Arguments
/// * `name` - The name of the app (up to 64 characters)
/// * `space` (optional) - The space ID where the app should be created
/// * `thread` (optional) - The thread ID within the space where the app should be created
///
/// # Example
/// ```rust
/// let response = add_app("Project Management App")
///     .space(10) // optional
///     .thread(11) // optional
///     .send(&client)?;
/// println!("Created app with ID: {}", response.app);
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/apps/add-app/>
pub fn add_app(name: impl Into<String>) -> AddAppRequest {
    let builder = RequestBuilder::new(http::Method::POST, "/v1/preview/app.json");
    AddAppRequest {
        builder,
        body: AddAppRequestBody {
            name: name.into(),
            space: None,
            thread: None,
        },
    }
}

#[must_use]
pub struct AddAppRequest {
    builder: RequestBuilder,
    body: AddAppRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddAppRequestBody {
    name: String,
    space: Option<u64>,
    thread: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAppResponse {
    #[serde(with = "stringified")]
    pub app: u64,
    #[serde(with = "stringified")]
    pub revision: u64,
}

impl AddAppRequest {
    /// Sets the space ID where the app should be created.
    ///
    /// This is used when creating an app within a specific space.
    /// Both `space` and `thread` should be specified together.
    pub fn space(mut self, space: u64) -> Self {
        self.body.space = Some(space);
        self
    }

    /// Sets the thread ID within the space where the app should be created.
    ///
    /// This is used when creating an app within a specific thread in a space.
    /// Both `space` and `thread` should be specified together.
    pub fn thread(mut self, thread: u64) -> Self {
        self.body.thread = Some(thread);
        self
    }

    /// Sends the request to create the app.
    ///
    /// # Returns
    /// A Result containing the AddAppResponse with the app ID and revision, or an ApiError.
    ///
    /// # Authentication
    /// This API requires username/password authentication. API tokens cannot be used.
    pub fn send(self, client: &KintoneClient) -> Result<AddAppResponse, ApiError> {
        self.builder.send(client, self.body)
    }
}
