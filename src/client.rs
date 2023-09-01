use std::fmt::Debug;

use base64::Engine;
use reqwest::{blocking::Client as ReqwestClient, Method};
use serde::de::DeserializeOwned;
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

    pub fn call<Resp: DeserializeOwned>(&self, req: Request) -> crate::Result<Resp> {
        let mut u = self.base_url.join(req.path)?;
        for (name, value) in req.form {
            u.query_pairs_mut().append_pair(name, value);
        }
        let mut req = self.http_client.request(req.method, u);
        //.header("content-type", "application/json");
        match &self.auth {
            Auth::Password { username, password } => {
                let body = format!("{username}:{password}");
                let header_value = base64::engine::general_purpose::STANDARD_NO_PAD.encode(body);
                req = req.header("x-cybozu-authorization", header_value);
            }
            Auth::ApiToken { tokens } => {
                req = req.header("x-cybozu-api-token", tokens.join(","));
            }
        }
        let resp = self.http_client.execute(req.build()?)?.error_for_status()?;
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
pub struct Request<'a> {
    method: Method,
    path: &'a str,
    form: Vec<(&'a str, &'a str)>,
}

impl Request<'_> {
    pub fn builder<'a>(method: Method, path: &'a str) -> RequestBuilder<'a> {
        RequestBuilder::new(method, path)
    }
}

#[derive(Clone)]
pub struct RequestBuilder<'a> {
    req: Request<'a>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(method: Method, path: &'a str) -> Self {
        Self {
            req: Request {
                method,
                path,
                form: Vec::new(),
            },
        }
    }

    pub fn query_param(mut self, key: &'a str, value: &'a str) -> Self {
        self.req.form.push((key, value));
        self
    }

    pub fn build(self) -> Request<'a> {
        self.req
    }
}
