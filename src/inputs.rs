use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Snipe", about = "Lightweight, fast, precise CLI HTTP client")]
pub struct SnipeArgs {
    #[arg(short, long, default_value = "targets.toml", help = "Path to config for target HTTP requests")]
    targets_path: PathBuf
}

impl SnipeArgs {
    pub fn targets_path(&self) -> &PathBuf {
        &self.targets_path
    }
}