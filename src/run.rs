use log::info;

use crate::cfg_resolver::CfgResolver;
use crate::commands::run_list_targets_cmd;
use crate::errors::RunError;
use crate::inputs::{Command, SnipeCLIArgs};
use crate::targets::Targets;

pub async fn run_cli(snipe_cli_args: SnipeCLIArgs) -> Result<(), RunError> {
    env_logger::init();

    info!("Resolving config file path.");
    let cfg_path = CfgResolver::new(snipe_cli_args.cfg(), snipe_cli_args.cfg_env())
        .resolve_cfg_path_from_env()?;
    info!("Resolved config file path to {}.", cfg_path.display());

    info!("Creating targets from file at {}.", cfg_path.display());
    let targets = Targets::from_toml_file(&cfg_path)?;
    info!("Targets successfully created.");

    match snipe_cli_args.command() {
        Command::ListTargets => run_list_targets_cmd(&targets),
        Command::Shoot(cmd) => cmd.run(&targets).await,
    }
}
