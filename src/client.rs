use std::fmt::Debug;

use base64::Engine;
use reqwest::{
    blocking::{Client as ReqwestClient, RequestBuilder as ReqwestRequestBuilder},
    Method,
};
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

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        RequestBuilder::new(&self.http_client, &self.base_url, &self.auth, method, path)
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

pub struct RequestBuilder {
    builder: ReqwestRequestBuilder,
}

impl RequestBuilder {
    pub(crate) fn new(
        client: &ReqwestClient,
        base_url: &Url,
        auth: &Auth,
        method: Method,
        path: &str,
    ) -> Self {
        let u = base_url.join(path).unwrap();
        let mut builder = client.request(method, u);
        match auth {
            Auth::Password { username, password } => {
                let body = format!("{username}:{password}");
                let header_value = base64::engine::general_purpose::STANDARD_NO_PAD.encode(body);
                builder = builder.header("x-cybozu-authorization", header_value);
            }
            Auth::ApiToken { tokens } => {
                builder = builder.header("x-cybozu-api-token", tokens.join(","));
            }
        }
        Self { builder }
    }

    pub fn query<V: Serialize>(mut self, key: &str, value: V) -> Self {
        self.builder = self.builder.query(&[(key, value)]);
        self
    }

    pub fn query_array<V: Serialize>(mut self, key: &str, values: &[V]) -> Self {
        for (i, v) in values.iter().enumerate() {
            let name = format!("{}[{}]", key, i);
            self.builder = self.builder.query(&[(name, v)]);
        }
        self
    }

    pub fn body<T: Serialize>(mut self, body: &T) -> Self {
        self.builder = self.builder.header("content-type", "application/json");
        self.builder = self.builder.json(body);
        self
    }

    pub fn send<Resp: DeserializeOwned>(self) -> crate::Result<Resp> {
        let resp = self.builder.send()?.error_for_status()?;
        Ok(resp.json()?)
    }
}
