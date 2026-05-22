use crate::inputs::format::RawFormat;
use crate::inputs::grab::RawGrab;
use clap::Args;
use std::path::{Path, PathBuf};

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub struct RawShootArgs {
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
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed"
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
    pub fn new(
        target: String,
        grab: RawGrab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Self {
        Self {
            target,
            grab,
            format,
            pretty,
            output_file,
        }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> RawGrab {
        self.grab
    }

    pub fn format(&self) -> RawFormat {
        self.format
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> Option<&Path> {
        self.output_file.as_deref()
    }

    pub fn into_parts(self) -> (String, RawGrab, RawFormat, bool, Option<PathBuf>) {
        (
            self.target,
            self.grab,
            self.format,
            self.pretty,
            self.output_file,
        )
    }
}
