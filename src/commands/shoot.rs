use crate::client::Client;
use crate::errors::RunError;
use crate::inputs::{RawFormat, ValidatedGrab};
use crate::targets::Targets;
use log::info;
use std::path::Path;

use crate::errors::ArgsValidationError;
use crate::inputs::{RawShootArgs, ValidatedFormat};
use crate::response::ResponseWriter;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ShootCmd {
    target: String,
    validated_grab: ValidatedGrab,
    validated_format: ValidatedFormat,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl ShootCmd {
    pub fn new(
        target: String,
        validated_grab: ValidatedGrab,
        validated_format: ValidatedFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Result<Self, ArgsValidationError> {
        Ok(Self {
            target,
            validated_grab,
            validated_format,
            pretty,
            output_file,
        })
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn validated_grab(&self) -> ValidatedGrab {
        self.validated_grab
    }

    pub fn validated_format(&self) -> ValidatedFormat {
        self.validated_format
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
        let validated_format =
            ValidatedFormat::new_validated(raw_format, pretty, output_file.as_deref())?;
        let validated_grab = ValidatedGrab::new_validated(raw_grab, validated_format)?;
        Ok(Self {
            target,
            validated_grab,
            validated_format,
            pretty,
            output_file,
        })
    }
}

impl ShootCmd {
    pub async fn run(&self, targets: &Targets) -> Result<(), RunError> {
        info!("Starting request process.");

        let some_nonsense: &str = "1";

        let target = targets
            .get_target(self.target())
            .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", self.target)))?;

        info!("Sending request for target '{}'.", target.name());
        let response = Client::new()?.send_request(target).await?;
        info!("Response recieved");

        let response_writer = ResponseWriter::new(response);

        info!("Outputting response");
        match self.validated_format().raw_format() {
            RawFormat::Binary => handle_binary_output(response_writer, self.output_file()).await,
            RawFormat::Http | RawFormat::Json => {
                handle_string_output(
                    response_writer,
                    self.validated_grab,
                    self.validated_format,
                    self.pretty,
                    self.output_file(),
                )
                .await
            }
        }?;
        info!("Successfully output response.");

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
    validated_grab: ValidatedGrab,
    validated_format: ValidatedFormat,
    pretty: bool,
    output_file: Option<&Path>,
) -> Result<(), RunError> {
    let result = match output_file {
        Some(output_file) => {
            response_writer
                .try_into_text_file(validated_grab, validated_format, pretty, output_file)
                .await
        }
        None => {
            response_writer
                .try_into_console(validated_grab, validated_format, pretty)
                .await
        }
    };

    Ok(result?)
}
