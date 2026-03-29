use reqwest::StatusCode;
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

impl From<RequestSenderError> for RunError {
    fn from(value: RequestSenderError) -> Self {
        RunError::Failure(value.to_string())
    }
}

impl From<ResponseWrapperError> for RunError {
    fn from(value: ResponseWrapperError) -> Self {
        RunError::Failure(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum TargetsError {
    #[error("Failed to deserialize targets. Error: {0}.")]
    Dersialization(String),
}

impl TargetsError {
    pub fn deserialization_from_err<T: StdErr>(err: T) -> Self {
        Self::Dersialization(err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum RequestSenderError {
    #[error("Failed to build client. Error: {0}.")]
    ClientBuild(String),

    #[error("Failed to build request. Error: {0}.")]
    RequestBuild(String),

    #[error("Failed to send request. Error: {0}.")]
    SendRequestFailure(String),

    #[error("Bad response for request. Status code: {0}, Error: {1}")]
    BadResponse(StatusCode, String),

    #[error("{0}")]
    ResponseBuild(String),
}

impl From<ResponseWrapperError> for RequestSenderError {
    fn from(value: ResponseWrapperError) -> Self {
        Self::ResponseBuild(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ResponseWrapperError {
    #[error("Failed to build response metadata. Error: {0}.")]
    Build(String),

    #[error("Failed to derserialize ResponseWrapper. Error {0}.")]
    Derserialize(String),
}
