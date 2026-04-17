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
    grab: RawGrab,

    #[arg(
        short,
        long,
        default_value = "http",
        help = "Format style for response data."
    )]
    format: Format,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed."
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

    pub fn grab(&self) -> RawGrab {
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
#[group(multiple = true, required = false)]
pub struct RawGrab {
    #[arg(
        long,
        short = 'S',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the status code should be grabbed from the response."
    )]
    status_code: bool,

    #[arg(
        long,
        short = 'H',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the headers should be grabbed from the response."
    )]
    headers: bool,

    #[arg(
        long,
        short = 'B',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the body should be grabbed from the response. If nothing is specified to grabbed from the response, the body will be grabbed by default."
    )]
    body: bool,

    #[arg(short = 'I', long, conflicts_with_all = ["status_code", "headers", "body", "full"], help = "Grab only the status code as an integer")]
    int_status_code: bool,

    #[arg(short = 'F', long, conflicts_with_all = ["status_code", "headers", "body", "int_status_code"], help = "Grab status code, headers, and body.")]
    full: bool,
}

impl RawGrab {
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

    pub fn full(&self) -> bool {
        self.full
    }
}

pub struct Grab {
    status_code: bool,
    headers: bool,
    body: bool,
    int_status_code: bool,
    full: bool,
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

    pub fn full(&self) -> bool {
        self.full
    }
}

impl From<RawGrab> for Grab {
    fn from(value: RawGrab) -> Self {
        if !value.status_code
            && !value.headers
            && !value.body
            && !value.int_status_code
            && !value.full
        {
            return Self {
                status_code: false,
                headers: false,
                body: true,
                int_status_code: false,
                full: false,
            };
        }
        Self {
            status_code: value.status_code,
            headers: value.headers,
            body: value.body,
            int_status_code: value.int_status_code,
            full: value.full,
        }
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
