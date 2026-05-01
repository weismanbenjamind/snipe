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

// TODO - dedupe all the code in the matches
// Also pull all this arg printing code into a module
fn print_args_validation_err(err: ArgsValidationError) {
    match err {
        ArgsValidationError::PrettyWithHTTP => eprintln!("{}", get_pretty_with_http_err_msg()),
        ArgsValidationError::PrettyWithBinary => eprintln!("{}", get_pretty_with_binary_err_msg()),
        ArgsValidationError::NonBodyWithBinary => {
            eprintln!("{}", get_non_body_with_binary_err_msg())
        }
        ArgsValidationError::NoOutputFileWithBinary => {
            eprintln!("{}", get_binary_without_output_file_err_msg())
        }
        _ => print_err(err),
    }
}

#[inline]
fn get_pretty_with_http_err_msg() -> String {
    format!(
        "{} the argument {} cannot be used with {}\n\n{} {} <TARGET> {}\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--pretty'".yellow(),
        "'--format http'".yellow(),
        "Usage:".bold().underline(),
        "snipe shoot --target".bold(),
        "--format http".bold(),
        "--help".bold()
    )
}

#[inline]
fn get_pretty_with_binary_err_msg() -> String {
    format!(
        "{} the argument {} cannot be used with {}\n\n{} {} <TARGET> {} <OUTPUT_FILE>\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--pretty'".yellow(),
        "'--format binary'".yellow(),
        "Usage:".bold().underline(),
        "snipe shoot --target".bold(),
        "--format binary --output-file".bold(),
        "--help".bold()
    )
}

#[inline]
fn get_non_body_with_binary_err_msg() -> String {
    format!(
        "{} the argument {} can only be used with {}\n\n{} {} <TARGET> {} <OUTPUT_FILE>\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--format binary'".yellow(),
        "'--body'".yellow(),
        "Usage:".bold().underline(),
        "snipe shoot --target".bold(),
        "--body --format binary --output-file".bold(),
        "--help".bold()
    )
}

// TODO - double check this is what clap output looks like when a depenent arg isn't passed
// // run `snipe shoot` and see output and mirror it
#[inline]
fn get_binary_without_output_file_err_msg() -> String {
    format!(
        "{} the argument {} can only be used with {}\n\n{} {} <TARGET> {} <OUTPUT_FILE>\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "'--format binary'".yellow(),
        "'--output-file <OUTPUT_FILE>'".yellow(),
        "Usage:".bold().underline(),
        "snipe shoot --target".bold(),
        "--format binary --output-file".bold(),
        "--help".bold()
    )
}

#[inline]
fn print_err<T: Error>(err: T) {
    eprintln!("{err}");
}
