use crate::containers::globals::GlobalReplaceableLocal;
use crate::containers::secrets::SecretTomlValue;
use crate::errors::TargetsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

const FILE_KEY: &str = "file";

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

impl GlobalReplaceableLocal for Payload {
    fn has_local(&self) -> bool {
        match self {
            Self::Params(p) => !p.is_empty(),
            Self::File(f) => f != Path::new(""),
        }
    }
}

impl TryFrom<RawPayload> for Payload {
    type Error = TargetsError;
    fn try_from(value: RawPayload) -> Result<Self, Self::Error> {
        match (value.file, value.params) {
            (None, None) => Err(TargetsError::MissingPayloadFields),
            (Some(file), Some(params)) => match params.is_empty() {
                true => Ok(Self::File(file)),
                false => Err(TargetsError::OverspecifiedPayload),
            },
            (Some(file), None) => Ok(Self::File(file)),
            (None, Some(params)) => match params.is_empty() {
                true => Err(TargetsError::MissingPayloadFields),
                false => Ok(Payload::from(params)),
            },
        }
    }
}

// This function exists because the global parse on payload will cause the .file field to be parsed into the .params field with the key "File"
// Good to have as a fallback
impl From<HashMap<String, SecretTomlValue>> for Payload {
    fn from(value: HashMap<String, SecretTomlValue>) -> Self {
        // Must check for single value here
        // Risk a panic below if don't
        if value.len() != 1 {
            return Payload::Params(value);
        }

        // .expect is safe here since we checked there is one value above
        // Must check for single value above or risk panic here
        let (k, v) = value
            .iter()
            .next()
            .expect("Invalid state. Expected payload to only have one field.");

        if k.to_lowercase() == FILE_KEY
            && let TomlValue::String(str_path) = v.as_toml_val()
        {
            Payload::File(PathBuf::from(str_path))
        } else {
            Payload::Params(value)
        }
    }
}
