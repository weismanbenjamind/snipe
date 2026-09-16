use std::process::ExitCode;

use clap::Parser;
use snipe::{RawSnipeCLIArgs, run_cli};

#[tokio::main]
async fn main() -> ExitCode {
    match run_cli(RawSnipeCLIArgs::parse()).await {
        Ok(success_msg) => {
            println!("{success_msg}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
