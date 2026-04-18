use crate::errors::ArgsValidationError;
use clap::{Args, Parser, ValueEnum};
use colored::Colorize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "Snipe", about = "Lightweight, fast, precise CLI HTTP client.")]
pub struct RawSnipeArgs {
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
    format: RawFormat,

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

impl RawSnipeArgs {
    pub fn new(
        cfg: PathBuf,
        target: String,
        grab: RawGrab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Self {
        Self {
            cfg,
            target,
            grab,
            format,
            pretty,
            output_file,
        }
    }

    pub fn cfg_path(&self) -> &PathBuf {
        &self.cfg
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

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
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
    pub fn new(
        status_code: bool,
        headers: bool,
        body: bool,
        int_status_code: bool,
        full: bool,
    ) -> Result<Self, ArgsValidationError> {
        let have_individuals = have_inividuals(status_code, headers, body);
        check_have_individuals_vs_int_status_code(have_individuals, int_status_code)?;

        if have_individuals && full {
            return ArgsValidationError::new_err(
                "Cannot pass status_code, headers, or body, with full.",
            );
        }

        if int_status_code && full {
            return ArgsValidationError::new_err("Cannot pass full with int_status_code.");
        }

        Ok(Self {
            status_code,
            headers,
            body,
            int_status_code,
            full,
        })
    }

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

    pub fn in_default_state(&self) -> bool {
        !self.status_code && !self.headers && !self.body && !self.int_status_code && !self.full
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Grab {
    status_code: bool,
    headers: bool,
    body: bool,
    int_status_code: bool,
}

impl Grab {
    pub fn new(
        status_code: bool,
        headers: bool,
        body: bool,
        int_status_code: bool,
    ) -> Result<Self, ArgsValidationError> {
        let have_individuals = have_inividuals(status_code, headers, body);
        check_have_individuals_vs_int_status_code(have_individuals, int_status_code)?;

        Ok(Self {
            status_code,
            headers,
            body,
            int_status_code,
        })
    }

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

#[inline]
fn have_inividuals(status_code: bool, headers: bool, body: bool) -> bool {
    status_code || headers || body
}

#[inline]
fn check_have_individuals_vs_int_status_code(
    have_individuals: bool,
    int_status_code: bool,
) -> Result<(), ArgsValidationError> {
    if have_individuals && int_status_code {
        return ArgsValidationError::new_err(
            "Cannot pass status_code, headers, or body, with int_status_code.",
        );
    }
    Ok(())
}

// Validation should occur at RawArgs level
impl From<RawGrab> for Grab {
    fn from(value: RawGrab) -> Self {
        // If everything is false default to headers
        if value.in_default_state() {
            return Self {
                status_code: false,
                headers: false,
                body: true,
                int_status_code: false,
            };
        }

        // If full is true then set status_code, headers, and body to true
        // and int_status_code to false
        if value.full {
            return Self {
                status_code: true,
                headers: true,
                body: true,
                int_status_code: false,
            };
        }

        // Otherwise return all values as set in RawGrab
        Self {
            status_code: value.status_code,
            headers: value.headers,
            body: value.body,
            int_status_code: value.int_status_code,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
pub enum RawFormat {
    Http,
    Json,
}

impl From<Format> for RawFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::Http => Self::Http,
            Format::Json => Self::Json,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Format {
    Http,
    Json,
}

impl Format {
    fn new(raw_format: RawFormat, pretty: bool) -> Result<Self, ArgsValidationError> {
        match raw_format {
            RawFormat::Http => match pretty {
                true => get_no_pretty_with_http_format_err(),
                false => Ok(Self::Http),
            },
            RawFormat::Json => Ok(Self::Json),
        }
    }
}

// TODO - need a CLI and non-CLI variant for this error message
#[inline]
fn get_no_pretty_with_http_format_err<T>() -> Result<T, ArgsValidationError> {
    Err(ArgsValidationError::Base(format!(
        "{} the argument {} cannot be used with {}\n\n{} {} {} <TARGET> {} {}\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--pretty'".yellow(),
        "'--format http'".yellow(),
        "Usage:".bold().underline(),
        "snipe".bold(),
        "--target".bold(),
        "--format".bold(),
        "http".bold(),
        "--help".bold()
    )))
}

pub struct SnipeArgs {
    cfg: PathBuf,
    target: String,
    grab: Grab,
    format: Format,
    pretty: bool,
    output_file: Option<PathBuf>,
}

impl SnipeArgs {
    pub fn new(
        cfg: PathBuf,
        target: String,
        grab: Grab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
    ) -> Result<Self, ArgsValidationError> {
        Ok(Self {
            cfg,
            target,
            grab,
            format: Format::new(format, pretty)?,
            pretty,
            output_file,
        })
    }

    pub fn cfg_path(&self) -> &PathBuf {
        &self.cfg
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> Grab {
        self.grab
    }

    pub(crate) fn format(&self) -> Format {
        self.format
    }

    pub fn raw_format(&self) -> RawFormat {
        self.format.into()
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }
}

impl TryFrom<RawSnipeArgs> for SnipeArgs {
    type Error = ArgsValidationError;
    fn try_from(value: RawSnipeArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            cfg: value.cfg,
            target: value.target,
            grab: Grab::from(value.grab),
            format: Format::new(value.format, value.pretty)?,
            pretty: value.pretty,
            output_file: value.output_file,
        })
    }
}
