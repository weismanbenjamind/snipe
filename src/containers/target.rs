use crate::containers::{Auth, Method, Payload, SecretString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Target {
    name: Option<String>,
    url: String,
    method: Method,
    timeout_seconds: Option<u64>,
    headers: Option<HashMap<String, SecretString>>,
    auth: Option<Auth>,
    payload: Option<Payload>,
}

impl Target {
    #[allow(dead_code)]
    pub fn new(
        name: Option<String>,
        url: &str,
        method: Method,
        timeout_seconds: Option<u64>,
        headers: Option<HashMap<String, SecretString>>,
        auth: Option<Auth>,
        payload: Option<Payload>,
    ) -> Self {
        Self {
            name,
            url: url.to_string(),
            method,
            timeout_seconds,
            headers,
            auth,
            payload,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn timeout_seconds(&self) -> Option<u64> {
        self.timeout_seconds
    }

    pub fn headers(&self) -> &Option<HashMap<String, SecretString>> {
        &self.headers
    }

    pub fn auth(&self) -> &Option<Auth> {
        &self.auth
    }

    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }
}
