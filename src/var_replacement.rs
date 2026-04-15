use log::debug;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use crate::errors::EnvVarReplaceError; // TODO - Update this error - need a general error for variable replacement

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

const VAR_PATTERN: &str = r"\$\{VAR\.([^}]+)\}";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Vars {
    variables: HashMap<String, String>,
}

impl Vars {
    fn get_owned(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

pub fn replace_variables(toml_str: &str) {
    let vars: Vars = toml::from_str(toml_str).unwrap();
    let vars_regex = build_regex(VAR_PATTERN).unwrap();
    let mut missing_vars: Vec<String> = Vec::new();
    let mut target_var: &str = "";

    let replaced = vars_regex.replace_all(toml_str, |captures: &Captures| {
        target_var = &captures[1].to_lowercase(); // TODO - might have to allocate memory for every variable in the closure

        debug!(
            "Found match {} and searching for pre-defined variable {target_var} in toml.",
            &captures[0]
        );

        vars.get_owned(target_var).unwrap_or_else(|| {
            missing_vars.push(target_var.to_string());
            &String::from("MISSING") // TODO - figure out if can returned a borrow value - otherwise might need a clone or to_owned
        })
    });
}

pub fn replace_env_vars(input: &str) -> Result<String, EnvVarReplaceError> {
    let env_var_regex = build_regex(ENV_VAR_PATTERN)?;
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
fn build_regex(pattern: &str) -> Result<Regex, EnvVarReplaceError> {
    Regex::new(pattern).map_err(get_regex_err)
}

#[inline]
fn get_regex_err(e: regex::Error) -> EnvVarReplaceError {
    EnvVarReplaceError::Base(format!(
        "Failed to create Regex object for var replacement. Error: {e}",
    ))
}

#[inline]
fn get_missing_env_vars_err<T>(missing_env_vars: &[String]) -> Result<T, EnvVarReplaceError> {
    Err(EnvVarReplaceError::Base(format!(
        "Env vars {} not set for injection into request",
        missing_env_vars.join(", ")
    )))
}
