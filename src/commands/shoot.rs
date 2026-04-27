use crate::client::Client;
use crate::errors::{ResponseDataError, RunError};
use crate::inputs::{Grab, RawFormat};
use crate::response_data::ResponseData;
use crate::targets::Targets;
use log::info;
use std::fs;
use std::path::Path;

use crate::errors::ArgsValidationError;
use crate::inputs::{Format, RawShootArgs};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ShootCmd {
    target: String,
    grab: Grab,
    format: Format,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl ShootCmd {
    pub fn new(
        target: String,
        grab: Grab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Result<Self, ArgsValidationError> {
        Ok(Self {
            target,
            grab,
            format: Format::new(format, pretty)?,
            pretty,
            output_file,
        })
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> Grab {
        self.grab
    }

    pub fn format(&self) -> RawFormat {
        self.format.into()
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> Option<&Path> {
        self.output_file.as_deref()
    }
}

impl TryFrom<RawShootArgs> for ShootCmd {
    type Error = ArgsValidationError;
    fn try_from(value: RawShootArgs) -> Result<Self, Self::Error> {
        let (target, raw_grab, raw_format, pretty, output_file) = value.into_parts();
        Ok(Self {
            target,
            grab: Grab::from(raw_grab),
            format: Format::new(raw_format, pretty)?,
            pretty,
            output_file,
        })
    }
}

impl ShootCmd {
    pub async fn run(&self, targets: &Targets) -> Result<(), RunError> {
        info!("Starting request process.");

        let target = targets
            .get_target(self.target())
            .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", self.target)))?;

        info!("Sending request for target '{}'.", target.name());
        let response_data = Client::new()?.send_request(target).await?;
        info!("Response recieved and ResponseData object built.");

        info!("Formatting response for output.");
        let output_string = match self.grab.int_status_code() {
            true => response_data.status_code_string(),
            false => {
                handle_formatted_output(&response_data, self.format(), self.grab, self.pretty)?
            }
        };
        info!("Response formatted for output.");

        match self.output_file() {
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
}

#[inline]
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
