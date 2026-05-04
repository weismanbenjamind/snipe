use std::error::Error as StdErr;
use thiserror::Error;

// TODO - check out which intos are needed for RunError
#[derive(Debug, Error)]
pub enum RunError {
    #[error("{0}")]
    Failure(String),
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
pub enum VarReplaceError {
    #[error("{0}")]
    Base(String),
}

#[derive(Debug, Error)]
pub enum TargetsError {
    #[error("Failed to deserialize targets file. Error: {0}.")]
    Dersialization(String),
}

impl TargetsError {
    pub fn deserialization_from_err<T: StdErr>(err: T) -> Self {
        Self::Dersialization(err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Failed to build client. Error: {0}.")]
    ClientBuild(String),

    #[error("Failed to build request. Error: {0}.")]
    RequestBuild(String),

    #[error("Failed to send request. Error: {0}.")]
    SendRequestFailure(String),

    #[error("{0}")]
    ResponseBuild(String),
}

#[derive(Debug, Error)]
pub enum ResponseDataError {
    #[error("Failed to build response metadata. Error: {0}.")]
    Build(String),

    #[error("Failed to convert response data to string. Error {0}.")]
    ToString(String),

    #[error("Failed to convert response field {0} to string. Error {1}.")]
    ResponseFieldToString(String, String),

    #[error("Cannot convert a binary output to a String")]
    BinaryToString,
}

impl ResponseDataError {
    pub fn to_string_err_from_err<T: StdErr>(e: T) -> Self {
        Self::ToString(e.to_string())
    }

    pub fn new_response_field_to_string<T: StdErr>(field: &str, err: T) -> Self {
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
}

impl ArgsValidationError {
    pub fn new_base<T>(msg: &str) -> Result<T, Self> {
        Err(Self::base_from_str(msg))
    }

    pub fn base_from_str(value: &str) -> Self {
        Self::Base(value.into())
    }
}

#[derive(Debug, Error)]
pub enum CfgResolverError {
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
pub enum FilesystemError {
    #[error("Failed to create path to output path {0}. Error: {1}")]
    PathCreation(String, String),

    #[error("Failed to create file {0}. Error: {1}")]
    FileCreation(String, String),
}

#[derive(Debug, Error)]
pub enum ResponseWriterError {
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

impl From<ResponseDataError> for ResponseWriterError {
    fn from(value: ResponseDataError) -> Self {
        Self::Base(value.to_string())
    }
}

impl ResponseWriterError {
    pub fn binary_write_from_err<T: StdErr>(err: T) -> Self {
        Self::BinaryWrite(err.to_string())
    }
}
