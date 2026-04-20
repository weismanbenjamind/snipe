use clap::Parser;
use colored::Colorize;
use snipe::errors::ArgsValidationError;
use snipe::inputs::{RawSnipeArgs, SnipeArgs};
use snipe::run;
use std::error::Error;
use std::process::ExitCode;

// TODO - special logic to capture an error related to pretty and json and exit
// Also check to see if pretty actually has in an impact on HTTP output and if it does just use it
#[tokio::main]
async fn main() -> ExitCode {
    let args = match SnipeArgs::try_from(RawSnipeArgs::parse()) {
        Ok(args) => args,
        Err(err) => {
            print_args_validation_err(err);
            return ExitCode::FAILURE;
        }
    };

    match run(args).await {
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
