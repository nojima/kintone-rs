use std::{
    io::{Cursor, Read},
    sync::Arc,
};

use http::Request;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

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

pub trait Handler {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError>;
}

impl<F> Handler for F
where
    F: Fn(http::Request<RequestBody>) -> Result<http::Response<ResponseBody>, ApiError>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError> {
        self(req)
    }
}

pub trait Layer<Inner: Handler + Send + Sync + 'static> {
    type Outer: Handler + Send + Sync + 'static;
    fn layer(self, inner: Inner) -> Self::Outer;
}

//-----------------------------------------------------------------------------

pub struct RetryLayer {
    max_attempts: usize,
    initial_delay: std::time::Duration,
    max_delay: std::time::Duration,
}

impl RetryLayer {
    pub fn new(
        max_attempts: usize,
        initial_delay: std::time::Duration,
        max_delay: std::time::Duration,
    ) -> Self {
        RetryLayer {
            max_attempts,
            initial_delay,
            max_delay,
        }
    }
}

impl<Inner: Handler + Send + Sync + 'static> Layer<Inner> for RetryLayer {
    type Outer = RetryHandler<Inner>;
    fn layer(self, inner: Inner) -> Self::Outer {
        RetryHandler { inner, layer: self }
    }
}

pub struct RetryHandler<Inner> {
    inner: Inner,
    layer: RetryLayer,
}

impl<Inner: Handler + Send + Sync + 'static> Handler for RetryHandler<Inner> {
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
                    if resp.status().is_success() || attempts >= self.layer.max_attempts {
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

impl<Inner: Handler + Send + Sync + 'static> Layer<Inner> for LoggingLayer {
    type Outer = LoggingHandler<Inner>;
    fn layer(self, inner: Inner) -> Self::Outer {
        LoggingHandler { inner }
    }
}

pub struct LoggingHandler<Inner> {
    inner: Inner,
}

impl<Inner: Handler + Send + Sync + 'static> Handler for LoggingHandler<Inner> {
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

pub struct NoLayer;

impl<Inner> Layer<Inner> for NoLayer
where
    Inner: Handler + Send + Sync + 'static,
{
    type Outer = Inner;
    fn layer(self, inner: Inner) -> Self::Outer {
        inner
    }
}

pub struct Stack<Head, Tail>(Head, Tail);

impl<Head, Tail> Stack<Head, Tail> {
    pub fn new(head: Head, tail: Tail) -> Self {
        Stack(head, tail)
    }
}

impl<Inner, Head, Tail> Layer<Inner> for Stack<Head, Tail>
where
    Inner: Handler + Send + Sync + 'static,
    Head: Layer<Tail::Outer> + Send + Sync + 'static,
    Tail: Layer<Inner> + Send + Sync + 'static,
{
    type Outer = Head::Outer;
    fn layer(self, inner: Inner) -> Self::Outer {
        self.0.layer(self.1.layer(inner))
    }
}
