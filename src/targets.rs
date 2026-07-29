use crate::errors::TargetsError;
use crate::var_replacement::resolve_vars;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
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

    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, TargetsError> {
        info!(
            "Attempting to read toml file at path {}.",
            path.as_ref().display()
        );
        validate_toml_path(&path)?;
        let raw = read_toml(&path)?;
        info!("Toml file successfully read.");
        Self::from_toml(&raw)
    }

    pub fn from_toml(raw: &str) -> Result<Self, TargetsError> {
        info!("Attempting to replace user defined variables and environment varables in toml.");
        let toml_str = Self::as_string(raw)?;
        let maybe_vars: Option<Vars> = toml::from_str(raw).ok();
        let resolved_toml = resolve_vars(&toml_str, maybe_vars.as_ref(), None, None)
            .map_err(TargetsError::deserialization_from_err)?;
        info!("Variables replaced.");
        Ok(toml::from_str::<Self>(&resolved_toml)?)
    }

    fn as_string(raw: &str) -> Result<String, TargetsError> {
        // Need to parse into struct then back to string to get rid of comments
        let as_struct = toml::from_str::<Self>(raw)?;
        toml::to_string(&as_struct).map_err(TargetsError::from)
    }

    pub fn get_target(&self, target: &str) -> Option<&Target> {
        self.targets.get(target)
    }

    pub fn as_map(&self) -> &HashMap<String, Target> {
        &self.targets
    }
}

fn validate_toml_path<P: AsRef<Path>>(path: P) -> Result<(), TargetsError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(TargetsError::Dersialization(format!(
            "Path {} does not exist.",
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
    name: Option<String>,
    url: String,
    method: Method,
    timeout_seconds: Option<u64>,
    headers: Option<HashMap<String, String>>,
    auth: Option<Auth>,
    payload: Option<Payload>,
}

impl Target {
    #[allow(dead_code)]
    pub fn new(
        name: Option<String>,
        url: &str,
        method: Method,
        timeout_seconds: Option<u64>,
        headers: Option<HashMap<String, String>>,
        auth: Option<Auth>,
        payload: Option<Payload>,
    ) -> Self {
        Self {
            name,
            url: url.to_string(),
            method,
            timeout_seconds,
            headers,
            auth,
            payload,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
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

    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawPayload {
    file: Option<PathBuf>,

    #[serde(flatten)]
    params: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "RawPayload")]
pub enum Payload {
    File(PathBuf),
    Params(HashMap<String, Value>),
}

impl TryFrom<RawPayload> for Payload {
    type Error = TargetsError;
    fn try_from(value: RawPayload) -> Result<Self, Self::Error> {
        debug!(
            "Attempting to parse RawPayload with file {:?} and params {:?}",
            value.file, value.params
        );
        match (value.file, value.params) {
            (None, None) => Err(TargetsError::MissingPayloadFields),
            (Some(file), Some(params)) => match params.is_empty() {
                true => Ok(Self::File(file)),
                false => Err(TargetsError::OverspecifiedPayload),
            },
            (Some(file), None) => Ok(Self::File(file)),
            (None, Some(params)) => match params.is_empty() {
                true => Err(TargetsError::MissingPayloadFields),
                false => Ok(Self::Params(params)),
            },
        }
    }
}
