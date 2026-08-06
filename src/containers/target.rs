use crate::containers::global_replaceable::{GlobalReplaceableError, Globals};
use crate::containers::{Auth, GlobalReplaceable, Method, Payload, SecretString};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TargetError {
    #[error("{0}")]
    GlobalReplaceable(#[from] GlobalReplaceableError),

    #[error("Found global key {0} but no global variables set for this key.")]
    GlobalsNotSet(String),
}

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
    pub(crate) fn into_target(self, globals: &Globals) -> Result<Target, TargetError> {
        Ok(Target {
            name: self.name,
            url: self.url,
            method: self.method,
            headers: replace_global(self.headers, globals.headers.as_ref())?,
            auth: replace_global(self.auth, globals.auth.as_ref())?,
            payload: replace_global(self.payload, globals.payload.as_ref())?,
            timeout_seconds: replace_global(
                self.timeout_seconds,
                globals.timeout_seconds.as_ref(),
            )?,
        })
    }
}

fn replace_global<T: Clone>(
    to_replace: Option<GlobalReplaceable<T>>,
    globals: Option<&HashMap<String, T>>,
) -> Result<Option<T>, TargetError> {
    to_replace
        .map(|tr| replace_global_some(tr, globals))
        .transpose()
}

fn replace_global_some<T: Clone>(
    to_replace: GlobalReplaceable<T>,
    globals: Option<&HashMap<String, T>>,
) -> Result<T, TargetError> {
    match globals {
        Some(gbls) => Ok(to_replace.into_concrete(gbls)?),
        None => match (to_replace.local, to_replace.global) {
            (Some(local), None) => Ok(local),
            (Some(local), Some(global)) => {
                debug!(
                    "Found local and global key {global} bug global variables not set for key. Using local."
                );
                Ok(local)
            }
            (None, Some(global)) => Err(TargetError::GlobalsNotSet(global)),
            (None, None) => Err(GlobalReplaceableError::Underspecified)?,
        },
    }
}
