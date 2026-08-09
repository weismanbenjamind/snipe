use clap::Parser;
use snipe::args::{RawSnipeCLIArgs, SnipeCLIArgs};
use snipe::formatting::{get_args_validation_err_msg, print_err};
use snipe::run_cli;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = match SnipeCLIArgs::try_from(RawSnipeCLIArgs::parse()) {
        Ok(args) => args,
        Err(err) => {
            print_err(&get_args_validation_err_msg(err));
            return ExitCode::FAILURE;
        }
    };

    match run_cli(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            print_err(&err.to_string());
            ExitCode::FAILURE
        }
    }
}
