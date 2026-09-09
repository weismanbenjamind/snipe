use log::info;

use super::{SnipeResult, SuccessMsg};
use crate::cfg_resolver::CfgResolver;
use crate::containers::Targets;
use crate::errors::RunError;
use crate::inputs::ListArgs;
use std::error::Error;
use std::fmt::Write;

pub(crate) fn run_list_targets_cmd(list_args: ListArgs) -> SnipeResult {
    info!("Generating target list.");

    let cfg_path = CfgResolver::new(&list_args.cfg, list_args.cfg_env.as_deref())
        .resolve_cfg_path_from_env()?;

    let targets = Targets::from_toml_file(&cfg_path)?;

    let mut target_names: Vec<&String> = targets.as_map().keys().collect();
    target_names.sort();

    let mut buf = String::new();
    target_names
        .iter()
        .try_for_each(|key| writeln!(buf, "{}", key).map_err(get_failed_get_targets_list_err))?;
    info!("Successfully generated target list.");

    Ok(SuccessMsg(buf.trim().to_string()))
}

#[inline]
fn get_failed_get_targets_list_err<T: Error>(e: T) -> RunError {
    RunError::Failure(format!("Failed to get targets list. Error: {}", e))
}
