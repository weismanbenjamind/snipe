use crate::errors::TargetsError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
#[serde(try_from = "String")]
pub(crate) enum Method {
    Get,
    Delete,
    Post,
    Patch,
    Put,
}

impl TryFrom<String> for Method {
    type Error = TargetsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "delete" => Ok(Self::Delete),
            "post" => Ok(Self::Post),
            "patch" => Ok(Self::Patch),
            "put" => Ok(Self::Put),
            _ => Err(TargetsError::Dersialization(format!(
                "Failed to parse string {value} into HTTP request method."
            ))),
        }
    }
}
