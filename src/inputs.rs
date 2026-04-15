use clap::{Parser, ValueEnum};
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

    #[arg(short, long, default_value = "body", help = "Response data to return.")]
    grab: Grab,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be attempted to be parsed into a json string."
    )]
    json: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        requires = "json",
        help = "If the output should be pretty printed. Only valid is the --json (-j) option is passed."
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

    pub fn json(&self) -> bool {
        self.json
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Grab {
    Full,
    Status,
    Headers,
    Body,
    StatusCodeAndHeaders,
    StatusCodeAndBody,
    HeadersAndBody,
    StatusCode,
}

#[derive(Clone, Copy, Debug)]
pub struct GrabSettings {
    status: bool,
    headers: bool,
    body: bool,
    status_code: bool,
}

impl GrabSettings {
    pub fn new(status: bool, headers: bool, body: bool, status_code: bool) -> Self {
        Self {
            status,
            headers,
            body,
            status_code,
        }
    }

    pub fn status(&self) -> bool {
        self.status
    }

    pub fn headers(&self) -> bool {
        self.headers
    }

    pub fn body(&self) -> bool {
        self.body
    }

    pub fn status_code(&self) -> bool {
        self.status_code
    }
}

impl From<Grab> for GrabSettings {
    fn from(value: Grab) -> Self {
        match value {
            Grab::Full => Self::new(true, true, true, true),
            Grab::Status => Self::new(true, false, false, false),
            Grab::Headers => Self::new(false, true, false, false),
            Grab::Body => Self::new(false, false, true, false),
            Grab::StatusCodeAndHeaders => Self::new(true, true, false, false),
            Grab::StatusCodeAndBody => Self::new(true, false, true, false),
            Grab::HeadersAndBody => Self::new(false, true, true, false),
            Grab::StatusCode => Self::new(false, false, false, true),
        }
    }
}
