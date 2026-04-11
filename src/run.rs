//use log::{debug, info};
use log::info;

use crate::client::Client;
use crate::errors::RunError;
use crate::inputs::SnipeArgs;
use crate::targets::Targets;
// use serde_json::to_string_pretty;
use std::fs;
use std::path::Path;

pub async fn run(args: SnipeArgs) -> Result<(), RunError> {
    env_logger::init();

    let targets = Targets::from_toml_file(args.cfg_path())?;

    let target = targets
        .get_target(args.target())
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", args.target())))?;

    info!("Sending request for target {}", target.name());
    let response = Client::new()?.send_request(target).await?;

    let output_string = match args.json() {
        true => response
            .to_json_string(args.grab(), args.pretty())
            .map_err(|e| RunError::Failure(e.to_string()))?,
        false => response.to_http_string(args.grab()),
    };

    match args.output_file() {
        None => println!("{output_string}"),
        Some(output_path) => {
            info!("Writing HTTP response to file {}.", output_path.display());
            write_response_to_file(output_path, &output_string)?
        }
    }

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
