use crate::inputs::RawFormat;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub(crate) enum OutputError {
    #[error("Invalid grab option {0}).")]
    InvalidGrab(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct OutputCfg {
    pub(crate) grab: Option<Vec<GrabCfg>>,
    pub(crate) format: Option<RawFormat>,
    pub(crate) pretty: Option<bool>,
    pub(crate) output_file: Option<PathBuf>,
    pub(crate) dry_run: Option<bool>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", rename_all = "snake_case")]
pub(crate) enum GrabCfg {
    StatusCode,
    Headers,
    Body,
    Full,
    IntStatusCode,
}

impl TryFrom<String> for GrabCfg {
    type Error = OutputError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "status_code" => Self::StatusCode,
            "headers" => Self::Headers,
            "body" => Self::Body,
            "full" => Self::Full,
            "int_status_code" => Self::IntStatusCode,
            _ => return Err(OutputError::InvalidGrab(value)),
        })
    }
}
