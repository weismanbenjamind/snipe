use crate::errors::CfgResolverError;
use log::info;

use std::env;
use std::path::{Path, PathBuf};

const HOME_DELMITER: &str = "~";

pub(crate) struct CfgResolver<'a> {
    cfg_path: &'a Path,
    cfg_env_var: Option<&'a str>,
}

impl<'a> CfgResolver<'a> {
    pub(crate) fn new(cfg_path: &'a Path, cfg_env_var: Option<&'a str>) -> Self {
        Self {
            cfg_path,
            cfg_env_var,
        }
    }

    pub(crate) fn resolve_cfg_path_from_env(self) -> Result<PathBuf, CfgResolverError> {
        let found = match self.cfg_path.exists() {
            true => {
                info!("Passed config file path exists. Skipping using env to find config.");
                Some(self.cfg_path.to_path_buf())
            }
            false => {
                info!(
                    "Passed config file path does not exist. Attempting to resolve config file path from env."
                );
                self.cfg_env_var
                    .and_then(|key| env::var(key).ok().map(PathBuf::from))
            }
        };
        let found = found.ok_or_else(|| self.get_unresolved_cfg_err())?;
        match found.starts_with(HOME_DELMITER) {
            true => replace_home_dir(found),
            false => Ok(found),
        }
    }

    fn get_unresolved_cfg_err(&'a self) -> CfgResolverError {
        match self.cfg_env_var {
            Some(key) => CfgResolverError::UnresolvedCfgWithEnv(
                self.cfg_path.display().to_string(),
                key.to_string(),
            ),
            None => CfgResolverError::UnresolvedCfg(self.cfg_path.display().to_string()),
        }
    }
}

fn replace_home_dir(path: PathBuf) -> Result<PathBuf, CfgResolverError> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        CfgResolverError::HomeDirExpansion("Failed to resolve home dir.".to_string())
    })?;

    let to_join = path.strip_prefix(HOME_DELMITER).map_err(|e| {
        CfgResolverError::HomeDirExpansion(format!("Failed to replace home directory. Error: {e}"))
    })?;

    Ok(home_dir.join(to_join))
}
