use crate::cfg_resolver::CfgResolver;
use crate::commands::{SnipeResult, run_list_targets_cmd, run_shoot_cmd};
use crate::containers::Targets;
use crate::errors::RunError;
use crate::inputs::{Command, SnipeCLIArgs};
use std::env;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt as tracing_subscriber_fmt;

const RUST_LOG: &str = "RUST_LOG";
const WARN: &str = "warn";
const INFO: &str = "info";
const DEBUG: &str = "debug";

pub async fn run_cli(snipe_cli_args: SnipeCLIArgs) -> SnipeResult {
    set_vebosity(snipe_cli_args.verbose)?;

    let cfg_path = CfgResolver::new(&snipe_cli_args.cfg, snipe_cli_args.cfg_env.as_deref())
        .resolve_cfg_path_from_env()?;

    let targets = Targets::from_toml_file(&cfg_path)?;

    match snipe_cli_args.command {
        Command::List => run_list_targets_cmd(targets),
        Command::Shoot(shoot_args) => run_shoot_cmd(shoot_args, targets).await,
    }
}

fn set_vebosity(verbosity: u8) -> Result<(), RunError> {
    let log_level = get_log_level(verbosity);

    if let Some(log_level) = log_level {
        init_tracing_subscriber(&log_level)?;
    }

    Ok(())
}

#[inline]
fn get_log_level(verbosity: u8) -> Option<String> {
    match env::var(RUST_LOG) {
        Ok(level) => Some(level),
        Err(_) => Some(match verbosity {
            0 => WARN.to_string(),
            1 => build_verbosity_string(INFO),
            _ => build_verbosity_string(DEBUG),
        }),
    }
}

#[inline]
fn build_verbosity_string(level: &str) -> String {
    format!("warn,snipe={level}")
}

#[inline]
fn init_tracing_subscriber(log_level: &str) -> Result<(), RunError> {
    tracing_subscriber_fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .try_init()
        .map_err(|e| RunError::Failure(format!("Failed to set verbosity. Error: {e}.")))
}
