use clap::Parser;
use colored::Colorize;
use snipe::errors::ArgsValidationError;
use snipe::inputs::{RawSnipeCLIArgs, SnipeCLIArgs};
use snipe::run_cli;
use std::error::Error;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = match SnipeCLIArgs::try_from(RawSnipeCLIArgs::parse()) {
        Ok(args) => args,
        Err(err) => {
            print_args_validation_err(err);
            return ExitCode::FAILURE;
        }
    };

    match run_cli(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            print_err(err);
            ExitCode::FAILURE
        }
    }
}

fn print_args_validation_err(err: ArgsValidationError) {
    match err {
        ArgsValidationError::PrettyWithHTTP => {
            eprintln!("{}", get_pretty_with_http_err_msg())
        }
        _ => print_err(err),
    }
}

#[inline]
fn get_pretty_with_http_err_msg() -> String {
    format!(
        "{} the argument {} cannot be used with {}\n\n{} {} {} <TARGET> {} {}\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--pretty'".yellow(),
        "'--format http'".yellow(),
        "Usage:".bold().underline(),
        "snipe".bold(),
        "--target".bold(),
        "--format".bold(),
        "http".bold(),
        "--help".bold()
    )
}

#[inline]
fn print_err<T: Error>(err: T) {
    eprintln!("{err}");
}
