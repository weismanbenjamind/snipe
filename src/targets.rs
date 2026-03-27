use serde::Deserialize;
use std::{collections::HashMap, ffi::OsStr};
use toml::Value;
use std::fs::read_to_string;
use std::path::Path;
use crate::errors::{MethodError, TargetsError};

#[derive(Debug, Deserialize)]
pub struct Targets {
    targets: HashMap<String, Target>
}

impl Targets {
    pub fn from_toml_file<P :AsRef<Path>>(path: P) -> Result<Self, TargetsError> {
        validate_toml_path(&path)?;
        let toml_str = read_toml(&path)?;
        Self::from_toml(&toml_str)
    }

    pub fn from_toml(toml_str: &str) -> Result<Self, TargetsError> {
        toml::from_str(toml_str).map_err(|err| TargetsError::deserialization_from_err(err))
    }
}

fn validate_toml_path<P: AsRef<Path>>(path: P) -> Result<(), TargetsError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(TargetsError::Dersialization(format!("Path {} does not exist.", path.display())))
    }
    if !path.is_file() {
        return Err(TargetsError::Dersialization(format!("Path {} is not a file.", path.display())))
    }
    check_for_toml_extension(path)?;

    Ok(())
}

fn read_toml<P: AsRef<Path>>(path: P) -> Result<String, TargetsError> {
    read_to_string(path).map_err(|e| TargetsError::Dersialization(format!("Failed to read toml to string. Error: {}", e.to_string())))
}

fn check_for_toml_extension(path: &Path) -> Result<(), TargetsError> {
    match path.extension() {
        None => Err(TargetsError::Dersialization(format!("Expected toml file, found path {}", path.display()))),
        Some(extension) => {
            match extension == OsStr::new(".toml") {
                true => Ok(()),
                false => Err(TargetsError::Dersialization(format!("Expected toml file, found path {}", path.display())))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Target {
    name: String,
    method: Method,
    headers: Option<HashMap<String, String>>,
    auth: Option<HashMap<String, String>>,
    payload: Option<HashMap<String, Value>>
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
enum Method {
    GET,
    DELETE,
    POST,
    PATCH,
    PUT
}

impl TryFrom<String> for Method {
    type Error = MethodError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(Self::GET),
            "delete" => Ok(Self::DELETE),
            "post" => Ok(Self::POST),
            "patch" => Ok(Self::PATCH),
            "put" => Ok(Self::PUT),
            _ => Err(MethodError::Dersialization(value)),
        }
    }
}