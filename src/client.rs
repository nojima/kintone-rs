use std::fmt::Debug;

use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct KintoneClient {
    base_url: url::Url,
    auth: Auth,
    http_client: ureq::Agent,
}

impl KintoneClient {
    pub fn new(base_url: &str, auth: Auth) -> crate::Result<Self> {
        let base_url = url::Url::parse(base_url)?;
        let user_agent = "kintone-rs/0.1.0";
        let http_client = ureq::AgentBuilder::new().user_agent(user_agent).build();
        Ok(Self {
            base_url,
            auth,
            http_client,
        })
    }

    pub fn request(&self, method: http::Method, path: &str) -> RequestBuilder {
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
    builder: ureq::Request,
}

impl RequestBuilder {
    pub(crate) fn new(
        client: &ureq::Agent,
        base_url: &url::Url,
        auth: &Auth,
        method: http::Method,
        path: &str,
    ) -> Self {
        let u = base_url.join(path).unwrap();
        let mut builder = client.request_url(method.as_str(), &u);
        match auth {
            Auth::Password { username, password } => {
                let body = format!("{username}:{password}");
                let header_value = BASE64.encode(body);
                builder = builder.set("x-cybozu-authorization", &header_value);
            }
            Auth::ApiToken { tokens } => {
                builder = builder.set("x-cybozu-api-token", &tokens.join(","));
            }
        }
        Self { builder }
    }

    pub fn query<V: Serialize>(mut self, key: &str, value: V) -> Self {
        let value_str = serde_json::to_string(&value).unwrap();
        self.builder = self.builder.query(key, &value_str);
        self
    }

    pub fn query_array<V: Serialize>(mut self, key: &str, values: &[V]) -> Self {
        for (i, v) in values.iter().enumerate() {
            let name = format!("{}[{}]", key, i);
            let value_str = serde_json::to_string(v).unwrap();
            self.builder = self.builder.query(&name, &value_str);
        }
        self
    }

    pub fn call<Resp: DeserializeOwned>(self) -> Result<Resp, crate::ApiError> {
        let resp = self.builder.call()?;
        Ok(resp.into_json()?)
    }
    pub fn send<Body: Serialize, Resp: DeserializeOwned>(
        self,
        body: Body,
    ) -> Result<Resp, crate::ApiError> {
        let resp = self.builder.send_json(body)?;
        Ok(resp.into_json()?)
    }
}
