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

pub(crate) trait GlobalReplaceableLocal {
    fn has_local(&self) -> bool;
}

// * Since I flatten local if the leftovers can't be parsed to local they just get parsed as None
// TODO - Try untagged here to try to force the serialize into T and error otherwise. Note the error message will probably be bad
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct GlobalsCfg {
    pub(crate) globals: Option<Globals>,
}
