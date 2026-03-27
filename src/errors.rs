use thiserror::Error;
use std::error::Error as StdErr;

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
pub enum MethodError {
    #[error("Failed to parse string {0} into HTTP request method.")]
    Dersialization(String)
}