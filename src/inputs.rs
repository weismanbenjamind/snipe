use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
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
}

impl SnipeArgs {
    pub fn cfg_path(&self) -> &PathBuf {
        &self.cfg
    }

    pub fn target(&self) -> &str {
        &self.target
    }
}
