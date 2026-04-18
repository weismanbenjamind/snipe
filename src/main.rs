use clap::Parser;
use snipe::inputs::{RawSnipeArgs, SnipeArgs};
use snipe::run;
use std::error::Error;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = match SnipeArgs::try_from(RawSnipeArgs::parse()) {
        Ok(args) => args,
        Err(err) => {
            return print_err_and_exit(err);
        }
    };

    match run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => print_err_and_exit(err),
    }
}

#[inline]
fn print_err_and_exit<T: Error>(err: T) -> ExitCode {
    eprintln!("{err}");
    ExitCode::FAILURE
}
