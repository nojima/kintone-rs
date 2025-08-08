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
//!     Auth::api_token("your-api-token".to_string())
//! );
//! ```
//!
//! ### Username/Password Authentication
//! ```rust
//! use kintone::client::{Auth, KintoneClient};
//!
//! let client = KintoneClient::new(
//!     "https://your-domain.cybozu.com",
//!     Auth::password("username".to_string(), "password".to_string())
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
//!         Auth::api_token("your-api-token".to_string())
//!     )
//!     .user_agent("MyApp/1.0")
//!     .build();
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
//!         Auth::api_token("your-api-token".to_string())
//!     )
//!     .guest_space_id(123)
//!     .build();
//!
//! // If you need to access a different guest space (ID 456), create another client
//! let another_guest_client = KintoneClientBuilder::new(
//!         "https://your-domain.cybozu.com",
//!         Auth::api_token("your-api-token".to_string())
//!     )
//!     .guest_space_id(456)
//!     .build();
//! ```

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;

use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use rand::RngCore as _;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

pub struct KintoneClient {
    http_client: ureq::Agent,
    base_url: url::Url,
    auth: Auth,
    guest_space_id: Option<u64>,
}

impl KintoneClient {
    pub fn new(base_url: &str, auth: Auth) -> Self {
        KintoneClientBuilder::new(base_url, auth).build()
    }
}

pub struct KintoneClientBuilder {
    base_url: url::Url,
    auth: Auth,
    user_agent: Option<String>,
    guest_space_id: Option<u64>,
}

impl KintoneClientBuilder {
    pub fn new(base_url: &str, auth: Auth) -> Self {
        let base_url = url::Url::parse(base_url).unwrap();
        Self {
            base_url,
            auth,
            user_agent: None,
            guest_space_id: None,
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

    pub fn build(self) -> KintoneClient {
        let user_agent = self.user_agent.unwrap_or_else(|| "kintone-rs".to_string());
        let http_client = ureq::AgentBuilder::new()
            .user_agent(&user_agent)
            .try_proxy_from_env(true)
            .build();

        KintoneClient {
            http_client,
            base_url: self.base_url,
            auth: self.auth,
            guest_space_id: self.guest_space_id,
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
            Auth::ApiToken { .. } => f
                .debug_struct("ApiToken")
                .field("tokens", &"<hidden>")
                .finish(),
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
        self.query.insert(key.to_string(), value.to_string());
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
        let resp = make_request(
            client,
            self.method,
            &self.api_path,
            self.headers,
            self.query,
        )
        .call()?;
        Ok(resp.into_json()?)
    }

    pub fn send<Body: Serialize, Resp: DeserializeOwned>(
        self,
        client: &KintoneClient,
        body: Body,
    ) -> Result<Resp, ApiError> {
        let resp = make_request(
            client,
            self.method,
            &self.api_path,
            self.headers,
            self.query,
        )
        .send_json(body)?;
        Ok(resp.into_json()?)
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
        content: impl Read,
    ) -> Result<Resp, ApiError> {
        let mut rng = rand::rng();
        let boundary = format!("{:#x}{:#x}", rng.next_u64(), rng.next_u64());

        let content_type = format!("multipart/form-data; boundary={boundary}");
        let mut headers = HashMap::with_capacity(1);
        headers.insert("content-type".to_string(), content_type);

        let req = make_request(client, self.method, &self.api_path, headers, HashMap::new());

        let header = format!(
            "--{boundary}\r\n\
             content-disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n\
             \r\n",
            self.name, self.filename
        )
        .into_bytes();
        let footer = format!("\r\n--{boundary}--\r\n").into_bytes();
        let body = header.chain(content).chain(&*footer);

        let resp = req.send(body)?;
        Ok(resp.into_json()?)
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
        self.query.insert(key.to_string(), value_str);
        self
    }

    pub fn send(self, client: &KintoneClient) -> Result<DownloadResponse, ApiError> {
        let req = make_request(
            client,
            self.method,
            &self.api_path,
            HashMap::new(),
            self.query,
        );
        let resp = req.call()?;
        let mime_type = resp.header("content-type").unwrap_or_default().to_owned();
        Ok(DownloadResponse {
            mime_type,
            content: resp.into_reader(),
        })
    }
}

fn make_request(
    client: &KintoneClient,
    method: http::Method,
    api_path: &str,
    mut headers: HashMap<String, String>,
    query: HashMap<String, String>,
) -> ureq::Request {
    match client.auth {
        Auth::Password {
            ref username,
            ref password,
        } => {
            let body = format!("{username}:{password}");
            let header_value = BASE64.encode(body);
            headers.insert("x-cybozu-authorization".to_string(), header_value);
        }
        Auth::ApiToken { ref tokens } => {
            headers.insert("x-cybozu-api-token".to_string(), tokens.join(","));
        }
    }

    let mut path = if let Some(guest_space_id) = client.guest_space_id {
        format!("/k/guest/{guest_space_id}")
    } else {
        "/k".to_string()
    };
    path += api_path;
    let mut u = client.base_url.clone();
    u.set_path(&path);

    let mut req = client.http_client.request(method.as_str(), u.as_str());
    for (key, value) in query {
        req = req.query(&key, &value);
    }
    for (key, value) in headers {
        req = req.set(&key, &value);
    }
    req
}
