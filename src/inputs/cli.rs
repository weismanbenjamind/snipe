use clap::{ArgAction, Parser, Subcommand};
use thiserror::Error;

use super::init::InitArgsError;
use crate::inputs::{InitArgs, ListArgs, RawInitArgs, RawListArgs, RawShootArgs, ShootArgs};

#[derive(Debug, Clone, Error)]
pub enum CLIError {
    #[error("{0}")]
    InitArgsConversion(#[from] InitArgsError),
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Snipe",
    about = "Lightweight, fast, precise CLI HTTP client",
    version
)]
pub struct RawSnipeCLIArgs {
    #[command(subcommand)]
    command: RawCommand,

    #[arg(short, long, action = ArgAction::Count, help = "Verbosity. Use -v for info. Use -vv for debug. Anything including and beyond -vv is set to debug. Defaults to warn.")]
    verbose: u8,
}

pub(crate) struct SnipeCLIArgs {
    pub(crate) command: Command,
    pub(crate) verbose: u8,
}

impl TryFrom<RawSnipeCLIArgs> for SnipeCLIArgs {
    type Error = CLIError;
    fn try_from(value: RawSnipeCLIArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            command: value.command.try_into()?,
            verbose: value.verbose,
        })
    }
}

#[derive(Clone, Debug, Subcommand)]
enum RawCommand {
    List(RawListArgs),
    Shoot(RawShootArgs),
    Init(RawInitArgs),
}

pub(crate) enum Command {
    List(ListArgs),
    Shoot(ShootArgs),
    Init(InitArgs),
}

impl TryFrom<RawCommand> for Command {
    type Error = CLIError;
    fn try_from(value: RawCommand) -> Result<Self, Self::Error> {
        let cmd = match value {
            RawCommand::List(args) => Self::List(args.into()),
            RawCommand::Shoot(args) => Self::Shoot(args.into()),
            RawCommand::Init(args) => Self::Init(args.try_into()?),
        };

        Ok(cmd)
    }
}
