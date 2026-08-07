use crate::containers::global_replaceable::GlobalReplaceableLocal;
use crate::containers::secrets::SecretTomlValue;
use crate::errors::TargetsError;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawPayload {
    file: Option<PathBuf>,

    #[serde(flatten)]
    params: Option<HashMap<String, SecretTomlValue>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "RawPayload")]
pub enum Payload {
    File(PathBuf),
    Params(HashMap<String, SecretTomlValue>),
}

// TODO - test this
impl GlobalReplaceableLocal for Payload {
    fn has_local(&self) -> bool {
        true
    }
}

impl TryFrom<RawPayload> for Payload {
    type Error = TargetsError;
    fn try_from(value: RawPayload) -> Result<Self, Self::Error> {
        debug!("Attempting to parse RawPayload into Payload. Parsing raw payload:\n{value:#?}");
        match (value.file, value.params) {
            (None, None) => Err(TargetsError::MissingPayloadFields),
            (Some(file), Some(params)) => match params.is_empty() {
                true => Ok(Self::File(file)),
                false => Err(TargetsError::OverspecifiedPayload),
            },
            (Some(file), None) => Ok(Self::File(file)),
            (None, Some(params)) => match params.is_empty() {
                true => Err(TargetsError::MissingPayloadFields),
                false => Ok(Self::Params(params)),
            },
        }
    }
}
