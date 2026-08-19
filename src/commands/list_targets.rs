use log::info;

use super::{SnipeResult, SuccessMsg};
use crate::containers::Targets;
use crate::errors::RunError;
use std::error::Error;
use std::fmt::Write;

pub(crate) fn run_list_targets_cmd(targets: Targets) -> SnipeResult {
    info!("Getting target list.");

    info!("Writing target names to buffer.");
    let mut target_names: Vec<&String> = targets.as_map().keys().collect();
    target_names.sort();

    let mut buf = String::new();
    target_names
        .iter()
        .try_for_each(|key| writeln!(buf, "{}", key).map_err(get_failed_get_targets_list_err))?;
    info!("Targets writtin to buffer. Displaying.");

    info!("Done getting targets list.");
    Ok(SuccessMsg(buf.trim().to_string()))
}

#[inline]
fn get_failed_get_targets_list_err<T: Error>(e: T) -> RunError {
    RunError::Failure(format!("Failed to get targets list. Error: {}", e))
}
