use crate::errors::ArgsValidationError;
use clap::ValueEnum;
use serde::Serialize;
use std::path::Path;

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
pub enum RawFormat {
    Http,
    Json,
    PrettyJson,
    Binary,
}

#[derive(Clone, Copy, Debug)]
pub struct ValidatedFormat {
    raw_format: RawFormat,
}

impl ValidatedFormat {
    pub fn raw_format(&self) -> RawFormat {
        self.raw_format
    }

    pub fn new_validated(
        raw_format: RawFormat,
        pretty: bool,
        output_file: Option<&Path>,
    ) -> Result<Self, ArgsValidationError> {
        Self::init_from_pretty(raw_format, pretty)?.validate_output_file_for_binary(output_file)
    }

    #[inline]
    fn init_from_pretty(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
        match pretty {
            false => Ok(Self { raw_format }),
            true => match raw_format {
                RawFormat::Json | RawFormat::Http | RawFormat::PrettyJson => {
                    Ok(Self { raw_format })
                }
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
            None => match self.raw_format {
                RawFormat::Http | RawFormat::Json | RawFormat::PrettyJson => Ok(self),
                RawFormat::Binary => Err(ArgsValidationError::NoOutputFileWithBinary),
            },
        }
    }
}
