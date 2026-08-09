use crate::containers::globals::{
    GlobalReplaceable, GlobalReplaceableCfg, GlobalReplaceableError, GlobalReplaceableLocal,
    Globals,
};
use crate::containers::{Auth, Headers, Method, Payload};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub(crate) enum TargetError {
    #[error("{0}")]
    GlobalReplaceable(#[from] GlobalReplaceableError),

    #[error("Found global key {0} but no global variables set for this key.")]
    GlobalsNotSet(String),
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct Target {
    pub(crate) name: Option<String>,
    pub(crate) url: String,
    pub(crate) method: Method,
    pub(crate) timeout_seconds: Option<u64>,
    pub(crate) headers: Option<Headers>,
    pub(crate) auth: Option<Auth>,
    pub(crate) payload: Option<Payload>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(super) struct GlobalReplaceableTarget {
    name: Option<String>,
    url: String,
    method: Method,
    timeout_seconds: Option<u64>,
    auth: Option<GlobalReplaceableCfg<Auth>>,
    headers: Option<GlobalReplaceableCfg<Headers>>,
    payload: Option<GlobalReplaceableCfg<Payload>>,
}

impl GlobalReplaceableTarget {
    pub(crate) fn into_target(self, globals: Option<&Globals>) -> Result<Target, TargetError> {
        let globals = OptionalGlobals { globals };

        Ok(Target {
            name: self.name,
            url: self.url,
            method: self.method,
            timeout_seconds: self.timeout_seconds,
            headers: replace_global(self.headers, globals.headers())?,
            auth: replace_global(self.auth, globals.auth())?,
            payload: replace_global(self.payload, globals.payload())?,
        })
    }
}

struct OptionalGlobals<'a> {
    globals: Option<&'a Globals>,
}

impl<'a> OptionalGlobals<'a> {
    fn headers(&self) -> Option<&HashMap<String, Headers>> {
        self.globals.and_then(|g| g.headers.as_ref())
    }

    fn auth(&self) -> Option<&HashMap<String, Auth>> {
        self.globals.and_then(|g| g.auth.as_ref())
    }

    fn payload(&self) -> Option<&HashMap<String, Payload>> {
        self.globals.and_then(|g| g.payload.as_ref())
    }
}

fn replace_global<T: Clone + GlobalReplaceableLocal>(
    to_replace: Option<GlobalReplaceableCfg<T>>,
    globals: Option<&HashMap<String, T>>,
) -> Result<Option<T>, TargetError> {
    to_replace
        .map(|tr| replace_global_some(tr, globals))
        .transpose()
}

fn replace_global_some<T: Clone + GlobalReplaceableLocal>(
    to_replace: GlobalReplaceableCfg<T>,
    globals: Option<&HashMap<String, T>>,
) -> Result<T, TargetError> {
    let global_replaceable: GlobalReplaceable<T> = to_replace.try_into()?;
    match globals {
        Some(gbls) => Ok(global_replaceable.into_concrete(gbls)?),
        None => match global_replaceable {
            GlobalReplaceable::Local(local) => Ok(local),
            GlobalReplaceable::Global(global) => Err(TargetError::GlobalsNotSet(global)),
        },
    }
}
