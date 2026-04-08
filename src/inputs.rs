use std::path::PathBuf;

use clap::{Parser, ValueEnum};

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
