use std::error::Error as StdErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("{0}")]
    Failure(String),
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

impl From<ResponseDataError> for RunError {
    fn from(value: ResponseDataError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<ArgsValidationError> for RunError {
    fn from(value: ArgsValidationError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<CfgResolverError> for RunError {
    fn from(value: CfgResolverError) -> Self {
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

impl From<ResponseDataError> for ClientError {
    fn from(value: ResponseDataError) -> Self {
        Self::ResponseBuild(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ResponseDataError {
    #[error("Failed to build response metadata. Error: {0}.")]
    Build(String),

    #[error("Failed to convert response data to string. Error {0}.")]
    ToString(String),

    #[error("Failed to convert response field {0} to string. Error {1}.")]
    ResponseFieldToString(String, String),
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

    #[error("Cannot use pretty formatting. Invalid for HTTP tring.")]
    PrettyWithHTTP,

    #[error("Cannot use pretty formatting. Invalid for Binary response.")]
    PrettyWithBinary,

    #[error("Cannot use Binary format and try to output a response field other than the body.")]
    NonBodyWithBinary,
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
