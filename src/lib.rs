mod cfg_resolver;
mod cli_formatting;
mod client;
mod commands;
mod containers;
mod errors;
mod inputs;
mod response;
mod run;
mod var_replacement;

pub use run::run_cli;

pub mod args {
    pub use super::inputs::{RawSnipeCLIArgs, SnipeCLIArgs};
}

pub mod formatting {
    pub use super::cli_formatting::{get_args_validation_err_msg, print_err};
}
