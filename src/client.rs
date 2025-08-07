use std::collections::HashMap;
use std::fmt::Debug;

use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ApiResult;

pub struct KintoneClient {
    base_url: url::Url,
    auth: Auth,
    http_client: ureq::Agent,
}

impl KintoneClient {
    pub fn new(base_url: &str, auth: Auth) -> crate::BoxResult<Self> {
        let base_url = url::Url::parse(base_url)?;
        let user_agent = "kintone-rs/0.1.0";
        let http_client = ureq::AgentBuilder::new().user_agent(user_agent).build();
        Ok(Self {
            base_url,
            auth,
            http_client,
        })
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
    method: http::Method,
    path: String,
    headers: HashMap<String, String>, // keys and values are NOT encoded
    query: HashMap<String, String>,   // keys and values are NOT encoded
}

impl RequestBuilder {
    pub fn new(method: http::Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
        }
    }

    pub fn query<V: Serialize>(mut self, key: &str, value: V) -> Self {
        let value_str = serde_json::to_string(&value).unwrap();
        self.query.insert(key.to_string(), value_str);
        self
    }

    pub fn query_array<V: Serialize>(mut self, key: &str, values: &[V]) -> Self {
        for (i, v) in values.iter().enumerate() {
            let name = format!("{key}[{i}]");
            let value_str = serde_json::to_string(v).unwrap();
            self.query.insert(name, value_str);
        }
        self
    }

    fn make_request(mut self, client: &KintoneClient) -> ureq::Request {
        match client.auth {
            Auth::Password {
                ref username,
                ref password,
            } => {
                let body = format!("{username}:{password}");
                let header_value = BASE64.encode(body);
                self.headers
                    .insert("x-cybozu-authorization".to_string(), header_value);
            }
            Auth::ApiToken { ref tokens } => {
                self.headers
                    .insert("x-cybozu-api-token".to_string(), tokens.join(","));
            }
        }
        let u = client.base_url.join(&self.path).unwrap();
        let mut req = client.http_client.request(self.method.as_str(), u.as_str());
        for (key, value) in self.query {
            req = req.query(&key, &value);
        }
        for (key, value) in self.headers {
            req = req.set(&key, &value);
        }
        req
    }

    pub fn call<Resp: DeserializeOwned>(self, client: &KintoneClient) -> ApiResult<Resp> {
        let resp = self.make_request(client).call()?;
        Ok(resp.into_json()?)
    }

    pub fn send<Body: Serialize, Resp: DeserializeOwned>(
        self,
        client: &KintoneClient,
        body: Body,
    ) -> ApiResult<Resp> {
        let resp = self.make_request(client).send_json(body)?;
        Ok(resp.into_json()?)
    }
}
