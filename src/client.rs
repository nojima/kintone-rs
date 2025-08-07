use std::collections::HashMap;
use std::fmt::Debug;

use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ApiResult;

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
        let http_client = ureq::AgentBuilder::new().user_agent(&user_agent).build();

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

        let mut u = client.base_url.clone();
        if let Some(guest_space_id) = client.guest_space_id {
            u = u.join(&format!("/k/guest/{guest_space_id}")).unwrap();
        } else {
            u = u.join("/k").unwrap();
        }
        u = u.join(&self.api_path).unwrap();

        let mut req = client.http_client.request(self.method.as_str(), u.as_str());
        for (key, value) in self.query {
            req = req.query(&key, &value);
        }
        for (key, value) in self.headers {
            req = req.set(&key, &value);
        }
        req
    }
}
