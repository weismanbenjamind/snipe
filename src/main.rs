use snipe::{SnipeArgs, run};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run(SnipeArgs::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}