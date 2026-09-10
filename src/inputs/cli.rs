use clap::{ArgAction, Parser, Subcommand};

use crate::inputs::{InitArgs, ListArgs, RawListArgs, RawShootArgs, ShootArgs};

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

pub struct SnipeCLIArgs {
    pub(crate) command: Command,
    pub(crate) verbose: u8,
}

impl From<RawSnipeCLIArgs> for SnipeCLIArgs {
    fn from(value: RawSnipeCLIArgs) -> Self {
        Self {
            command: value.command.into(),
            verbose: value.verbose,
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
enum RawCommand {
    List(RawListArgs),
    Shoot(RawShootArgs),
    Init(InitArgs),
}

pub(crate) enum Command {
    List(ListArgs),
    Shoot(ShootArgs),
    Init(InitArgs),
}

impl From<RawCommand> for Command {
    fn from(value: RawCommand) -> Self {
        match value {
            RawCommand::List(args) => Self::List(args.into()),
            RawCommand::Shoot(args) => Self::Shoot(args.into()),
            RawCommand::Init(args) => Self::Init(args),
        }
    }
}
