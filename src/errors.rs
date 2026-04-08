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
pub enum ClientError {
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

impl From<ResponseDataError> for ClientError {
    fn from(value: ResponseDataError) -> Self {
        Self::ResponseBuild(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ResponseDataError {
    #[error("Failed to build response metadata. Error: {0}.")]
    Build(String),

    #[error("Failed to derserialize response data. Error {0}.")]
    Derserialize(String),

    #[error("Failed to serialize response data. Error {0}")]
    Serialize(String),
}
