use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::containers::{Auth, Headers, Payload};

#[derive(Clone, Debug, Error)]
pub(crate) enum GlobalReplaceableError {
    #[error("Could not find requested global variable {0}.")]
    MissingGlobal(String),

    #[error("Must only specify a local or global for all replaceable variables.")]
    Overspecified,

    #[error("Must specify a local or global for all global replaceable variables.")]
    Underspecified,
}

pub(super) trait GlobalReplaceableLocal {
    fn has_local(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub(super) struct GlobalReplaceableCfg<T> {
    pub(super) global: Option<String>,

    #[serde(flatten)]
    pub(super) local: Option<T>,
}

pub(super) enum GlobalReplaceable<T> {
    Global(String),
    Local(T),
}

impl<T: GlobalReplaceableLocal> TryFrom<GlobalReplaceableCfg<T>> for GlobalReplaceable<T> {
    type Error = GlobalReplaceableError;
    fn try_from(value: GlobalReplaceableCfg<T>) -> Result<Self, Self::Error> {
        match (value.local, value.global) {
            (Some(local), None) => match local.has_local() {
                true => Ok(Self::Local(local)),
                false => Err(GlobalReplaceableError::Underspecified),
            },
            (None, Some(global)) => Ok(Self::Global(global)),
            (Some(local), Some(global)) => match local.has_local() {
                true => Err(GlobalReplaceableError::Overspecified),
                false => Ok(Self::Global(global)),
            },
            (None, None) => Err(GlobalReplaceableError::Underspecified),
        }
    }
}

impl<T: Clone> GlobalReplaceable<T> {
    pub(super) fn into_concrete(
        self,
        globals: &HashMap<String, T>,
    ) -> Result<T, GlobalReplaceableError> {
        match self {
            Self::Local(local) => Ok(local),
            Self::Global(global) => Ok(globals
                .get(&global)
                .ok_or(GlobalReplaceableError::MissingGlobal(global))?
                .clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Globals {
    pub(super) auth: Option<HashMap<String, Auth>>,
    pub(super) headers: Option<HashMap<String, Headers>>,
    pub(super) payload: Option<HashMap<String, Payload>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct GlobalsCfg {
    pub(super) globals: Option<Globals>,
}
