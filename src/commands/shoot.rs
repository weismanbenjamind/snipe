use crate::client::Client;
use crate::containers::Targets;
use crate::containers::output::{GrabCfg, OutputCfg};
use crate::errors::{ArgsValidationError, RunError};
use crate::inputs::{RawFormat, RawGrab, ShootArgs, ValidatedFormat, ValidatedGrab};
use crate::response::ResponseWriter;
use log::info;
use std::path::Path;
use std::path::PathBuf;

pub(crate) struct MergedArgs {
    validated_grab: ValidatedGrab,
    validated_format: ValidatedFormat,
    pretty: bool,
    output_file: Option<PathBuf>,
}

pub(crate) async fn run_shoot_cmd(shoot_args: ShootArgs, targets: Targets) -> Result<(), RunError> {
    info!("Starting request process.");

    let target = targets
        .get_target(&shoot_args.target)
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", shoot_args.target)))?;

    info!(
        "Sending request for target '{}'.",
        target.name.as_ref().unwrap_or(&shoot_args.target)
    );

    let merged_args = build_merged_args(shoot_args, target.output_cfg.as_ref())?;

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

fn build_merged_args(
    shoot_args: ShootArgs,
    output_cfg: Option<&OutputCfg>,
) -> Result<MergedArgs, ArgsValidationError> {
    match output_cfg {
        None => build_merged_args_output_cfg_none(shoot_args),
        Some(cfg) => build_merged_args_output_cfg_some(shoot_args, cfg),
    }
}

fn build_merged_args_output_cfg_none(
    shoot_args: ShootArgs,
) -> Result<MergedArgs, ArgsValidationError> {
    let raw_grab = shoot_args.grab.ok_or(ArgsValidationError::UnderspecifiedMerge("Must specify components to grab from request via CLI if omitting them from target config."))?;
    let raw_format = shoot_args
        .format
        .ok_or(ArgsValidationError::UnderspecifiedMerge(
            "Must specify output format via CLI if omitting it from target config.",
        ))?;

    // TODO - Tweak encap so can only build validated format via `new`
    let validated_format = ValidatedFormat::new_validated(
        raw_format,
        shoot_args.pretty,
        shoot_args.output_file.as_deref(),
    )?;

    // TODO - Tweak encap so can only build validated grab via `new`
    let validated_grab = ValidatedGrab::new_validated(raw_grab, validated_format)?;

    Ok(MergedArgs {
        validated_grab,
        validated_format,
        pretty: shoot_args.pretty,
        output_file: shoot_args.output_file,
    })
}

fn build_merged_args_output_cfg_some(
    shoot_args: ShootArgs,
    cfg: &OutputCfg,
) -> Result<MergedArgs, ArgsValidationError> {
    let pretty = merge_pretty(shoot_args.pretty, cfg.pretty);
    let output_file = merge_output_file(shoot_args.output_file, cfg.output_file.as_ref());

    let validated_format = merge_format(
        shoot_args.format,
        cfg.format,
        pretty,
        output_file.as_deref(),
    )?;

    let validated_grab = merge_grab(shoot_args.grab, cfg.grab.as_ref(), validated_format)?;

    Ok(MergedArgs {
        validated_grab,
        validated_format,
        pretty,
        output_file,
    })
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

fn merge_format(
    from_args: Option<RawFormat>,
    from_cfg: Option<RawFormat>,
    pretty: bool,
    output_file: Option<&Path>,
) -> Result<ValidatedFormat, ArgsValidationError> {
    match (from_args, from_cfg) {
        (Some(format), None) | (Some(format), Some(_)) => {
            ValidatedFormat::new_validated(format, pretty, output_file)
        }
        (None, Some(format)) => ValidatedFormat::new_validated(format, pretty, output_file),
        (None, None) => Ok(ValidatedFormat::new(RawFormat::Http)), // If nothing passed via CLI and via cfg default to HTTP
    }
}

fn merge_grab(
    from_args: Option<RawGrab>,
    from_cfg: Option<&Vec<GrabCfg>>,
    validated_format: ValidatedFormat,
) -> Result<ValidatedGrab, ArgsValidationError> {
    match (from_args, from_cfg) {
        (Some(grab), None) | (Some(grab), Some(_)) => {
            ValidatedGrab::new_validated(grab, validated_format)
        }
        (None, Some(grab)) => ValidatedGrab::new_validated(RawGrab::from(grab), validated_format),
        (None, None) => Ok(ValidatedGrab {
            status_code: false,
            headers: false,
            body: true, // If nothing passed via CLI and via cfg default to body
            int_status_code: false,
        }),
    }
}
