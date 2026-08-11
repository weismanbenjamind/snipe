use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
enum FormatError {
    #[error("Invalid grab option {0})")]
    InvalidGrab(String),

    #[error("Invalid format {0}")]
    InvalidFormat(String),

    #[error("Cannot pass other grab options when grabbing {0}")]
    InvalidGrabCombo(String),

    #[error("Cannot specify pretty formatted output when using binary format")]
    PrettyWithBinary,

    #[error("Must specify format if passing pretty")]
    PrettyWithoutFormat,

    #[error("Must specify an output file if using binary format")]
    BinaryWithNoOutputFile,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct OutputSettings {
    grab: Vec<Grab>,
    format: Option<Format>,
    pretty: Option<bool>,
    output_file: Option<PathBuf>,
}

// TODO - should input validation live here or when merging final result with args. Probably when merging CLI args.
impl TryFrom<RawOutputSettings> for OutputSettings {
    type Error = FormatError;
    fn try_from(value: RawOutputSettings) -> Result<Self, Self::Error> {
        validate_grab(&value.grab)?;
        validate_format_pretty_combo(value.format, value.pretty)?;
        validate_format_output_file_combo(value.format, value.output_file.as_ref())?;

        Ok(OutputSettings {
            grab: value.grab,
            format: value.format,
            pretty: value.pretty,
            output_file: value.output_file,
        })
    }
}

fn validate_grab(grab: &[Grab]) -> Result<(), FormatError> {
    // If 1 or 0 fields in valid state
    if grab.len() <= 1 {
        return Ok(());
    }

    // int status code an full must be unique
    if grab.contains(&Grab::IntStatusCode) {
        Err(FormatError::InvalidGrabCombo("int status code".to_string()))
    } else if grab.contains(&Grab::Full) {
        Err(FormatError::InvalidGrabCombo("full".to_string()))
    } else {
        Ok(())
    }
}

fn validate_format_pretty_combo(
    format: Option<Format>,
    pretty: Option<bool>,
) -> Result<(), FormatError> {
    match (format, pretty) {
        (Some(f), Some(p)) => validate_format_pretty_combo_some(f, p),
        (Some(_), None) => Ok(()),
        (None, Some(_)) => Err(FormatError::PrettyWithoutFormat),
        (None, None) => Ok(()),
    }
}

fn validate_format_pretty_combo_some(format: Format, pretty: bool) -> Result<(), FormatError> {
    // If pretty is false - valid in all cases
    if !pretty {
        return Ok(());
    }

    match format {
        Format::Binary => Err(FormatError::PrettyWithBinary),
        _ => Ok(()),
    }
}

fn validate_format_output_file_combo(
    format: Option<Format>,
    output_file: Option<&PathBuf>,
) -> Result<(), FormatError> {
    match (format, output_file) {
        (Some(f), None) => match f {
            Format::Binary => Err(FormatError::BinaryWithNoOutputFile),
            _ => Ok(()),
        },
        (Some(_), Some(_)) => Ok(()),
        (None, Some(_)) => Ok(()),
        (None, None) => Ok(()),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawOutputSettings {
    grab: Vec<Grab>,
    format: Option<Format>,
    pretty: Option<bool>,
    output_file: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(try_from = "String")]
enum Grab {
    StatusCode,
    Headers,
    Body,
    Full,
    IntStatusCode,
}

impl TryFrom<String> for Grab {
    type Error = FormatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "status_code" => Ok(Self::StatusCode),
            "headers" => Ok(Self::Headers),
            "body" => Ok(Self::Body),
            "full" => Ok(Self::Full),
            "int_status_code" => Ok(Self::IntStatusCode),
            _ => Err(FormatError::InvalidGrab(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
enum Format {
    Http,
    Json,
    PrettyJson,
    Binary,
}

impl TryFrom<String> for Format {
    type Error = FormatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "http" => Ok(Self::Http),
            "json" => Ok(Self::Json),
            "pretty_json" => Ok(Self::PrettyJson),
            "binary" => Ok(Self::Binary),
            _ => Err(FormatError::InvalidFormat(value)),
        }
    }
}
