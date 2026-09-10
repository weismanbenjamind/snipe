mod errors;
mod examples;
mod gitignore;
mod run;

pub use errors::InitError;
use gitignore::GitIgnore;
pub(crate) use run::run_init_cmd;
