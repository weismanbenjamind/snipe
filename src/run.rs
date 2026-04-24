use log::info;

use crate::cfg_resolver::CfgResolver;
use crate::client::Client;
use crate::errors::{ResponseDataError, RunError};
use crate::inputs::{Command, Grab, RawFormat, ShootArgs, SnipeCLIArgs};
use crate::response_data::ResponseData;
use crate::targets::Targets;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::path::Path;

pub async fn run_cli(snipe_cli_args: SnipeCLIArgs) -> Result<(), RunError> {
    match snipe_cli_args.command() {
        Command::ListTargets => list_targets(snipe_cli_args.cfg(), snipe_cli_args.cfg_env()),
        Command::Shoot(shoot_args) => shoot(shoot_args).await,
    }
}

// TODO - Maybe make this a method off the List targets enum? - House a run_cli argument that just matches the command then calls run off the command - or even house this command off the CLI iteself
pub fn list_targets<P: AsRef<Path>>(cfg_path: P, cfg_env: Option<&str>) -> Result<(), RunError> {
    env_logger::init();
    info!("Getting target list.");

    let cfg_path = CfgResolver::new(cfg_path.as_ref(), cfg_env).resolve_cfg_path_from_env()?;

    info!("Creating targets from file at {}.", cfg_path.display());
    let targets = Targets::from_toml_file(&cfg_path)?;
    info!("Targets successfully created.");

    info!("Writing target names to buffer");
    let mut buf = String::new();
    targets
        .as_map()
        .keys()
        .try_for_each(|key| writeln!(buf, "{}", key).map_err(get_failed_get_targets_list_err))?;
    info!("Targets writtin");

    print!("{buf}");

    info!("Done getting targets list.");
    Ok(())
}

#[inline]
fn get_failed_get_targets_list_err<T: Error>(e: T) -> RunError {
    RunError::Failure(format!("Failed to get targets list. Error: {}", e))
}

// TODO - maybe make this a method off the ShootArgs class? - House a run_cli argument that just matches the command then calls run off the command - or even house this command off the CLI itself
// // When make this a method way want to remove the borrow to &ShootArgs
pub async fn shoot(args: &ShootArgs) -> Result<(), RunError> {
    env_logger::init();

    info!("Starting request process.");

    let cfg_path = CfgResolver::new(args.cfg_path(), args.cfg_env()).resolve_cfg_path_from_env()?;

    info!("Creating targets from file at {}.", cfg_path.display());
    let targets = Targets::from_toml_file(&cfg_path)?;
    info!("Targets successfully created.");

    let target = targets
        .get_target(args.target())
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", args.target())))?;

    info!("Sending request for target {}.", target.name());
    let response_data = Client::new()?.send_request(target).await?;
    info!("Response recieved.");

    info!("Formatting response for output.");
    let grab = args.grab();
    let output_string = match grab.int_status_code() {
        true => response_data.status_code_string(),
        false => handle_formatted_output(&response_data, args.format(), grab, args.pretty())?,
    };

    info!("Response formatted for output.");

    match args.output_file() {
        None => {
            info!("Printing output to console.");
            println!("{output_string}");
            info!("Successfully printed output to console.");
        }
        Some(output_path) => {
            info!("Writing response to file {}.", output_path.display());
            write_response_to_file(output_path, &output_string)?;
            info!(
                "Successfully wrote response to file {}.",
                output_path.display()
            );
        }
    }

    info!("Finished request process.");

    Ok(())
}

fn handle_formatted_output(
    response_data: &ResponseData,
    format: RawFormat,
    grab: Grab,
    pretty: bool,
) -> Result<String, ResponseDataError> {
    match format {
        RawFormat::Http => {
            response_data.to_http_string(grab.status_code(), grab.headers(), grab.body())
        }
        RawFormat::Json => {
            response_data.to_json_string(grab.status_code(), grab.headers(), grab.body(), pretty)
        }
    }
}

fn write_response_to_file<P: AsRef<Path>>(output_path: P, response: &str) -> Result<(), RunError> {
    let as_ref = output_path.as_ref();

    if let Some(parent) = as_ref.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            RunError::Failure(format!(
                "Failed to create path to output path {}. Error {e}",
                parent.display()
            ))
        })?;
    }

    fs::write(as_ref, response).map_err(|e| {
        RunError::Failure(format!(
            "Failed to write results to output file {}. Error {e}",
            as_ref.display()
        ))
    })?;

    Ok(())
}
