use crate::containers::{Target, Vars};
use crate::errors::TargetsError;
use crate::var_replacement::resolve_vars;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use std::{collections::HashMap, ffi::OsStr};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Targets {
    targets: HashMap<String, Target>,
}

impl Targets {
    #[allow(dead_code)]
    pub fn new(targets: HashMap<String, Target>) -> Self {
        Self { targets }
    }

    // Note Targets is never meant to be written back to a toml file
    // It's a runtime artifact full of replaced variables, environment variables, potential secrets, etc.
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, TargetsError> {
        info!(
            "Attempting to read toml file at path {}.",
            path.as_ref().display()
        );
        validate_toml_path(&path)?;
        let raw = read_toml(&path)?;
        info!("Toml file successfully read.");
        let targets = Self::from_toml(&raw)?;
        debug!("Parsed targets file as: {:#?}", targets);
        Ok(targets)
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
        // Note - this will parse the actual secret values to the string (not redact them)
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
