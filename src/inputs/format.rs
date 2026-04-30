use crate::errors::ArgsValidationError;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
pub enum RawFormat {
    Http,
    Json,
    Binary,
}

impl From<Format> for RawFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::Http => Self::Http,
            Format::Json => Self::Json,
            Format::Binary => Self::Binary,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Http,
    Json,
    Binary,
}

impl Format {
    pub fn new(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
        match pretty {
            false => match raw_format {
                RawFormat::Json => Ok(Self::Json),
                RawFormat::Http => Ok(Self::Http),
                RawFormat::Binary => Ok(Self::Binary),
            },
            true => match raw_format {
                RawFormat::Json => Ok(Self::Json),
                RawFormat::Http => Err(ArgsValidationError::PrettyWithHTTP),
                RawFormat::Binary => Err(ArgsValidationError::PrettyWithBinary),
            },
        }
    }
}
