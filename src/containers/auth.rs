use crate::containers::SecretString;
use crate::containers::globals::GlobalReplaceableLocal;
use crate::errors::TargetsError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct RawAuth {
    scheme: String,
    token: Option<SecretString>,
    username: Option<String>,
    password: Option<SecretString>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(tag = "scheme", try_from = "RawAuth")]
pub(crate) enum Auth {
    Bearer(BearerAuth),
    Basic(BasicAuth),
}

impl TryFrom<RawAuth> for Auth {
    type Error = TargetsError;

    fn try_from(value: RawAuth) -> Result<Self, Self::Error> {
        match value.scheme.to_lowercase().as_str() {
            "bearer" => {
                let token = value
                    .token
                    .ok_or_else(|| get_missing_auth_field_err("token", "bearer"))?;
                Ok(Self::Bearer(BearerAuth { token }))
            }
            "basic" => {
                let username = value
                    .username
                    .ok_or_else(|| get_missing_basic_auth_field_error("username"))?;
                let password = value
                    .password
                    .ok_or_else(|| get_missing_basic_auth_field_error("password"))?;
                Ok(Self::Basic(BasicAuth { username, password }))
            }
            _ => Err(TargetsError::Dersialization(format!(
                "Invalud auth scheme {}.",
                value.scheme
            ))),
        }
    }
}

impl GlobalReplaceableLocal for Auth {
    fn has_local(&self) -> bool {
        true
    }
}

#[inline]
fn get_missing_basic_auth_field_error(field_name: &str) -> TargetsError {
    get_missing_auth_field_err(field_name, "basic")
}

#[inline]
fn get_missing_auth_field_err(field_name: &str, auth_type: &str) -> TargetsError {
    TargetsError::Dersialization(format!(
        "Must pass {field_name} field for {auth_type} auth."
    ))
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct BearerAuth {
    pub(crate) token: SecretString,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct BasicAuth {
    pub(crate) username: String,
    pub(crate) password: SecretString,
}
