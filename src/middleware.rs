//! # Middleware System for Kintone Client
//!
//! This module provides a middleware system for the Kintone client that allows
//! for intercepting and modifying requests and responses. Middleware can handle cross-cutting
//! concerns such as retries, logging, authentication, and custom request/response processing.
//!
//! ## Core Concepts
//!
//! The middleware system is built around two fundamental concepts:
//!
//! ### Handler: Request → Response Function
//! A [`Handler`] is essentially a function that transforms an HTTP request into an HTTP response.
//! This is the core abstraction that represents any piece of logic that can process requests.
//!
//! ### Layer: Handler Stacking Mechanism  
//! A [`Layer`] is a mechanism for "stacking" multiple Handlers to create a single, more powerful Handler.
//! Layers allow you to compose functionality by wrapping one Handler with another, creating
//! a chain where each layer can add behavior before and after the inner handler processes the request.
//!
//! ## How Stacking Works
//!
//! When you stack layers like this:
//! ```ignore
//! client_builder
//!     .layer(RetryLayer::new(...))     // Layer A
//!     .layer(LoggingLayer::new())      // Layer B
//!     .build()
//! ```
//!
//! You get a handler stack that looks like:
//! ```ignore
//! RetryLayer(LoggingLayer(BaseHandler))
//! ```
//!
//! Requests flow through: RetryLayer → LoggingLayer → BaseHandler  
//! Responses flow back: BaseHandler → LoggingLayer → RetryLayer
//!
//! ## Built-in Middleware
//!
//! - [`RetryLayer`] - Automatically retries failed requests with exponential backoff
//! - [`LoggingLayer`] - Logs request and response information for debugging

use std::{
    io::{Cursor, Read},
    sync::Arc,
};

use http::Request;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

/// Represents the body of an HTTP request in the middleware system.
///
/// This abstraction allows for different types of request bodies while maintaining
/// the ability to clone and reuse them for retry operations. The body can be:
/// - Empty (void)
/// - Bytes in memory (cloneable for retries)
/// - A streaming reader (non-cloneable)
///
/// # Examples
///
/// ```ignore
/// // Empty body
/// let body = RequestBody::void();
///
/// // JSON body from bytes
/// let json_bytes = serde_json::to_vec(&data)?;
/// let body = RequestBody::from_bytes(json_bytes);
///
/// // Streaming body from file
/// let file = std::fs::File::open("large_file.txt")?;
/// let body = RequestBody::from_reader(file);
/// ```
pub struct RequestBody(RequestBodyInner);

impl RequestBody {
    pub fn void() -> Self {
        RequestBody(RequestBodyInner::Void)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        RequestBody(RequestBodyInner::Bytes(Arc::from(bytes.into_boxed_slice())))
    }

    pub fn from_reader(reader: impl Read + Sync + Send + 'static) -> Self {
        RequestBody(RequestBodyInner::Reader(Box::new(reader)))
    }

    pub fn try_clone(&self) -> Option<Self> {
        match &self.0 {
            RequestBodyInner::Void => Some(RequestBody(RequestBodyInner::Void)),
            RequestBodyInner::Bytes(p) => Some(RequestBody(RequestBodyInner::Bytes(Arc::clone(p)))),
            RequestBodyInner::Reader(_) => None, // Reader cannot be cloned
        }
    }

    pub(crate) fn into_ureq_body(self) -> ureq::SendBody<'static> {
        match self.0 {
            RequestBodyInner::Void => ureq::SendBody::none(),
            RequestBodyInner::Bytes(b) => ureq::SendBody::from_owned_reader(Cursor::new(b)),
            RequestBodyInner::Reader(reader) => ureq::SendBody::from_owned_reader(reader),
        }
    }
}

enum RequestBodyInner {
    Void,
    Bytes(Arc<[u8]>),
    Reader(Box<dyn Read + Sync + Send + 'static>),
}

/// Represents the body of an HTTP response in the middleware system.
///
/// This wrapper around the raw response body provides methods for reading
/// the response content, including JSON deserialization and streaming access.
/// The body can only be consumed once due to its streaming nature.
///
/// # Examples
///
/// ```ignore
/// // Read as JSON
/// let data: MyStruct = response_body.read_json()?;
///
/// // Read as raw stream
/// let reader = response_body.into_reader();
/// std::io::copy(&mut reader, &mut output_file)?;
/// ```
pub struct ResponseBody(ureq::Body);

impl ResponseBody {
    pub(crate) fn from_ureq_body(body: ureq::Body) -> Self {
        ResponseBody(body)
    }

    pub fn into_reader(self) -> impl Read + 'static {
        self.0.into_reader()
    }

    pub fn read_json<D: DeserializeOwned>(&mut self) -> Result<D, ApiError> {
        self.0.read_json().map_err(|e| e.into())
    }
}

//-----------------------------------------------------------------------------

/// Core trait for handling HTTP requests in the middleware system.
///
/// **At its essence, a Handler is a function that transforms an HTTP request into an HTTP response.**
/// This is the fundamental building block of the middleware system. Every Handler takes a request
/// and produces either a successful response or an error.
///
/// Handlers form the foundation of the middleware system, where each middleware layer
/// wraps a handler to add additional functionality while maintaining this core contract
/// of `Request -> Response`.
///
/// # The Function-like Nature
///
/// You can think of a Handler as:
/// ```ignore
/// fn handle(request: Request) -> Result<Response, Error>
/// ```
///
/// This simple concept allows for powerful composition through the middleware system.
pub trait Handler: Send + Sync + 'static {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError>;
}

/// Trait for middleware layers that can wrap handlers to add functionality.
///
/// **A Layer is a mechanism for "stacking" multiple Handlers to create a single, more powerful Handler.**
/// Think of it as a way to compose functionality by wrapping one Handler with another.
///
/// Each Layer takes an inner Handler and produces a new Handler that adds some behavior
/// around the inner one. This creates a "Russian doll" effect where requests flow through
/// each layer in order, and responses flow back through them in reverse order.
///
/// # The Stacking Concept
///
/// When you have multiple layers, they stack like this:
/// ```ignore
/// Layer3(Layer2(Layer1(BaseHandler)))
/// ```
///
/// The request flows: Layer3 -> Layer2 -> Layer1 -> BaseHandler
/// The response flows: BaseHandler -> Layer1 -> Layer2 -> Layer3
///
/// This allows each layer to:
/// - Modify the request before passing it down
/// - Modify the response after receiving it back
/// - Add cross-cutting concerns (logging, retries, authentication, etc.)
/// - Short-circuit the chain (e.g., return cached responses)
///
/// # Type Parameters
///
/// * `Inner` - The type of handler that this layer wraps
///
/// # Associated Types
///
/// * `Outer` - The type of handler returned after wrapping the inner handler
///
/// # Examples
///
/// ```ignore
/// impl<Inner: Handler> Layer<Inner> for MyMiddleware {
///     type Outer = MyHandler<Inner>;
///
///     fn layer(self, inner: Inner) -> Self::Outer {
///         // Return a new handler that wraps the inner one
///         MyHandler { inner, config: self }
///     }
/// }
/// ```
pub trait Layer<Inner: Handler>: Send + Sync + 'static {
    type Outer: Handler;
    fn layer(self, inner: Inner) -> Self::Outer;
}

//-----------------------------------------------------------------------------

/// Type alias for a function that determines whether a request should be retried.
///
/// This function receives the original request (without body) and the response,
/// and returns `true` if the request should be retried. The request body is
/// removed to avoid lifetime issues and because retry decisions are typically
/// based on response status rather than request content.
///
/// # Examples
///
/// ```ignore
/// let should_retry: Box<ShouldRetryFn> = Box::new(|req, resp| {
///     // Retry on server errors (5xx) or specific client errors
///     resp.status().is_server_error() || resp.status() == 429
/// });
/// ```
pub type ShouldRetryFn =
    dyn Fn(&http::Request<()>, &http::Response<ResponseBody>) -> bool + Send + Sync + 'static;

/// Middleware layer that automatically retries failed requests with exponential backoff.
///
/// This layer is particularly useful for handling transient errors like database locks
/// (GAIA_DA02 in Kintone) or network timeouts. It implements exponential backoff to
/// avoid overwhelming the server with rapid retry attempts.
///
/// # Retry Logic
///
/// - Requests are retried up to `max_attempts` times
/// - Delay between retries starts at `initial_delay` and doubles after each attempt
/// - Delay is capped at `max_delay` to prevent excessively long waits
/// - Only requests with cloneable bodies can be retried (streaming requests are not retried)
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use kintone::middleware::RetryLayer;
///
/// // Retry up to 5 times with exponential backoff
/// let retry_layer = RetryLayer::new(
///     5,                              // max_attempts
///     Duration::from_millis(500),     // initial_delay
///     Duration::from_secs(30),        // max_delay
///     None                            // use default retry logic
/// );
/// ```
pub struct RetryLayer {
    max_attempts: usize,
    initial_delay: std::time::Duration,
    max_delay: std::time::Duration,
    should_retry: Box<ShouldRetryFn>,
}

impl RetryLayer {
    pub fn new(
        max_attempts: usize,
        initial_delay: std::time::Duration,
        max_delay: std::time::Duration,
        should_retry: Option<Box<ShouldRetryFn>>,
    ) -> Self {
        let should_retry: Box<ShouldRetryFn> = match should_retry {
            Some(f) => f,
            None => Box::new(|_: &http::Request<()>, resp: &http::Response<ResponseBody>| {
                !resp.status().is_success()
            }),
        };
        RetryLayer {
            max_attempts,
            initial_delay,
            max_delay,
            should_retry,
        }
    }
}

impl<Inner: Handler> Layer<Inner> for RetryLayer {
    type Outer = RetryHandler<Inner>;
    fn layer(self, inner: Inner) -> Self::Outer {
        RetryHandler { inner, layer: self }
    }
}

/// Handler implementation that wraps another handler with retry logic.
///
/// This handler implements the actual retry behavior for the [`RetryLayer`].
/// It attempts requests multiple times according to the configured retry policy,
/// with exponential backoff between attempts.
///
/// This is an internal implementation detail and should not be used directly.
pub struct RetryHandler<Inner> {
    inner: Inner,
    layer: RetryLayer,
}

impl<Inner: Handler> Handler for RetryHandler<Inner> {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError> {
        let (parts, body) = req.into_parts();

        let mut attempts = 0;
        let mut delay = self.layer.initial_delay;

        loop {
            let Some(body_cloned) = body.try_clone() else {
                // Body cannot be cloned. We cannot retry this request.
                let req = Request::from_parts(parts, body);
                return self.inner.handle(req);
            };
            let req_cloned = http::Request::from_parts(parts.clone(), body_cloned);
            let result = self.inner.handle(req_cloned);

            match result {
                Ok(resp) => {
                    if attempts >= self.layer.max_attempts {
                        return Ok(resp);
                    }
                    let req_nobody = http::Request::from_parts(parts.clone(), ());
                    let retry_ok = (self.layer.should_retry)(&req_nobody, &resp);
                    if !retry_ok {
                        return Ok(resp);
                    }
                    // do retry
                }
                Err(e) => {
                    if attempts >= self.layer.max_attempts {
                        return Err(e);
                    }
                    // do retry
                }
            }

            std::thread::sleep(delay);
            delay = std::cmp::min(delay * 2, self.layer.max_delay);
            attempts += 1;
        }
    }
}

//-----------------------------------------------------------------------------

/// Middleware layer that logs HTTP request and response information.
///
/// This layer provides debugging capabilities by logging details about each
/// HTTP request and response. It logs to stderr using `eprintln!` macros,
/// making it suitable for development and debugging scenarios.
///
/// # Logged Information
///
/// - Request: HTTP method and URL
/// - Response: HTTP status code
/// - Errors: Full error details
///
/// # Examples
///
/// ```rust
/// use kintone::middleware::LoggingLayer;
///
/// let logging_layer = LoggingLayer::new();
/// // or
/// let logging_layer = LoggingLayer::default();
/// ```
///
/// # Output Example
///
/// ```text
/// Request: method=GET, url="https://example.cybozu.com/k/v1/records.json?app=123"
/// Response: status=200
/// ```
pub struct LoggingLayer;

impl LoggingLayer {
    pub fn new() -> Self {
        LoggingLayer
    }
}

impl Default for LoggingLayer {
    fn default() -> Self {
        LoggingLayer::new()
    }
}

impl<Inner: Handler> Layer<Inner> for LoggingLayer {
    type Outer = LoggingHandler<Inner>;
    fn layer(self, inner: Inner) -> Self::Outer {
        LoggingHandler { inner }
    }
}

/// Handler implementation that wraps another handler with logging functionality.
///
/// This handler implements the actual logging behavior for the [`LoggingLayer`].
/// It logs request details before calling the inner handler and logs response
/// details after receiving the response.
///
/// This is an internal implementation detail and should not be used directly.
pub struct LoggingHandler<Inner> {
    inner: Inner,
}

impl<Inner: Handler> Handler for LoggingHandler<Inner> {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError> {
        eprintln!("Request: method={}, url={:?}", req.method(), req.uri());
        let result = self.inner.handle(req);
        match &result {
            Ok(resp) => eprintln!("Response: status={:?}", resp.status().as_u16()),
            Err(e) => eprintln!("Error: {e:?}"),
        }
        result
    }
}

//-----------------------------------------------------------------------------

/// A no-op middleware layer that provides no additional functionality.
///
/// This layer is used as the base case in the middleware stack. When applied,
/// it simply returns the inner handler unchanged. It's primarily used internally
/// by the [`KintoneClientBuilder`] as the starting point for building middleware stacks.
///
/// [`KintoneClientBuilder`]: crate::client::KintoneClientBuilder
pub struct NoLayer;

impl<Inner: Handler> Layer<Inner> for NoLayer {
    type Outer = Inner;
    fn layer(self, inner: Inner) -> Self::Outer {
        inner
    }
}

/// A stack of two middleware layers that composes them into a single layer.
///
/// This type allows for building chains of middleware by combining pairs of layers.
/// When applied, it first applies the `Tail` layer to the inner handler, then
/// applies the `Head` layer to the result.
///
/// This is an internal implementation detail used by the middleware system to
/// build complex middleware stacks from individual layers.
///
/// # Type Parameters
///
/// * `Head` - The outer layer (applied last)
/// * `Tail` - The inner layer (applied first)
///
/// # Examples
///
/// ```ignore
/// // This creates a stack: LoggingLayer -> RetryLayer -> Handler
/// let stack = Stack::new(LoggingLayer::new(), RetryLayer::new(...));
/// ```
pub struct Stack<Head, Tail>(Head, Tail);

impl<Head, Tail> Stack<Head, Tail> {
    pub fn new(head: Head, tail: Tail) -> Self {
        Stack(head, tail)
    }
}

impl<Inner, Head, Tail> Layer<Inner> for Stack<Head, Tail>
where
    Inner: Handler,
    Head: Layer<Tail::Outer>,
    Tail: Layer<Inner>,
{
    type Outer = Head::Outer;
    fn layer(self, inner: Inner) -> Self::Outer {
        self.0.layer(self.1.layer(inner))
    }
}
