use clap::Parser;
use snipe::{RawSnipeArgs, SnipeArgs, run};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = match SnipeArgs::try_from(RawSnipeArgs::parse()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
