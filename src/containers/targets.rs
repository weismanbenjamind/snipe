use crate::containers::Vars;
use crate::containers::globals::{Globals, GlobalsCfg};
use crate::containers::target::{GlobalReplaceableTarget, Target, TargetError};
use crate::errors::TargetsError;
use crate::var_replacement::resolve_vars;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use std::{collections::HashMap, ffi::OsStr};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct Targets {
    targets: HashMap<String, Target>,
}

impl Targets {
    #[allow(dead_code)]
    pub(crate) fn new(targets: HashMap<String, Target>) -> Self {
        Self { targets }
    }

    // Note Targets is never meant to be written back to a toml file
    // It's a runtime artifact full of replaced variables, environment variables, potential secrets, etc.
    pub(crate) fn from_toml_file<P: AsRef<Path>>(path: &P) -> Result<Self, TargetsError> {
        info!(
            "Generating Targets from .toml file at {}",
            path.as_ref().display()
        );

        let raw = read_toml(&path)?;
        let resolved_toml = replace_vars(&raw)?;
        let to_replace: GlobalReplaceableTargets = toml::from_str(&resolved_toml)?;
        let globals: Option<Globals> = toml::from_str::<GlobalsCfg>(&resolved_toml)?.globals;

        info!("Replacing globals in targets file.");
        let replaced = to_replace.into_targets(globals.as_ref())?;
        info!("Succesfully replaced globals in targets file.");

        info!(
            "Succesfully generated targets from .toml file at {}",
            path.as_ref().display()
        );
        debug!("Parsed targets file as:\n{replaced:#?}");

        Ok(replaced)
    }

    pub(crate) fn get_target(&self, target: &str) -> Option<&Target> {
        self.targets.get(target)
    }

    pub(crate) fn as_map(&self) -> &HashMap<String, Target> {
        &self.targets
    }
}

type GlobalReplaceableTargetsType = HashMap<String, GlobalReplaceableTarget>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct GlobalReplaceableTargets {
    targets: GlobalReplaceableTargetsType,
}

impl GlobalReplaceableTargets {
    fn into_targets(self, globals: Option<&Globals>) -> Result<Targets, TargetsError> {
        self.targets
            .into_iter()
            .map(|(k, v)| Ok((k, v.into_target(globals)?)))
            .collect::<Result<HashMap<String, Target>, TargetError>>()
            .map_err(TargetsError::from)
            .map(Targets::new)
    }
}

fn read_toml<P: AsRef<Path>>(path: &P) -> Result<String, TargetsError> {
    debug!(
        "Attempting to read .toml file at {}",
        path.as_ref().display()
    );
    validate_toml_path(path)?;
    let result = read_to_string(path).map_err(|e| {
        TargetsError::Dersialization(format!("Failed to read toml to string. Error: {e}",))
    })?;
    debug!(
        "Successfully read .toml file at {}",
        path.as_ref().display()
    );
    Ok(result)
}

fn validate_toml_path<P: AsRef<Path>>(path: &P) -> Result<(), TargetsError> {
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

fn replace_vars(raw: &str) -> Result<String, TargetsError> {
    info!(
        "Attempting to replace user defined variables and environment varables in global replaceable toml."
    );
    let toml_str = get_replacement_string(raw)?;
    let maybe_vars: Option<Vars> = toml::from_str(raw).ok();
    let resolved_toml = resolve_vars(&toml_str, maybe_vars.as_ref(), None, None)
        .map_err(TargetsError::deserialization_from_err)?;
    info!("Variables replaced.");
    Ok(resolved_toml)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct VarReplaceTarget {
    globals: Option<Globals>,
    targets: GlobalReplaceableTargetsType,
}

fn get_replacement_string(raw: &str) -> Result<String, TargetsError> {
    // Need to parse into struct then back to string to get rid of comments
    // Note - this will parse the actual secret values to the string (not redact them)
    let as_struct = toml::from_str::<VarReplaceTarget>(raw)?;
    toml::to_string(&as_struct).map_err(TargetsError::from)
}
