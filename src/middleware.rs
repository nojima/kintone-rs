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

pub trait Middleware {
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError>;
}

impl<
    F: Fn(http::Request<RequestBody>) -> Result<http::Response<ResponseBody>, ApiError>
        + Send
        + Sync
        + 'static,
> Middleware for F
{
    fn handle(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<ResponseBody>, ApiError> {
        self(req)
    }
}

pub trait Layer<Inner: Middleware + Send + Sync + 'static> {
    fn layer(self, inner: Inner) -> impl Middleware;
}

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

impl<Inner: Middleware + Send + Sync + 'static> Layer<Inner> for RetryLayer {
    fn layer(self, inner: Inner) -> impl Middleware {
        move |req: http::Request<RequestBody>| {
            let (parts, body) = req.into_parts();

            let mut attempts = 0;
            let mut delay = self.initial_delay;

            loop {
                let Some(body_cloned) = body.try_clone() else {
                    // Body cannot be cloned. We cannot retry this request.
                    let req = Request::from_parts(parts, body);
                    return inner.handle(req);
                };
                let req_cloned = http::Request::from_parts(parts.clone(), body_cloned);
                let result = inner.handle(req_cloned);

                match result {
                    Ok(resp) => {
                        if resp.status().is_success() || attempts >= self.max_attempts {
                            return Ok(resp);
                        }
                        // do retry
                    }
                    Err(e) => {
                        if attempts >= self.max_attempts {
                            return Err(e);
                        }
                        // do retry
                    }
                }

                std::thread::sleep(delay);
                delay = std::cmp::min(delay * 2, self.max_delay);
                attempts += 1;
            }
        }
    }
}
