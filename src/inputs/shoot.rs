use crate::errors::ArgsValidationError;
use crate::inputs::format::{Format, RawFormat};
use crate::inputs::grab::{Grab, RawGrab};
use clap::Args;
use std::path::{Path, PathBuf};

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub struct RawShootArgs {
    #[arg(short, long, help = "Target HTTP request to send")]
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

#[derive(Clone, Debug)]
pub struct ShootArgs {
    target: String,
    grab: Grab,
    format: Format,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl ShootArgs {
    // Since take RawGrab and RawFormat here
    // ::new() enforces validated grab and format args in the constructor
    // We know outside this module ShootArgs will always been in a valid state
    pub fn new(
        target: String,
        grab: RawGrab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Result<Self, ArgsValidationError> {
        // TODO - de-dupe this code if it appears everywhere.
        // Might also be worth it to let Grab know about format and pretty - that might just be ShootArgs at this points
        let format = Format::new(format, pretty)?;
        let grab = Grab::new(grab, format)?;

        Ok(Self {
            target,
            grab,
            format,
            pretty,
            output_file,
        })
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> Grab {
        self.grab
    }

    pub fn format(&self) -> RawFormat {
        self.format.into()
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }
}

impl TryFrom<RawShootArgs> for ShootArgs {
    type Error = ArgsValidationError;
    fn try_from(value: RawShootArgs) -> Result<Self, Self::Error> {
        Self::new(
            value.target,
            value.grab,
            value.format,
            value.pretty,
            value.output_file,
        )
    }
}
