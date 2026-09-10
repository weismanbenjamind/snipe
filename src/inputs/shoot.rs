use std::path::PathBuf;

use clap::Args;

use super::cfg_env::resolve_cfg_env;
use crate::inputs::{
    format::RawFormat, grab::RawGrab, output_file::OutputFileArgs, pretty::PrettyArgs,
};

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub(crate) struct RawShootArgs {
    #[arg(help = "Target HTTP request to send")]
    pub(crate) target: String,

    #[command(flatten)]
    pub(crate) grab: Option<RawGrab>,

    #[arg(short, long, help = "Format style for response data")]
    pub(crate) format: Option<RawFormat>,

    #[command(flatten)]
    pub(crate) pretty_args: PrettyArgs,

    #[command(flatten)]
    pub(crate) output_file_args: OutputFileArgs,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If request should be built but not sent"
    )]
    pub(crate) dry_run: bool,

    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config for target HTTP requests"
    )]
    pub(crate) cfg: PathBuf,

    #[arg(
        short = 'e',
        long,
        default_value = "SNIPE_TARGETS",
        help = "Environment variable whose value will be used to look for cfg the file if the path pointed to by the --cfg (-c) argument does not exist. Pass 'skip' to disable searching for this env var"
    )]
    pub(crate) cfg_env: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ShootArgs {
    pub(crate) target: String,
    pub(crate) grab: Option<RawGrab>,
    pub(crate) format: Option<RawFormat>,
    pub(crate) pretty_args: PrettyArgs,
    pub(crate) output_file_args: OutputFileArgs,
    pub(crate) dry_run: bool,
    pub(crate) cfg: PathBuf,
    pub(crate) cfg_env: Option<String>,
}

impl From<RawShootArgs> for ShootArgs {
    fn from(value: RawShootArgs) -> Self {
        let cfg_env = resolve_cfg_env(&value.cfg_env);

        Self {
            target: value.target,
            grab: value.grab,
            format: value.format,
            pretty_args: value.pretty_args,
            output_file_args: value.output_file_args,
            dry_run: value.dry_run,
            cfg: value.cfg,
            cfg_env,
        }
    }
}
