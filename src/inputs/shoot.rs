use crate::inputs::format::RawFormat;
use crate::inputs::grab::RawGrab;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub(crate) struct ShootArgs {
    #[arg(help = "Target HTTP request to send")]
    pub(crate) target: String,

    #[command(flatten)]
    pub(crate) grab: Option<RawGrab>,

    #[arg(short, long, help = "Format style for response data")]
    pub(crate) format: Option<RawFormat>,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed. No-op if `--format pretty-json` (`-f pretty-json`) is passed"
    )]
    pub(crate) pretty: bool,

    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout"
    )]
    pub(crate) output_file: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If request should be built but not sent"
    )]
    pub(crate) dry_run: bool,
}
