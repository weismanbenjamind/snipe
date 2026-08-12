use crate::client::Client;
use crate::containers::Targets;
use crate::containers::output::{GrabCfg, OutputCfg};
use crate::errors::RunError;
use crate::inputs::{RawFormat, RawGrab, ValidatedGrab};
use log::info;
use std::path::Path;

use crate::errors::ArgsValidationError::{self, GrabWithoutFormat};
use crate::inputs::{RawShootArgs, ValidatedFormat};
use crate::response::ResponseWriter;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct ShootCmd {
    target: String,
    validated_grab: Option<ValidatedGrab>,
    validated_format: Option<ValidatedFormat>,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl TryFrom<RawShootArgs> for ShootCmd {
    type Error = ArgsValidationError;
    fn try_from(value: RawShootArgs) -> Result<Self, Self::Error> {
        let (target, raw_grab, raw_format, pretty, output_file) = value.into_parts();

        let validated_format = raw_format
            .map(|r| ValidatedFormat::new_validated(r, pretty, output_file.as_deref()))
            .transpose()?;

        let validated_grab = build_validated_grab(raw_grab, validated_format)?;

        Ok(Self {
            target,
            validated_grab,
            validated_format,
            pretty,
            output_file,
        })
    }
}

fn build_validated_grab(
    raw_grab: Option<RawGrab>,
    validated_format: Option<ValidatedFormat>,
) -> Result<Option<ValidatedGrab>, ArgsValidationError> {
    match (raw_grab, validated_format) {
        (None, None) | (None, Some(_)) => Ok(None),
        (Some(g), Some(f)) => Ok(Some(ValidatedGrab::new_validated(g, f)?)),
        (Some(_), None) => return Err(GrabWithoutFormat), // TODO - this should not be an error case. Should be able to use config for format
    }
}

pub(crate) struct MergedArgs {
    validated_grab: ValidatedGrab,
    validated_format: ValidatedFormat,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl From<ShootCmd> for MergedArgs {
    fn from(value: ShootCmd) -> Self {
        Self {
            validated_grab: value.validated_grab.unwrap(), // TODO - Remove this unwrap
            validated_format: value.validated_format.unwrap(), // TODO - Remove this unwrap
            pretty: value.pretty,
            output_file: value.output_file,
        }
    }
}

impl ShootCmd {
    pub(crate) async fn run(self, targets: Targets) -> Result<(), RunError> {
        info!("Starting request process.");

        let target = targets
            .get_target(&self.target)
            .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", self.target)))?;

        info!(
            "Sending request for target '{}'.",
            target.name.as_ref().unwrap_or(&self.target)
        );

        let merged_args = self._merge_output_cfg(target.output_cfg.as_ref());

        let response = Client::new()?.send_request(target).await?;
        info!("Response recieved");

        let response_writer = ResponseWriter::new(response);

        info!("Outputting response");
        match merged_args.validated_format.raw() {
            RawFormat::Binary => {
                handle_binary_output(response_writer, merged_args.output_file.as_deref()).await
            }
            RawFormat::Http | RawFormat::Json | RawFormat::PrettyJson => {
                handle_string_output(
                    response_writer,
                    merged_args.validated_grab,
                    merged_args.validated_format,
                    merged_args.pretty,
                    merged_args.output_file.as_deref(),
                )
                .await
            }
        }?;
        info!("Successfully output response.");

        info!("Finished request process.");

        Ok(())
    }

    fn _merge_output_cfg(self, _output_cfg: Option<&OutputCfg>) -> MergedArgs {
        match _output_cfg {
            None => MergedArgs::from(self),
            Some(cfg) => {
                let pretty = merge_pretty(self.pretty, cfg.pretty);
                let output_file = merge_output_file(self.output_file, cfg.output_file.as_ref());

                // TODO - remove unwrap
                let validated_format = merge_format(
                    self.validated_format,
                    cfg.format,
                    pretty,
                    output_file.as_deref(),
                )
                .unwrap();

                // TODO - Remove unwrap
                let validated_grab =
                    merge_grab(self.validated_grab, cfg.grab.as_ref(), validated_format).unwrap();

                MergedArgs {
                    validated_grab,
                    validated_format,
                    pretty,
                    output_file,
                }
            }
        }
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

// TODO - Fix error handling below

fn merge_grab(
    from_args: Option<ValidatedGrab>,
    from_cfg: Option<&Vec<GrabCfg>>,
    validated_format: ValidatedFormat,
) -> Result<ValidatedGrab, &'static str> {
    match (from_args, from_cfg) {
        (Some(grab), None) | (Some(grab), Some(_)) => Ok(grab),
        (None, Some(grab)) => ValidatedGrab::new_validated(RawGrab::from(grab), validated_format)
            .map_err(|_| "Failed to create validated args given format"),
        (None, None) => {
            Err("Must specify what component(s) to grab out of request from either CLI or config.")
        }
    }
}

fn merge_format(
    from_args: Option<ValidatedFormat>,
    from_cfg: Option<RawFormat>,
    pretty: bool,
    output_file: Option<&Path>,
) -> Result<ValidatedFormat, &'static str> {
    match (from_args, from_cfg) {
        (Some(format), None) | (Some(format), Some(_)) => Ok(format),
        (None, Some(args)) => ValidatedFormat::new_validated(args, pretty, output_file)
            .map_err(|_| "Failed to create validated args given format"),
        (None, None) => Err("Must specify response formatting from either CLI or config."),
    }
}

// If args will default pretty to false unless passed; then it's true.
// This behavior is because --pretty is a flag
fn merge_pretty(from_args: bool, from_cfg: Option<bool>) -> bool {
    match (from_args, from_cfg) {
        (true, None) | (true, Some(_)) => true,
        (false, Some(pretty)) => pretty,
        (false, None) => false,
    }
}

fn merge_output_file(from_args: Option<PathBuf>, from_cfg: Option<&PathBuf>) -> Option<PathBuf> {
    match (from_args, from_cfg) {
        (Some(path), None) | (Some(path), Some(_)) => Some(path),
        (None, Some(path)) => Some(path.clone()),
        (None, None) => None,
    }
}
