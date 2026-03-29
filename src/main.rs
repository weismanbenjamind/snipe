use clap::Parser;
use snipe::{SnipeArgs, run};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match run(SnipeArgs::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
