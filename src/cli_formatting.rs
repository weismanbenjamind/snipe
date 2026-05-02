use crate::errors::ArgsValidationError;
use colored::Colorize;

#[inline]
pub fn get_args_validation_err_msg(err: ArgsValidationError) -> String {
    match err {
        ArgsValidationError::PrettyWithHTTP => get_pretty_with_http_err_msg(),
        ArgsValidationError::PrettyWithBinary => get_pretty_with_binary_err_msg(),
        ArgsValidationError::NonBodyWithBinary => get_non_body_with_binary_err_msg(),
        ArgsValidationError::NoOutputFileWithBinary => get_binary_without_output_file_err_msg(),
        _ => err.to_string(),
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

#[inline]
fn get_binary_without_output_file_err_msg() -> String {
    format!(
        "{} the following required arguments were not provided:\n  {}\n\n{} {} <TARGET> {} <OUTPUT_FILE>\n\nFor more information, try '{}'.",
        "error:".red().bold(),
        "--output-file <OUTPUT_FILE>".green(),
        "Usage:".bold().underline(),
        "snipe shoot --target".bold(),
        "--format binary --output-file".bold(),
        "--help".bold()
    )
}

#[inline]
pub fn print_err(err: &str) {
    eprintln!("{err}");
}
