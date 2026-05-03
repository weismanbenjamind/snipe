use crate::client::Client;
use crate::errors::RunError;
use crate::inputs::{Grab, RawFormat};
use crate::targets::Targets;
use log::info;
use std::path::Path;

use crate::errors::ArgsValidationError;
use crate::inputs::{Format, RawShootArgs};
use crate::response_writer::ResponseWriter;
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
        format: Format,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Result<Self, ArgsValidationError> {
        Ok(Self {
            target,
            grab,
            format,
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
        let format = Format::new(raw_format, pretty, output_file.as_deref())?;
        let grab = Grab::new(raw_grab, format)?;
        Ok(Self {
            target,
            grab,
            format,
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
        let response = Client::new()?.send_request(target).await?;
        info!("Response recieved");

        let response_writer = ResponseWriter::new(response);

        match self.format {
            Format::Binary => handle_binary_output(response_writer, self.output_file()).await,
            Format::Http | Format::Json => {
                handle_string_output(
                    response_writer,
                    self.grab,
                    self.format(),
                    self.pretty,
                    self.output_file(),
                )
                .await
            }
        }?;

        info!("Finished request process.");

        Ok(())
    }
}

#[inline]
async fn handle_binary_output(
    response_writer: ResponseWriter,
    output_file: Option<&Path>,
) -> Result<(), RunError> {
    match output_file {
        Some(output_file) => Ok(response_writer.try_into_binary_file(output_file).await?),
        None => Err(RunError::from(
            "Must set output file for writing to a binary file",
        )),
    }
}

#[inline]
async fn handle_string_output(
    response_writer: ResponseWriter,
    grab: Grab,
    format: RawFormat,
    pretty: bool,
    output_file: Option<&Path>,
) -> Result<(), RunError> {
    let result = match output_file {
        Some(output_file) => {
            response_writer
                .try_into_text_file(grab, format, pretty, output_file)
                .await
        }
        None => response_writer.try_into_console(grab, format, pretty).await,
    };

    Ok(result?)
}
