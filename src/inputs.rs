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

    #[arg(
        short,
        long,
        default_value = "headers",
        help = "Response data to return."
    )]
    grab: Grab,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed."
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

    pub fn pretty(&self) -> bool {
        self.pretty
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Grab {
    Full,
    StatusCode,
    Headers,
    Body,
    StatusCodeAndHeaders,
    StatusCodeAndBody,
    HeadersAndBody,
}
