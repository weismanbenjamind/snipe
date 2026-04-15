//use log::{debug, info};
use log::info;

use crate::client::Client;
use crate::errors::RunError;
use crate::inputs::{GrabSettings, SnipeArgs};
use crate::response_formatter::ResponseFormatter;
use crate::targets::Targets;
use std::fs;
use std::path::Path;

pub async fn run(args: SnipeArgs) -> Result<(), RunError> {
    env_logger::init();

    info!("Starting request process.");

    let targets = Targets::from_toml_file(args.cfg_path())?;

    let target = targets
        .get_target(args.target())
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", args.target())))?;

    info!("Sending request for target {}.", target.name());
    let _grab_settings = GrabSettings::from(args.grab());
    let response_data = Client::new()?.send_request(target).await?;
    info!("Response recieved.");

    info!("Formatting response for output.");
    let response_formatter = ResponseFormatter::from(&response_data);
    let output_string = match args.json() {
        true => response_formatter.get_json_string(args.grab(), args.pretty())?,
        false => response_formatter.get_http_string(args.grab())?,
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
