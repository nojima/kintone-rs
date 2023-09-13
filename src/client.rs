use std::fmt::Debug;

use base64::Engine;
use reqwest::{blocking::Client as ReqwestClient, Method};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub struct KintoneClient {
    base_url: Url,
    auth: Auth,
    http_client: ReqwestClient,
}

impl KintoneClient {
    pub fn new(base_url: &str, auth: Auth) -> crate::Result<Self> {
        let base_url = Url::parse(base_url)?;
        let user_agent = "kintone-rs/0.1.0";
        let http_client = ReqwestClient::builder().user_agent(user_agent).build()?;
        Ok(Self {
            base_url,
            auth,
            http_client,
        })
    }

    pub fn call<Resp, Body>(&self, req: Request<Body>) -> crate::Result<Resp>
    where
        Resp: DeserializeOwned,
        Body: Serialize,
    {
        let mut u = self.base_url.join(req.path)?;
        for (name, value) in req.query_params {
            u.query_pairs_mut().append_pair(name, value);
        }
        let mut builder = self.http_client.request(req.method, u);
        if let Some(body) = req.body {
            builder = builder.header("content-type", "application/json");
            builder = builder.json(&body);
        }
        match &self.auth {
            Auth::Password { username, password } => {
                let body = format!("{username}:{password}");
                let header_value = base64::engine::general_purpose::STANDARD_NO_PAD.encode(body);
                builder = builder.header("x-cybozu-authorization", header_value);
            }
            Auth::ApiToken { tokens } => {
                builder = builder.header("x-cybozu-api-token", tokens.join(","));
            }
        }
        let resp = self
            .http_client
            .execute(builder.build()?)?
            .error_for_status()?;
        Ok(resp.json()?)
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
            Auth::Password { username, .. } => {
                write!(
                    f,
                    "Password {{ username: {username:?}, password: <hidden> }}"
                )
            }
            Auth::ApiToken { .. } => write!(f, "ApiToken {{ tokens: <hidden> }}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request<'req, Body> {
    method: Method,
    path: &'req str,
    query_params: Vec<(&'req str, &'req str)>,
    body: Option<Body>,
}

impl<Body> Request<'_, Body> {
    pub fn builder<'req>(method: Method, path: &'req str) -> RequestBuilder<'req, Body> {
        RequestBuilder::new(method, path)
    }
}

#[derive(Clone)]
pub struct RequestBuilder<'req, Body> {
    req: Request<'req, Body>,
}

impl<'req, Body> RequestBuilder<'req, Body> {
    pub fn new(method: Method, path: &'req str) -> Self {
        Self {
            req: Request {
                method,
                path,
                query_params: Vec::new(),
                body: None,
            },
        }
    }

    pub fn query_param(mut self, key: &'req str, value: &'req str) -> Self {
        self.req.query_params.push((key, value));
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.req.body = Some(body);
        self
    }

    pub fn build(self) -> Request<'req, Body> {
        self.req
    }
}
