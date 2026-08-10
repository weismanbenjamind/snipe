use log::info;

use crate::containers::Targets;
use crate::errors::RunError;
use std::error::Error;
use std::fmt::Write;

pub(crate) fn run_list_targets_cmd(targets: &Targets) -> Result<(), RunError> {
    info!("Generating targets list.");
    let mut target_names: Vec<&String> = targets.as_map().keys().collect();
    target_names.sort();

    let mut buf = String::new();
    target_names
        .iter()
        .try_for_each(|key| writeln!(buf, "{}", key).map_err(get_failed_get_targets_list_err))?;

    info!("Successfully generated targets list. Displaying.");
    print!("{buf}");

    Ok(())
}

#[inline]
fn get_failed_get_targets_list_err<T: Error>(e: T) -> RunError {
    RunError::Failure(format!("Failed to get targets list. Error: {}", e))
}
