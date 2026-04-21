use crate::errors::CfgResolverError;
use log::info;
use std::env;
use std::path::{Path, PathBuf};

pub struct CfgResolver<'a> {
    cfg_path: &'a Path,
    cfg_env_var: Option<&'a str>,
}

impl<'a> CfgResolver<'a> {
    pub fn new(cfg_path: &'a Path, cfg_env_var: Option<&'a str>) -> Self {
        Self {
            cfg_path,
            cfg_env_var,
        }
    }

    pub fn resolve_cfg_path_from_env(self) -> Result<PathBuf, CfgResolverError> {
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
        found.ok_or_else(|| self.get_unresolved_cfg_err())
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
