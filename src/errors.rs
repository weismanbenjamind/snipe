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

    #[error("Failed to convert response data to string. Error {0}")]
    ToString(String),
}

impl ResponseDataError {
    pub fn to_string_err_from_err<T: StdErr>(e: T) -> Self {
        Self::ToString(e.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ArgsValidationError {
    #[error("{0}")]
    Base(String),

    #[error("Cannot pretty formatting invalid for HTTP String")]
    PrettyWithHTTP,
}

impl ArgsValidationError {
    pub fn new_base<T>(msg: &str) -> Result<T, Self> {
        Err(Self::base_from_str(msg))
    }

    pub fn base_from_str(value: &str) -> Self {
        Self::Base(value.into())
    }
}
