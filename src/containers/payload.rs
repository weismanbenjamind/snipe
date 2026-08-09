use crate::containers::globals::GlobalReplaceableLocal;
use crate::containers::secrets::SecretTomlValue;
use crate::errors::TargetsError;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;
use toml::map::Map as TomlMap;

const FILE_KEY: &str = "file";
const PARAMS_KEY: &str = "params";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RawPayload {
    file: Option<PathBuf>,

    #[serde(flatten)]
    params: Option<HashMap<String, SecretTomlValue>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "RawPayload")]
pub(crate) enum Payload {
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
// Additionall the global parse on payload will cause the .params field to always be parsed as {"params": {"actual": params}}
// Need special logic to unwrap this nested mapping. Could cause a failure if a payload has a single top level "params" key
impl From<HashMap<String, SecretTomlValue>> for Payload {
    fn from(value: HashMap<String, SecretTomlValue>) -> Self {
        // Must check for single value and short-circuit here
        // Risk a panic below if don't
        if value.len() != 1 {
            return Payload::Params(value);
        }

        // .expect is safe here since we checked there is one value above
        // Must check for single value above or risk panic here
        let (k, v) = value
            .into_iter()
            .next()
            .expect("Invalid state. Expected payload to only have one field.");

        let key_lowered = k.to_lowercase();
        let raw_toml: TomlValue = v.into();

        // If the lowered key is 'file' and we have a String value -> parse as Payload::File
        if key_lowered == FILE_KEY
            && let TomlValue::String(str_path) = raw_toml
        {
            debug!(
                "Found single length parms map with lowercased key 'file' key a string value. Parsing payload as file variant."
            );
            Payload::File(PathBuf::from(str_path))
        }
        // If the lowred key is 'params' and we have a Table toml value -> remove the 'params' key and return the inner HashMap
        else if key_lowered == PARAMS_KEY
            && let TomlValue::Table(t) = raw_toml
        {
            debug!(
                "Found single length parms map with lowercased key 'params' key a table value. Parsing as flattened map. \
                POTENTIAL ERROR CASE if payload has a single top level key when lowercased resolves to 'params'."
            );
            Payload::Params(TomlMapWrapper(t).into())
        }
        // All other case fall back to params
        else {
            Payload::Params(rebuild_map(k, raw_toml))
        }
    }
}

struct TomlMapWrapper(TomlMap<String, TomlValue>);

impl From<TomlMapWrapper> for HashMap<String, SecretTomlValue> {
    fn from(value: TomlMapWrapper) -> HashMap<String, SecretTomlValue> {
        let map = value.0;
        let mut converted = HashMap::<String, SecretTomlValue>::with_capacity(map.len());

        map.into_iter().for_each(|(k, v)| {
            let _ = converted.insert(k, SecretTomlValue::from(v));
        });

        converted
    }
}

fn rebuild_map(k: String, raw_toml: TomlValue) -> HashMap<String, SecretTomlValue> {
    let mut map = HashMap::<String, SecretTomlValue>::with_capacity(1);
    map.insert(k, raw_toml.into());
    map
}
