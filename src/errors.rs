use thiserror::Error;
use std::error::Error as StdErr;
use reqwest::StatusCode;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("{0}")]
    Failure(String)
}

impl From<TargetsError> for RunError {
    fn from(value: TargetsError) -> Self {
        RunError::Failure(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum TargetsError {
    #[error("Failed to deserialize targets. Error: {0}")]
    Dersialization(String)
}

impl TargetsError {
    pub fn deserialization_from_err<T: StdErr>(err: T) -> Self {
        Self::Dersialization(err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum RequestManagerError {
    #[error("Failed to build request. Error: (0)")]
    RequestBuild(String),

    #[error("Failed to send request. Error: {0}")]
    SendRequestFailure(String),

    #[error("Bad response for request. Status code: {0}, Error: {1}")]
    BadResponse(StatusCode, String)
}