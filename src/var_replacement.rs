use log::debug;
use regex::{Captures, Regex};
use std::env;

use crate::errors::EnvVarReplaceError;

// Raw string to not treat \ as escape characters
// \$ to match literal '$'
// \{ENV\. to match literal '{ENV.'
// '(' to open the group we want to extract - called a capture groups
// `[` - Open a character class - single unit that matches one character
// '^}` - Match 'Anything but the } character`
// `]+` - close the character group - `[^}]+` - says match one or more characters that is not a '}'
//  `)` - Close capture group - allows use to capture - ([^}]+) - basically capture strings that aren't `}` that appear one or more times
// \} to match literal '}'
// Putting it all together - Give me a string that starts with '${ENV.', followed by one or more characters that are not `}`, follow by `}`
const ENV_VAR_PATTERN: &str = r"\$\{ENV\.([^}]+)\}";

pub fn replace_env_vars(input: &str) -> Result<String, EnvVarReplaceError> {
    let env_var_regex = Regex::new(ENV_VAR_PATTERN).map_err(get_regex_err)?;
    let mut missing_env_vars: Vec<String> = Vec::new();

    let replaced = env_var_regex.replace_all(input, |captures: &Captures| {
        // Note - duplicate using &captures[index] to prevent memory allocation
        debug!(
            "Found match {} and searching for env var {}",
            &captures[0], &captures[1]
        );
        env::var(&captures[1]).unwrap_or_else(|_| {
            missing_env_vars.push((captures[1]).to_string());
            String::from("MISSING")
        })
    });

    match missing_env_vars.is_empty() {
        true => Ok(replaced.to_string()),
        false => get_missing_env_vars_err(&missing_env_vars),
    }
}

#[inline]
fn get_regex_err(e: regex::Error) -> EnvVarReplaceError {
    EnvVarReplaceError::Base(format!(
        "Failed to create Regex object for env var replacement. Error: {e}",
    ))
}

#[inline]
fn get_missing_env_vars_err<T>(missing_env_vars: &[String]) -> Result<T, EnvVarReplaceError> {
    Err(EnvVarReplaceError::Base(format!(
        "Env vars {} not set for injection into request",
        missing_env_vars.join(", ")
    )))
}
