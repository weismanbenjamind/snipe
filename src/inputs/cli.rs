use crate::commands::ShootCmd;
use crate::errors::ArgsValidationError;
use crate::inputs::shoot::RawShootArgs;
use clap::{ArgAction, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Snipe",
    about = "Lightweight, fast, precise CLI HTTP client",
    version
)]
pub struct RawSnipeCLIArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config for target HTTP requests"
    )]
    cfg: PathBuf,

    #[command(subcommand)]
    command: RawCommand,

    #[arg(
        short = 'e',
        long,
        default_value = "SNIPE_TARGETS",
        help = "Environment variable whose value will be used to look for cfg the file if the path pointed to by the --cfg (-c) argument does not exist. Pass 'skip' to disable searching for this env var"
    )]
    cfg_env: String,

    #[arg(short, long, action = ArgAction::Count, help = "Verbosity. Use -v for info. Use -vv for debug. Anything including and beyond -vv is set to debug. Defaults to warn.")]
    verbose: u8,
}

impl RawSnipeCLIArgs {
    pub fn cfg(&self) -> &Path {
        &self.cfg
    }

    pub fn command(&self) -> &RawCommand {
        &self.command
    }

    pub fn cfg_env(&self) -> &str {
        &self.cfg_env
    }

    pub fn verbose(&self) -> u8 {
        self.verbose
    }
}

pub struct SnipeCLIArgs {
    cfg: PathBuf,
    command: Command,
    cfg_env: Option<String>,
    verbose: u8,
}

impl SnipeCLIArgs {
    pub fn cfg(&self) -> &Path {
        &self.cfg
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn cfg_env(&self) -> Option<&str> {
        self.cfg_env.as_deref()
    }

    pub fn verbose(&self) -> u8 {
        self.verbose
    }

    fn resolve_cfg_env(cfg_env: String) -> Option<String> {
        match cfg_env.to_lowercase().as_str() {
            "skip" => None,
            _ => Some(cfg_env.to_string()),
        }
    }
}

impl TryFrom<RawSnipeCLIArgs> for SnipeCLIArgs {
    type Error = ArgsValidationError;
    fn try_from(value: RawSnipeCLIArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            cfg: value.cfg,
            command: Command::try_from(value.command)?,
            cfg_env: Self::resolve_cfg_env(value.cfg_env),
            verbose: value.verbose,
        })
    }
}

// Note - comments below are actually used of for CLI documentation
#[derive(Clone, Debug, Subcommand)]
pub enum RawCommand {
    /// List all potential API requests to make
    ListTargets,
    Shoot(RawShootArgs),
}

#[derive(Clone, Debug)]
pub enum Command {
    ListTargets,
    Shoot(ShootCmd),
}

impl TryFrom<RawCommand> for Command {
    type Error = ArgsValidationError;
    fn try_from(value: RawCommand) -> Result<Self, Self::Error> {
        match value {
            RawCommand::ListTargets => Ok(Self::ListTargets),
            RawCommand::Shoot(raw_shoot_args) => {
                Ok(Self::Shoot(ShootCmd::try_from(raw_shoot_args)?))
            }
        }
    }
}
