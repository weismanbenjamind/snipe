use crate::errors::CfgResolverError;
use std::env;
use std::path::{Path, PathBuf};

pub struct CfgResolver<'a> {
    cfg_path: &'a Path,
    cfg_env_var: &'a str,
}

impl<'a> CfgResolver<'a> {
    pub fn new(cfg_path: &'a Path, cfg_env_var: &'a str) -> Self {
        Self {
            cfg_path,
            cfg_env_var,
        }
    }

    pub fn resolve_cfg_path_from_env(self) -> Result<PathBuf, CfgResolverError> {
        let found = match self.cfg_path.exists() {
            true => Ok(self.cfg_path.to_path_buf()),
            false => env::var(self.cfg_env_var).map(PathBuf::from),
        };
        found.map_err(|_| self.get_unresolved_cfg_err())
    }

    fn get_unresolved_cfg_err(&'a self) -> CfgResolverError {
        CfgResolverError::UnresolvedCfg(
            self.cfg_path.display().to_string(),
            self.cfg_env_var.to_string(),
        )
    }
}
