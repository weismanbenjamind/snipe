use crate::errors::ArgsValidationError;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
pub enum RawFormat {
    Http,
    Json,
}

impl From<Format> for RawFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::Http => Self::Http,
            Format::Json => Self::Json,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Http,
    Json,
}

impl Format {
    pub fn new(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
        match raw_format {
            RawFormat::Http => match pretty {
                true => Err(ArgsValidationError::PrettyWithHTTP),
                false => Ok(Self::Http),
            },
            RawFormat::Json => Ok(Self::Json),
        }
    }
}
