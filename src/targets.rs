use crate::errors::TargetsError;
use crate::var_replacement::resolve_vars;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use std::{collections::HashMap, ffi::OsStr};
use toml::Value;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Targets {
    targets: HashMap<String, Target>,
}

impl Targets {
    #[allow(dead_code)]
    pub fn new(targets: HashMap<String, Target>) -> Self {
        Self { targets }
    }

    // TODO - add ability to resolve special characters like ~
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, TargetsError> {
        validate_toml_path(&path)?;
        let toml_str = read_toml(&path)?;
        Self::from_toml(&toml_str)
    }

    pub fn from_toml(toml_str: &str) -> Result<Self, TargetsError> {
        let maybe_vars: Option<Vars> =
            toml::from_str(toml_str).map_err(TargetsError::deserialization_from_err)?;
        let resolved_toml = resolve_vars(toml_str, maybe_vars.as_ref(), None, None)
            .map_err(TargetsError::deserialization_from_err)?;
        toml::from_str(&resolved_toml).map_err(TargetsError::deserialization_from_err)
    }

    pub fn get_target(&self, target: &str) -> Option<&Target> {
        self.targets.get(target)
    }
}

fn validate_toml_path<P: AsRef<Path>>(path: P) -> Result<(), TargetsError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(TargetsError::Dersialization(format!(
            "Path {} does not exist",
            path.display()
        )));
    }
    if !path.is_file() {
        return Err(TargetsError::Dersialization(format!(
            "Path {} is not a file",
            path.display()
        )));
    }
    check_for_toml_extension(path)?;

    Ok(())
}

fn check_for_toml_extension(path: &Path) -> Result<(), TargetsError> {
    match path.extension() {
        None => Err(get_toml_extension_err(path)),
        Some(extension) => match extension == OsStr::new("toml") {
            true => Ok(()),
            false => Err(get_toml_extension_err(path)),
        },
    }
}

fn get_toml_extension_err(path: &Path) -> TargetsError {
    TargetsError::Dersialization(format!("Expected toml file, found {}", path.display()))
}

fn read_toml<P: AsRef<Path>>(path: P) -> Result<String, TargetsError> {
    read_to_string(path).map_err(|e| {
        TargetsError::Dersialization(format!("Failed to read toml to string. Error: {e}",))
    })
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Target {
    name: String,
    url: String,
    method: Method,
    timeout_seconds: Option<u64>,
    headers: Option<HashMap<String, String>>,
    auth: Option<Auth>,
    payload: Option<HashMap<String, Value>>,
}

impl Target {
    #[allow(dead_code)]
    pub fn new(
        name: &str,
        url: &str,
        method: Method,
        timeout_seconds: Option<u64>,
        headers: Option<HashMap<String, String>>,
        auth: Option<Auth>,
        payload: Option<HashMap<String, Value>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            method,
            timeout_seconds,
            headers,
            auth,
            payload,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn timeout_seconds(&self) -> Option<u64> {
        self.timeout_seconds
    }

    pub fn headers(&self) -> &Option<HashMap<String, String>> {
        &self.headers
    }

    pub fn auth(&self) -> &Option<Auth> {
        &self.auth
    }

    pub fn payload(&self) -> &Option<HashMap<String, Value>> {
        &self.payload
    }
}

#[derive(Debug, Deserialize, Clone)]
struct RawAuth {
    scheme: String,
    token: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(tag = "scheme", try_from = "RawAuth")]
pub enum Auth {
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
pub struct BearerAuth {
    token: String,
}

impl BearerAuth {
    #[allow(dead_code)]
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    #[allow(dead_code)]
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
#[serde(try_from = "String")]
pub enum Method {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vars {
    vars: HashMap<String, String>,
}

impl Vars {
    pub fn get(&self, value: &str) -> Option<&str> {
        self.vars.get(value).map(|val| val.as_str())
    }
}
