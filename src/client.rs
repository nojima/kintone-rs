//! # Kintone HTTP Client
//!
//! This module provides the HTTP client for communicating with Kintone's REST API.
//! It handles authentication, request building, and response processing for all API calls.
//!
//! ## Key Components
//!
//! - [`KintoneClient`] - The main HTTP client for making API requests
//! - [`KintoneClientBuilder`] - Builder for configuring the client with custom options
//! - [`Auth`] - Authentication methods (API token or username/password)
//!
//! ## Authentication
//!
//! The client supports two authentication methods:
//!
//! ### API Token Authentication
//!
//! **Note**: API tokens can be generated from the Kintone app settings page.
//!
//! ```rust
//! use kintone::client::{Auth, KintoneClient};
//!
//! let client = KintoneClient::new(
//!     "https://your-domain.cybozu.com",
//!     Auth::api_token("your-api-token".to_owned())
//! );
//! ```
//!
//! ### Username/Password Authentication
//! ```rust
//! use kintone::client::{Auth, KintoneClient};
//!
//! let client = KintoneClient::new(
//!     "https://your-domain.cybozu.com",
//!     Auth::password("username".to_owned(), "password".to_owned())
//! );
//! ```
//!
//! ## Client Configuration
//!
//! For advanced configuration, use the builder pattern:
//!
//! ```rust
//! use kintone::client::{Auth, KintoneClientBuilder};
//!
//! let client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_owned())
//!     )
//!     .user_agent("MyApp/1.0")
//!     .build();
//! ```
//!
//! ## Middleware Configuration
//!
//! The client supports middleware through the builder pattern. Middleware can handle
//! cross-cutting concerns like retries, logging, and custom request/response processing.
//!
//! ### Retry Middleware
//!
//! Automatically retries failed requests with exponential backoff, particularly useful
//! for handling database lock errors (GAIA_DA02):
//!
//! ```rust
//! use std::time::Duration;
//! use kintone::client::{Auth, KintoneClientBuilder};
//! use kintone::middleware;
//!
//! let client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_owned())
//!     )
//!     .layer(middleware::RetryLayer::new(
//!         5,                              // max_attempts
//!         Duration::from_secs(1),         // initial_delay
//!         Duration::from_secs(8),         // max_delay
//!         None                            // Optional custom should_retry function
//!     ))
//!     .build();
//! ```
//!
//! ### Logging Middleware
//!
//! Logs detailed information about API requests and responses for debugging:
//!
//! ```rust
//! use kintone::client::{Auth, KintoneClientBuilder};
//! use kintone::middleware;
//!
//! let client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_owned())
//!     )
//!     .layer(middleware::LoggingLayer::new())
//!     .build();
//! ```
//!
//! ### Combined Middleware
//!
//! You can combine multiple middleware layers. The order in which layers are added determines
//! the execution order:
//!
//! **Important**: Layers are applied in a stack-like manner. The first layer added becomes the
//! outermost layer, and subsequent layers are nested inside. For requests, the execution flows
//! from the outermost layer to the innermost, and for responses, it flows back in reverse order.
//!
//! ```rust
//! use std::time::Duration;
//! use kintone::client::{Auth, KintoneClientBuilder};
//! use kintone::middleware;
//!
//! let client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::password("username".to_owned(), "password".to_owned())
//!     )
//!     .layer(middleware::RetryLayer::new(      // Layer A (outermost) - handles retries
//!         5,
//!         Duration::from_secs(1),
//!         Duration::from_secs(8),
//!         None
//!     ))
//!     .layer(middleware::LoggingLayer::new())  // Layer B (inner) - logs actual requests
//!     .build();
//!
//! // Execution flow:
//! // Request:  RetryLayer -> LoggingLayer -> HTTP Request
//! // Response: HTTP Response -> LoggingLayer -> RetryLayer
//! //
//! // This means:
//! // 1. RetryLayer receives the original request
//! // 2. LoggingLayer logs each actual request attempt (including retries)
//! // 3. HTTP request is sent
//! // 4. LoggingLayer logs each response (including retry attempts)
//! // 5. RetryLayer decides whether to retry or return the final response
//! //
//! // This order ensures that all retry attempts are logged, giving you
//! // complete visibility into what requests were actually sent.
//! ```
//!
//! ## Guest Space Support
//!
//! The client supports guest spaces by specifying a guest space ID during client creation.
//!
//! Each guest space requires its own `KintoneClient` instance. You cannot
//! use a single client to access multiple guest spaces or mix guest space and regular space operations.
//!
//! ### Creating a Client for Guest Space
//! ```rust
//! use kintone::client::{Auth, KintoneClientBuilder};
//!
//! // Client for guest space ID 123
//! let guest_client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_owned())
//!     )
//!     .guest_space_id(123)
//!     .build();
//!
//! // If you need to access a different guest space (ID 456), create another client
//! let another_guest_client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_owned())
//!     )
//!     .guest_space_id(456)
//!     .build();
//! ```

use std::fmt::Debug;
use std::io::Read;
use std::{collections::HashMap, io::Cursor};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::RngCore as _;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::ApiError;
use crate::middleware;

pub struct KintoneClient {
    base_url: url::Url,
    auth: Auth,
    guest_space_id: Option<u64>,
    handler: Box<dyn middleware::Handler>,
}

impl KintoneClient {
    pub fn new(base_url: &str, auth: Auth) -> Self {
        KintoneClientBuilder::new(base_url, auth).build()
    }

    pub(crate) fn run(
        &self,
        req: http::Request<middleware::RequestBody>,
    ) -> Result<http::Response<middleware::ResponseBody>, ApiError> {
        self.handler.handle(req)
    }
}

pub struct RequestHandler {
    http_client: ureq::Agent,
}

impl middleware::Handler for RequestHandler {
    fn handle(
        &self,
        req: http::Request<middleware::RequestBody>,
    ) -> Result<http::Response<middleware::ResponseBody>, ApiError> {
        let req = req.map(|body| body.into_ureq_body());
        let resp = self.http_client.run(req)?;
        if resp.status().as_u16() >= 400 {
            return Err(ApiError::from(resp));
        }
        let (parts, body) = resp.into_parts();
        let body = middleware::ResponseBody::from_ureq_body(body);
        Ok(http::Response::from_parts(parts, body))
    }
}

pub struct KintoneClientBuilder<L> {
    base_url: url::Url,
    auth: Auth,
    user_agent: Option<String>,
    guest_space_id: Option<u64>,
    layer: L,
}

impl KintoneClientBuilder<middleware::NoLayer> {
    pub fn new(base_url: &str, auth: Auth) -> Self {
        let base_url = url::Url::parse(base_url).unwrap();
        Self {
            base_url,
            auth,
            user_agent: None,
            guest_space_id: None,
            layer: middleware::NoLayer,
        }
    }
}

impl<L> KintoneClientBuilder<L> {
    pub fn layer<L2>(self, new_layer: L2) -> KintoneClientBuilder<middleware::Stack<L, L2>> {
        let layer_stack = middleware::Stack::new(self.layer, new_layer);
        KintoneClientBuilder {
            base_url: self.base_url,
            auth: self.auth,
            user_agent: self.user_agent,
            guest_space_id: self.guest_space_id,
            layer: layer_stack,
        }
    }

    pub fn guest_space_id(mut self, guest_space_id: u64) -> Self {
        self.guest_space_id = Some(guest_space_id);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}

impl<L> KintoneClientBuilder<L>
where
    L: middleware::Layer<RequestHandler>,
{
    pub fn build(self) -> KintoneClient {
        let user_agent = self.user_agent.unwrap_or_else(|| "kintone-rs".to_owned());
        let http_client: ureq::Agent = ureq::Agent::config_builder()
            .user_agent(&user_agent)
            .http_status_as_error(false)
            .build()
            .into();

        let handler = self.layer.layer(RequestHandler { http_client });

        KintoneClient {
            base_url: self.base_url,
            auth: self.auth,
            guest_space_id: self.guest_space_id,
            handler: Box::new(handler),
        }
    }
}

#[derive(Clone)]
pub enum Auth {
    Password { username: String, password: String },
    ApiToken { tokens: Vec<String> },
}

impl Auth {
    pub fn password(username: String, password: String) -> Self {
        Self::Password { username, password }
    }

    pub fn api_token(token: String) -> Self {
        Self::ApiToken {
            tokens: vec![token],
        }
    }

    pub fn api_tokens(tokens: Vec<String>) -> Self {
        Self::ApiToken { tokens }
    }
}

impl Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Auth::Password { username, .. } => f
                .debug_struct("Password")
                .field("username", username)
                .field("password", &"<hidden>")
                .finish(),
            Auth::ApiToken { .. } => {
                f.debug_struct("ApiToken").field("tokens", &"<hidden>").finish()
            }
        }
    }
}

pub(crate) struct RequestBuilder {
    method: http::Method,
    api_path: String,                 // DO NOT include "/k" prefix
    headers: HashMap<String, String>, // keys and values are NOT encoded
    query: HashMap<String, String>,   // keys and values are NOT encoded
}

impl RequestBuilder {
    pub fn new(method: http::Method, api_path: impl Into<String>) -> Self {
        Self {
            method,
            api_path: api_path.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
        }
    }

    pub fn query<V: ToString>(mut self, key: &str, value: V) -> Self {
        self.query.insert(key.to_owned(), value.to_string());
        self
    }

    pub fn query_array<V: ToString>(mut self, key: &str, values: &[V]) -> Self {
        for (i, v) in values.iter().enumerate() {
            let name = format!("{key}[{i}]");
            self.query.insert(name, v.to_string());
        }
        self
    }

    pub fn call<Resp: DeserializeOwned>(self, client: &KintoneClient) -> Result<Resp, ApiError> {
        let req = make_request(client, self.method, &self.api_path, self.headers, self.query)?;
        let resp = client.run(req)?;
        resp.into_body().read_json()
    }

    pub fn send<Body: Serialize, Resp: DeserializeOwned>(
        mut self,
        client: &KintoneClient,
        body: Body,
    ) -> Result<Resp, ApiError> {
        let body = middleware::RequestBody::from_bytes(serde_json::to_vec_pretty(&body)?);
        self.headers.insert("content-type".to_owned(), "application/json".to_owned());
        let req = make_request(client, self.method, &self.api_path, self.headers, self.query)?
            .map(|_| body);
        let resp = client.run(req)?;
        resp.into_body().read_json()
    }
}

pub(crate) struct UploadRequest {
    method: http::Method,
    api_path: String, // DO NOT include "/k" prefix
    name: String,
    filename: String,
}

impl UploadRequest {
    pub fn new(
        method: http::Method,
        api_path: impl Into<String>,
        name: String,
        filename: String,
    ) -> Self {
        Self {
            method,
            api_path: api_path.into(),
            name,
            filename,
        }
    }

    pub fn send<Resp: DeserializeOwned>(
        self,
        client: &KintoneClient,
        content: impl Read + Send + Sync + 'static,
    ) -> Result<Resp, ApiError> {
        let mut rng = rand::rng();
        let boundary = format!("{:#x}{:#x}", rng.next_u64(), rng.next_u64());

        let content_type = format!("multipart/form-data; boundary={boundary}");
        let mut headers = HashMap::with_capacity(1);
        headers.insert("content-type".to_owned(), content_type);

        let header = Cursor::new(
            format!(
                "--{boundary}\r\n\
             content-disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n\
             \r\n",
                self.name, self.filename
            )
            .into_bytes(),
        );
        let footer = Cursor::new(format!("\r\n--{boundary}--\r\n").into_bytes());
        let body = header.chain(content).chain(footer);
        let body = middleware::RequestBody::from_reader(body);

        let req = make_request(client, self.method, &self.api_path, headers, HashMap::new())?
            .map(|_| body);

        let resp = client.run(req)?;
        resp.into_body().read_json()
    }
}

pub(crate) struct DownloadRequest {
    method: http::Method,
    api_path: String,               // DO NOT include "/k" prefix
    query: HashMap<String, String>, // keys and values are NOT encoded
}

pub(crate) struct DownloadResponse {
    pub mime_type: String,
    pub content: Box<dyn Read + Send + Sync + 'static>,
}

impl DownloadRequest {
    pub fn new(method: http::Method, api_path: impl Into<String>) -> Self {
        Self {
            method,
            api_path: api_path.into(),
            query: HashMap::new(),
        }
    }

    pub fn query<V: Serialize>(mut self, key: &str, value: V) -> Self {
        let value_str = serde_json::to_string(&value).unwrap();
        self.query.insert(key.to_owned(), value_str);
        self
    }

    fn get_content_type<B>(resp: &http::Response<B>) -> Option<String> {
        let content_type = resp.headers().get(http::header::CONTENT_TYPE)?;
        let content_type = content_type.to_str().ok()?;
        Some(content_type.to_owned())
    }

    pub fn send(self, client: &KintoneClient) -> Result<DownloadResponse, ApiError> {
        let req = make_request(client, self.method, &self.api_path, HashMap::new(), self.query)?;
        let resp = client.run(req)?;
        let mime_type = Self::get_content_type(&resp).unwrap_or_default();
        let content_reader = Box::new(resp.into_body().into_reader());
        Ok(DownloadResponse {
            mime_type,
            content: content_reader,
        })
    }
}

fn make_request(
    client: &KintoneClient,
    method: http::Method,
    api_path: &str,
    mut headers: HashMap<String, String>,
    query: HashMap<String, String>,
) -> Result<http::Request<middleware::RequestBody>, http::Error> {
    // Add headers for auth
    match client.auth {
        Auth::Password {
            ref username,
            ref password,
        } => {
            let body = format!("{username}:{password}");
            let header_value = BASE64.encode(body);
            headers.insert("x-cybozu-authorization".to_owned(), header_value);
        }
        Auth::ApiToken { ref tokens } => {
            headers.insert("x-cybozu-api-token".to_owned(), tokens.join(","));
        }
    }

    // Construct URL
    let mut u = client.base_url.clone();
    let mut path = if let Some(guest_space_id) = client.guest_space_id {
        format!("/k/guest/{guest_space_id}")
    } else {
        "/k".to_owned()
    };
    path += api_path;
    u.set_path(&path);
    for (key, value) in query {
        u.query_pairs_mut().append_pair(&key, &value);
    }

    let mut req = http::Request::builder().method(method).uri(u.as_str());
    for (key, value) in headers {
        req = req.header(&key, &value);
    }
    req.body(middleware::RequestBody::void())
}
