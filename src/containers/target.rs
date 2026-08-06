use crate::containers::{
    Auth, GlobalReplaceable, Method, Payload, SecretString,
    global_replaceable::{GlobalReplaceableError, Globals},
};
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct GlobalReplaceableTarget {
    pub(crate) name: Option<String>,
    pub(crate) url: String,
    pub(crate) method: Method,
    pub(crate) auth: Option<GlobalReplaceable<Auth>>,
    pub(crate) headers: Option<GlobalReplaceable<HashMap<String, SecretString>>>,
    pub(crate) payload: Option<GlobalReplaceable<Payload>>,
    pub(crate) timeout_seconds: Option<GlobalReplaceable<u64>>,
}

impl GlobalReplaceableTarget {
    // TODO - remove unwraps
    fn replace_globals(self, globals: &Globals) -> Target {
        let auth = replace_global(self.auth, globals.auth.as_ref()).unwrap();
        let headers = replace_global(self.headers, globals.headers.as_ref()).unwrap();
        let payload = replace_global(self.payload, globals.payload.as_ref()).unwrap();
        let timeout_seconds =
            replace_global(self.timeout_seconds, globals.timeout_seconds.as_ref()).unwrap();

        Target {
            name: self.name,
            url: self.url,
            method: self.method,
            timeout_seconds,
            headers,
            auth,
            payload,
        }
    }
}

fn replace_global<T: Clone>(
    to_replace: Option<GlobalReplaceable<T>>,
    globals: Option<&HashMap<String, T>>,
) -> Result<Option<T>, GlobalReplaceableError> {
    let result = match to_replace {
        // If have a variable to replace - replace it if globals exist
        // If globals do no exist - return the local value
        Some(to_replace) => match globals {
            Some(globals) => Some(to_replace.into_concrete(globals)?),
            None => {
                // TODO - Handle global and local case - maybe fallback to locals with a log
                if let Some(key) = to_replace.global {
                    panic!("Found global key {key} but no global variables set for this key.")
                } else {
                    to_replace.local
                }
            }
        },
        // If variable to replace is not present - simply return None
        None => None,
    };

    Ok(result)
}
