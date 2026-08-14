use crate::errors::ArgsValidationError;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::Path;

// TODO - Might need special de-serialization logic here on the enum to turn it back into a string
#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(try_from = "String")]
pub(crate) enum RawFormat {
    Http,
    Json,
    PrettyJson,
    Binary,
}

impl TryFrom<String> for RawFormat {
    type Error = ArgsValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "http" => Ok(Self::Http),
            "json" => Ok(Self::Json),
            "pretty_json" => Ok(Self::PrettyJson),
            "binary" => Ok(Self::Binary),
            _ => Err(ArgsValidationError::InvalidFormat(value)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ValidatedFormat(RawFormat);

impl ValidatedFormat {
    // TODO - may want to tweak encap here so only way to build ValidtedGrab is via a `new` method that constains validation
    pub(crate) fn new(raw_format: RawFormat) -> Self {
        Self(raw_format)
    }

    pub(crate) fn raw(&self) -> RawFormat {
        self.0
    }

    pub(crate) fn new_validated(
        raw_format: RawFormat,
        pretty: bool,
        output_file: Option<&Path>,
    ) -> Result<Self, ArgsValidationError> {
        Self::init_from_pretty(raw_format, pretty)?.validate_output_file_for_binary(output_file)
    }

    #[inline]
    fn init_from_pretty(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
        match pretty {
            false => Ok(Self(raw_format)),
            true => match raw_format {
                RawFormat::Json | RawFormat::Http | RawFormat::PrettyJson => Ok(Self(raw_format)),
                RawFormat::Binary => Err(ArgsValidationError::PrettyWithBinary),
            },
        }
    }

    #[inline]
    fn validate_output_file_for_binary(
        self,
        output_file: Option<&Path>,
    ) -> Result<Self, ArgsValidationError> {
        match output_file {
            Some(_) => Ok(self),
            None => match self.0 {
                RawFormat::Http | RawFormat::Json | RawFormat::PrettyJson => Ok(self),
                RawFormat::Binary => Err(ArgsValidationError::NoOutputFileWithBinary),
            },
        }
    }
}
