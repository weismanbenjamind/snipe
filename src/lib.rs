// TODO - Make sure pub/private are dialed in
mod client;
mod errors;
pub mod inputs;
mod response_data;
mod response_output;
pub mod run;
mod targets;
mod var_replacement;

// TODO - modify error scopes to allow for snipe::errors::<err>
pub use errors::{ArgsValidationError, RunError};
pub use inputs::{RawSnipeArgs, SnipeArgs};
pub use run::run;
