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
//! ### Client Certificate Authentication (Mutual TLS)
//!
//! For Kintone's "Secure Access" feature (セキュアアクセス):
//!
//! ```rust
//! use kintone::client::{Auth, KintoneClientBuilder};
//!
//! let cert_pem = std::fs::read("client.crt")?;
//! let key_pem = std::fs::read("client.key")?;
//!
//! let client = KintoneClientBuilder::new(
//!     "https://your-domain.cybozu.com",
//!     Auth::api_token("your-api-token".to_owned())
//! )
//! .client_certificate_from_pem(&cert_pem, &key_pem)?
//! .build();
//! # Ok::<(), Box<dyn std::error::Error>>(())
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
use ureq::tls::{Certificate, ClientCert, PrivateKey, TlsConfig};

use crate::error::ApiError;
use crate::middleware;

/// The main HTTP client for communicating with Kintone's REST API.
///
/// This client handles authentication, request building, and response processing
/// for all API calls. It supports both API token and username/password authentication,
/// as well as guest spaces.
///
/// The client is designed to be reused across multiple API calls and is thread-safe.
/// For advanced configuration like middleware support, use [`KintoneClientBuilder`].
///
/// # Examples
///
/// ```rust
/// use kintone::client::{Auth, KintoneClient};
///
/// // Simple client creation
/// let client = KintoneClient::new(
///     "https://your-domain.cybozu.com",
///     Auth::api_token("your-api-token".to_owned())
/// );
/// ```
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

/// Internal HTTP request handler that implements the actual HTTP communication.
///
/// This is an internal implementation detail and should not be used directly.
/// Use [`KintoneClient`] or [`KintoneClientBuilder`] instead.
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

/// Builder for configuring and creating [`KintoneClient`] instances.
///
/// This builder provides a fluent API for configuring various aspects of the Kintone client,
/// including middleware, user agent, and guest space settings. The builder uses a type-safe
/// approach to ensure that middleware layers are properly configured.
///
/// # Type Parameters
///
/// * `L` - The middleware layer type. This is used to ensure type safety when building
///   the middleware stack.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use kintone::client::{Auth, KintoneClientBuilder};
/// use kintone::middleware;
///
/// let client = KintoneClientBuilder::new(
///         "https://your-domain.cybozu.com",
///         Auth::api_token("your-api-token".to_owned())
///     )
///     .user_agent("MyApp/1.0")
///     .guest_space_id(123)
///     .layer(middleware::RetryLayer::new(5, Duration::from_secs(1), Duration::from_secs(8), None))
///     .layer(middleware::LoggingLayer::new())
///     .build();
/// ```
pub struct KintoneClientBuilder<L> {
    base_url: url::Url,
    auth: Auth,
    user_agent: Option<String>,
    guest_space_id: Option<u64>,
    client_cert: Option<ClientCert>,
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
            client_cert: None,
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
            client_cert: self.client_cert,
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

    /// Sets a client certificate for mutual TLS authentication.
    ///
    /// This method configures the client to use a client certificate for authentication,
    /// which is required when using cybozu.com's "Secure Access" feature.
    /// Secure Access is an enterprise security feature that requires mutual TLS (mTLS)
    /// authentication using client certificates.
    ///
    /// The certificate and private key must be provided in PEM format.
    ///
    /// # Arguments
    ///
    /// * `cert_pem` - The client certificate in PEM format (as bytes)
    /// * `key_pem` - The private key corresponding to the certificate in PEM format (as bytes)
    ///
    /// # Converting from PFX format
    ///
    /// Secure Access certificates are typically downloaded in PFX (PKCS#12) format.
    /// You can convert them to PEM format using OpenSSL commands:
    ///
    /// ```bash
    /// openssl pkcs12 -in input.pfx -nokeys -out client-cert.pem
    /// openssl pkcs12 -in input.pfx -nocerts -out client-key.pem -nodes
    /// ```
    ///
    /// The `-nodes` flag ensures the private key is not encrypted with a passphrase.
    /// After conversion, you can load these PEM files using the example below.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use kintone::client::{Auth, KintoneClientBuilder};
    ///
    /// // Load certificate and key from PEM files
    /// let cert_pem = std::fs::read("client-cert.pem")?;
    /// let key_pem = std::fs::read("client-key.pem")?;
    ///
    /// let client = KintoneClientBuilder::new(
    ///         "https://your-domain.cybozu.com",
    ///         Auth::api_token("your-api-token".to_owned())
    ///     )
    ///     .client_certificate_from_pem(&cert_pem, &key_pem)?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn client_certificate_from_pem(
        mut self,
        cert_pem: &[u8],
        key_pem: &[u8],
    ) -> Result<Self, std::io::Error> {
        let cert = Certificate::from_pem(cert_pem).map_err(|e| e.into_io())?;
        let key = PrivateKey::from_pem(key_pem).map_err(|e| e.into_io())?;
        self.client_cert = Some(ClientCert::new_with_certs(&[cert], key));
        Ok(self)
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
            .tls_config(TlsConfig::builder().client_cert(self.client_cert).build())
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

/// Authentication configuration for Kintone API access.
///
/// Kintone supports two primary authentication methods:
/// - API Token authentication
/// - Username/Password authentication
///
/// # Examples
///
/// ```rust
/// use kintone::client::Auth;
///
/// // API token authentication
/// let auth = Auth::api_token("your-api-token".to_owned());
///
/// // Multiple API tokens (for accessing multiple apps)
/// let auth = Auth::api_tokens(vec![
///     "token1".to_owned(),
///     "token2".to_owned(),
/// ]);
///
/// // Username/password authentication
/// let auth = Auth::password("username".to_owned(), "password".to_owned());
/// ```
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

/// Internal builder for file upload requests to the Kintone API.
///
/// This builder handles the multipart/form-data encoding required for file uploads
/// to Kintone. It automatically generates proper boundaries and formats the request
/// according to the multipart specification.
///
/// This is an internal implementation detail and should not be used directly.
/// Use the [`crate::v1::file::upload`] function instead.
///
/// # Examples
///
/// This is typically used internally like:
/// ```ignore
/// let upload = UploadRequest::new(
///     http::Method::POST,
///     "/v1/file.json",
///     "file".to_owned(),
///     "document.pdf".to_owned()
/// );
/// let response = upload.send(&client, file_content)?;
/// ```
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

/// Internal builder for file download requests from the Kintone API.
///
/// This builder handles the download of files from Kintone, including proper
/// content-type detection and streaming of file content. It supports downloading
/// files by their file key.
///
/// This is an internal implementation detail and should not be used directly.
/// Use the [`crate::v1::file::download`] function instead.
pub(crate) struct DownloadRequest {
    method: http::Method,
    api_path: String,               // DO NOT include "/k" prefix
    query: HashMap<String, String>, // keys and values are NOT encoded
}

/// Response from a file download operation.
///
/// Contains the downloaded file's content as a readable stream and its MIME type.
/// The content is provided as a `Read` trait object to allow for efficient streaming
/// of large files without loading them entirely into memory.
///
/// # Examples
///
/// ```ignore
/// let response = download_request.send(&client)?;
/// println!("Downloaded file type: {}", response.mime_type);
///
/// // Stream the content to a file
/// let mut file = std::fs::File::create("downloaded_file")?;
/// std::io::copy(&mut response.content, &mut file)?;
/// ```
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
