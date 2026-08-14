use crate::containers::TargetError;
use std::{error::Error as StdErr, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("{0}")]
    Failure(String),

    #[error("{0}")]
    ArgsValidation(#[from] ArgsValidationError),
}

impl From<&str> for RunError {
    fn from(value: &str) -> Self {
        Self::Failure(value.into())
    }
}

impl From<TargetsError> for RunError {
    fn from(value: TargetsError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<ClientError> for RunError {
    fn from(value: ClientError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<CfgResolverError> for RunError {
    fn from(value: CfgResolverError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<ResponseWriterError> for RunError {
    fn from(value: ResponseWriterError) -> Self {
        RunError::Failure(value.to_string())
    }
}

#[derive(Debug, Error)]
pub(crate) enum VarReplaceError {
    #[error("{0}")]
    Base(String),
}

#[derive(Debug, Error)]
pub(crate) enum TargetsError {
    #[error("Failed to deserialize targets file. Error: {0}")]
    Dersialization(String),

    #[error("Must specify 'file' or manually specify request params in request payload body.")]
    MissingPayloadFields,

    #[error("Can only specify 'file' or manually specify params in request payload. Not both.")]
    OverspecifiedPayload,

    #[error("Failed to write target to string. Error: {0}")]
    FailedToString(#[from] toml::ser::Error),

    #[error("{0}")]
    Target(#[from] TargetError),
}

impl TargetsError {
    pub(crate) fn deserialization_from_err<T: StdErr>(err: T) -> Self {
        Self::Dersialization(err.to_string())
    }
}

impl From<toml::de::Error> for TargetsError {
    fn from(value: toml::de::Error) -> Self {
        Self::Dersialization(value.to_string().trim_end_matches("\n").to_string())
    }
}

#[derive(Debug, Error)]
pub(crate) enum ClientError {
    #[error("Failed to build client. Error: {0}.")]
    ClientBuild(String),

    #[error("Failed to build request. Error: {0}.")]
    RequestBuild(String),

    #[error("Failed to send request. Error: {0}.")]
    SendRequestFailure(String),

    #[error("Failed to request body as {path} into bytes. Error {source}")]
    BodyToBytes {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
pub(crate) enum ResponseFormatterError {
    #[error("Failed to build response metadata. Error: {0}.")]
    Build(String),

    #[error("Failed to convert response data to string. Error {0}.")]
    ToString(String),

    #[error("Failed to convert response field {0} to string. Error {1}.")]
    ResponseFieldToString(String, String),

    #[error("Cannot convert a binary output to a String")]
    BinaryToString,
}

impl ResponseFormatterError {
    pub(crate) fn to_string_err_from_err<T: StdErr>(e: T) -> Self {
        Self::ToString(e.to_string())
    }

    pub(crate) fn new_response_field_to_string<T: StdErr>(field: &str, err: T) -> Self {
        Self::ResponseFieldToString(field.into(), err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ArgsValidationError {
    #[error("{0}")]
    Base(String),

    #[error("Cannot use pretty formatting. Invalid for HTTP string.")]
    PrettyWithHTTP,

    #[error("Cannot use pretty formatting. Invalid for Binary response.")]
    PrettyWithBinary,

    #[error("Cannot use Binary format and try to output a response field other than the body.")]
    NonBodyWithBinary,

    #[error("Must provide an output file for binary format")]
    NoOutputFileWithBinary,

    #[error("Value {0} is not a valid format.")]
    InvalidFormat(String),

    #[error("Cannot grab {0} with other response components")]
    InvalidGrab(&'static str),

    #[error("Must specify response components to grab either via CLI or via cfg file")]
    MissingGrab,

    #[error("No value passed for grab arg {0}")]
    GrabNotSet(&'static str),
}

#[derive(Debug, Error)]
pub(crate) enum CfgResolverError {
    #[error(
        "Failed to resolve path to configuration file. Path at {0} does not exist and value at env var {1} is not not set."
    )]
    UnresolvedCfgWithEnv(String, String),

    #[error("Failed to resolve path to configuration file. Path at {0} does not exist.")]
    UnresolvedCfg(String),

    #[error("{0}")]
    HomeDirExpansion(String),
}

#[derive(Debug, Error)]
pub(crate) enum FilesystemError {
    #[error("Failed to create path to output path {0}. Error: {1}")]
    PathCreation(String, String),

    #[error("Failed to create file {0}. Error: {1}")]
    FileCreation(String, String),
}

#[derive(Debug, Error)]
pub(crate) enum ResponseWriterError {
    #[error("{0}")]
    Base(String),

    #[error("Bad response with status code {0}. {1}.")]
    BadResponse(u16, String),

    #[error("Failed to write binary. Error: {0}")]
    BinaryWrite(String),

    #[error("Failed to write response to file {0}. Error: {1}")]
    TextWrite(String, String),
}

impl From<FilesystemError> for ResponseWriterError {
    fn from(value: FilesystemError) -> Self {
        Self::Base(value.to_string())
    }
}

impl From<ResponseFormatterError> for ResponseWriterError {
    fn from(value: ResponseFormatterError) -> Self {
        Self::Base(value.to_string())
    }
}

impl ResponseWriterError {
    pub(crate) fn binary_write_from_err<T: StdErr>(err: T) -> Self {
        Self::BinaryWrite(err.to_string())
    }
}
