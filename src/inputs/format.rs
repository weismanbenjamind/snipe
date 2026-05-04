use crate::errors::ArgsValidationError;
use clap::ValueEnum;
use serde::Serialize;
use std::path::Path;

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
    // Use this new method to create a validated Format enum from pretty
    pub fn new(
        raw_format: RawFormat,
        pretty: bool,
        output_file: Option<&Path>,
    ) -> Result<Self, ArgsValidationError> {
        let format = Self::init_format(raw_format, pretty)?;
        format.validate_output_file_for_binary(output_file)?;
        Ok(format)
    }

    #[inline]
    fn init_format(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
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

    #[inline]
    fn validate_output_file_for_binary(
        &self,
        output_file: Option<&Path>,
    ) -> Result<(), ArgsValidationError> {
        match output_file {
            Some(_) => Ok(()),
            None => match self {
                Self::Binary => Err(ArgsValidationError::NoOutputFileWithBinary),
                Self::Http | Self::Json => Ok(()),
            },
        }
    }
}
