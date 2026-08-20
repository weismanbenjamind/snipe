use crate::client::Client;
use crate::commands::{SnipeResult, SuccessMsg};
use crate::containers::Targets;
use crate::containers::output::{GrabCfg, OutputCfg};
use crate::errors::{ArgsValidationError, RunError};
use crate::inputs::{RawFormat, RawGrab, ShootArgs, ValidatedFormat, ValidatedGrab};
use crate::response::ResponseWriter;
use log::{debug, info};
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct MergedArgs {
    validated_grab: ValidatedGrab,
    validated_format: ValidatedFormat,
    pretty: bool,
    output_file: Option<PathBuf>,
    dry_run: bool,
}

pub(crate) async fn run_shoot_cmd(shoot_args: ShootArgs, targets: Targets) -> SnipeResult {
    info!("Running shoot command.");

    let target = targets
        .get_target(&shoot_args.target)
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", shoot_args.target)))?;

    info!(
        "Initiating request build/send process for target '{}'.",
        target.name.as_ref().unwrap_or(&shoot_args.target)
    );

    let merged_args = build_merged_args(shoot_args, target.output_cfg.as_ref())?;

    let client = Client::new()?;
    let request = client.build_request(target)?;

    if merged_args.dry_run {
        return Ok(SuccessMsg(
            "Dry run detected. Request succesfully built. Not sending.".to_string(),
        ));
    }

    let response = client.send_request(request).await?;

    let response_writer = ResponseWriter::new(response);

    let result = match merged_args.validated_format.raw() {
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
    };
    info!("Successfully ran shoot command.");

    result
}

fn build_merged_args(
    shoot_args: ShootArgs,
    output_cfg: Option<&OutputCfg>,
) -> Result<MergedArgs, ArgsValidationError> {
    let args = match output_cfg {
        None => build_merged_args_output_cfg_none(shoot_args),
        Some(cfg) => build_merged_args_output_cfg_some(shoot_args, cfg),
    }?;

    debug!("Using merged args:\n{args:#?}");

    Ok(args)
}

fn build_merged_args_output_cfg_none(
    shoot_args: ShootArgs,
) -> Result<MergedArgs, ArgsValidationError> {
    let raw_grab = shoot_args.grab.unwrap_or_default(); // RawGrab defauts to grabbing body. Which is what we want if grab isn't present
    let raw_format = shoot_args.format.unwrap_or_default(); // RawFormat defauts to HTTP. Which is what we want if format isn't present

    let validated_format = ValidatedFormat::new(
        raw_format,
        shoot_args.pretty,
        shoot_args.output_file.as_deref(),
    )?;

    let validated_grab = ValidatedGrab::new(raw_grab, validated_format)?;

    Ok(MergedArgs {
        validated_grab,
        validated_format,
        pretty: shoot_args.pretty,
        output_file: shoot_args.output_file,
        dry_run: shoot_args.dry_run,
    })
}

fn build_merged_args_output_cfg_some(
    shoot_args: ShootArgs,
    cfg: &OutputCfg,
) -> Result<MergedArgs, ArgsValidationError> {
    let pretty = merge_cli_flag(shoot_args.pretty, cfg.pretty);
    let output_file = merge_output_file(shoot_args.output_file, cfg.output_file.as_deref());
    let validated_format = merge_format(
        shoot_args.format,
        cfg.format,
        pretty,
        output_file.as_deref(),
    )?;
    let validated_grab = merge_grab(shoot_args.grab, cfg.grab.as_ref(), validated_format)?;
    let dry_run = merge_cli_flag(shoot_args.dry_run, cfg.dry_run);

    Ok(MergedArgs {
        validated_grab,
        validated_format,
        pretty,
        output_file,
        dry_run,
    })
}

// Args will default to false unless passed; then it's true.
// This behavior is because --arg is a flag
fn merge_cli_flag(from_args: bool, from_cfg: Option<bool>) -> bool {
    match (from_args, from_cfg) {
        (true, None) | (true, Some(_)) => true,
        (false, Some(cfg_val)) => cfg_val,
        (false, None) => false,
    }
}

fn merge_output_file(from_args: Option<PathBuf>, from_cfg: Option<&Path>) -> Option<PathBuf> {
    match (from_args, from_cfg) {
        (Some(path), None) | (Some(path), Some(_)) => Some(path),
        (None, Some(path)) => Some(PathBuf::from(path)),
        (None, None) => None,
    }
}

fn merge_format(
    from_args: Option<RawFormat>,
    from_cfg: Option<RawFormat>,
    pretty: bool,
    output_file: Option<&Path>,
) -> Result<ValidatedFormat, ArgsValidationError> {
    match (from_args, from_cfg) {
        (Some(format), None) | (Some(format), Some(_)) => {
            ValidatedFormat::new(format, pretty, output_file)
        }
        (None, Some(format)) => ValidatedFormat::new(format, pretty, output_file),
        (None, None) => ValidatedFormat::new(RawFormat::default(), pretty, output_file), // If nothing passed via CLI and via cfg use default which is HTTP
    }
}

fn merge_grab(
    from_args: Option<RawGrab>,
    from_cfg: Option<&Vec<GrabCfg>>,
    validated_format: ValidatedFormat,
) -> Result<ValidatedGrab, ArgsValidationError> {
    match (from_args, from_cfg) {
        (Some(grab), None) | (Some(grab), Some(_)) => ValidatedGrab::new(grab, validated_format),
        (None, Some(grab)) => ValidatedGrab::new(RawGrab::try_from(grab)?, validated_format),
        (None, None) => ValidatedGrab::new(RawGrab::default(), validated_format), // If nothing passed via CLI and via cfg use default which is Body
    }
}

#[inline]
async fn handle_binary_output(
    response_writer: ResponseWriter,
    output_file: Option<&Path>,
) -> SnipeResult {
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
) -> SnipeResult {
    let result = match output_file {
        Some(output_file) => {
            response_writer
                .try_into_text_file(validated_grab, validated_format, pretty, output_file)
                .await
        }
        None => {
            response_writer
                .try_into_string(validated_grab, validated_format, pretty)
                .await
        }
    };

    result.map_err(RunError::from)
}
