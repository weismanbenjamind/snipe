use clap::Parser;
use snipe::args::{RawSnipeCLIArgs, SnipeCLIArgs};
use snipe::run_cli;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match run_cli(SnipeCLIArgs::from(RawSnipeCLIArgs::parse())).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}
