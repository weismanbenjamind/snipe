use std::{
    collections::HashMap,
    error::Error,
    fmt::Write,
    path::{Path, PathBuf},
};

use log::info;
use serde::Deserialize;
use thiserror::Error;

use super::{SnipeResult, SuccessMsg};
use crate::{cfg_resolver::CfgResolver, errors::RunError, inputs::ListArgs};

#[derive(Debug, Error)]
pub enum ListError {
    #[error("Failed to read targets file at {path}. Error: {source}")]
    FileRead {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("Failed to deserailze targets file at {path}. Error: {source}")]
    Deserialize {
        path: PathBuf,

        #[source]
        source: toml::de::Error,
    },
}

pub(crate) fn run_list_targets_cmd(list_args: ListArgs) -> SnipeResult {
    info!("Generating target list.");

    let cfg_path = CfgResolver::new(&list_args.cfg, list_args.cfg_env.as_deref())
        .resolve_cfg_path_from_env()?;

    let targets = TargetsForList::from_toml_file(&cfg_path)?;

    let mut target_names: Vec<&String> = targets.map.keys().collect();
    target_names.sort();

    let mut buf = String::new();
    target_names
        .iter()
        .try_for_each(|key| writeln!(buf, "{}", key).map_err(get_failed_get_targets_list_err))?;
    info!("Successfully generated target list.");

    Ok(SuccessMsg(buf.trim().to_string()))
}

#[derive(Debug, Clone, Deserialize)]
struct TargetsForList {
    #[serde(rename = "targets")]
    map: HashMap<String, toml::Value>,
}

impl TargetsForList {
    fn from_toml_file(path: &Path) -> Result<Self, ListError> {
        info!("Generating Targets from file {}.", path.display());

        let raw = std::fs::read(path).map_err(|source| ListError::FileRead {
            path: path.into(),
            source,
        })?;

        let targets =
            toml::from_slice::<TargetsForList>(&raw).map_err(|source| ListError::Deserialize {
                path: path.into(),
                source,
            })?;

        info!(
            "Succesfully generated Targets from file {}.",
            path.display()
        );

        Ok(targets)
    }
}

#[inline]
fn get_failed_get_targets_list_err<T: Error>(e: T) -> RunError {
    RunError::Failure(format!("Failed to get targets list. Error: {}", e))
}
