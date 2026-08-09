use crate::inputs::format::RawFormat;
use crate::inputs::grab::RawGrab;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub(crate) struct RawShootArgs {
    #[arg(help = "Target HTTP request to send")]
    target: String,

    #[command(flatten)]
    grab: RawGrab,

    #[arg(
        short,
        long,
        default_value = "http",
        help = "Format style for response data"
    )]
    format: RawFormat,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed. No-op if `--format pretty-json` (`-f pretty-json`) is passed"
    )]
    pretty: bool,

    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout"
    )]
    output_file: Option<PathBuf>,
}

impl RawShootArgs {
    pub(crate) fn into_parts(self) -> (String, RawGrab, RawFormat, bool, Option<PathBuf>) {
        (
            self.target,
            self.grab,
            self.format,
            self.pretty,
            self.output_file,
        )
    }
}
