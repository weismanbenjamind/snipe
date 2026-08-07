use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::containers::{Auth, Payload, SecretString};

#[derive(Clone, Debug, Error)]
pub enum GlobalReplaceableError {
    #[error("Could not find requested global variable {0}")]
    MissingGlobal(String),

    #[error("Must only specify a local or global for all replaceable variables")]
    Overspecified,

    #[error("Must specify a local or global for all global replaceable variables")]
    Underspecified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub(crate) struct GlobalReplaceableCfg<T> {
    pub(crate) global: Option<String>,

    #[serde(flatten)]
    pub(crate) local: Option<T>,
}

pub(crate) enum GlobalReplaceable<T> {
    Global(String),
    Local(T),
}

impl<T> TryFrom<GlobalReplaceableCfg<T>> for GlobalReplaceable<T> {
    type Error = GlobalReplaceableError;
    fn try_from(value: GlobalReplaceableCfg<T>) -> Result<Self, Self::Error> {
        match (value.local, value.global) {
            (Some(local), None) => Ok(Self::Local(local)),
            (None, Some(global)) => Ok(Self::Global(global)),
            (Some(_), Some(_)) => Err(GlobalReplaceableError::Overspecified),
            (None, None) => Err(GlobalReplaceableError::Underspecified),
        }
    }
}

impl<T: Clone> GlobalReplaceable<T> {
    pub(crate) fn into_concrete(
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

// TODO - Add timeout
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Globals {
    pub(crate) auth: Option<HashMap<String, Auth>>,
    pub(crate) headers: Option<HashMap<String, HashMap<String, SecretString>>>,
    pub(crate) payload: Option<HashMap<String, Payload>>,
}
