use crate::errors::ArgsValidationError;
use clap::{Args, CommandFactory, Parser, ValueEnum};
use colored::Colorize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "Snipe", about = "Lightweight, fast, precise CLI HTTP client.")]
pub struct SnipeArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config for target HTTP requests."
    )]
    cfg: PathBuf,

    #[arg(short, long, help = "Target HTTP request to send.")]
    target: String,

    #[command(flatten)]
    grab: Grab,

    #[arg(
        short,
        long,
        default_value = "http",
        help = "Format style for response data."
    )]
    format: Format,

    // TODO - Figure out if there is a way to require Format::JSON to be present if this argument is passed
    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed otherise this args has no effect."
    )]
    pretty: bool,

    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout."
    )]
    output_file: Option<PathBuf>,
}

impl SnipeArgs {
    pub fn cfg_path(&self) -> &PathBuf {
        &self.cfg
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> Grab {
        self.grab
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }

    pub fn validate(&self) -> Result<(), ArgsValidationError> {
        match self.format {
            Format::HTTP => match self.pretty {
                true => get_no_pretty_with_http_format_err(),
                false => Ok(()),
            },
            Format::JSON => Ok(()),
        }
    }
}

#[derive(Args, Clone, Copy, Debug)]
#[group(multiple = true, required = true)]
pub struct Grab {
    #[arg(
        long,
        short,
        conflicts_with = "int_status_code",
        help = "If the status code should be grabbed from the response."
    )]
    status_code: bool,

    #[arg(
        long,
        short = 'H',
        conflicts_with = "int_status_code",
        help = "If the headers should be grabbed from the response."
    )]
    headers: bool,

    #[arg(
        long,
        short,
        conflicts_with = "int_status_code",
        help = "If the body should be grabbed from the response."
    )]
    body: bool,

    #[arg(short, long, conflicts_with_all = ["status_code", "headers", "body"], help = "Grab only the status code as an integer")]
    int_status_code: bool,
}

impl Grab {
    pub fn status_code(&self) -> bool {
        self.status_code
    }

    pub fn headers(&self) -> bool {
        self.headers
    }

    pub fn body(&self) -> bool {
        self.body
    }

    pub fn int_status_code(&self) -> bool {
        self.int_status_code
    }
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
pub enum Format {
    HTTP,
    JSON,
}

#[inline]
fn get_no_pretty_with_http_format_err() -> Result<(), ArgsValidationError> {
    Err(ArgsValidationError::Base(format!(
        "{} the argument {} cannot be used with {}\n\n{}\n\nFor more information try '{}'",
        "error:".red().bold(),
        "'--pretty'".yellow(),
        "'--format http'".yellow(),
        SnipeArgs::command().render_usage(),
        "--help".bold()
    )))
}
