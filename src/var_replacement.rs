use log::debug;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use toml::de::Error;

use crate::errors::VarReplaceError; // TODO - Update this error - need a general error for variable replacement

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
const ENV_VAR_PATTERN: &str = r"\$\{ENV\.([^}]+)\}"; // Inject into function and use this as default

const VAR_PATTERN: &str = r"\$\{VAR\.([^}]+)\}"; // Inject into function as use this as default

const MISSING: &str = "MISSING";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Vars {
    vars: HashMap<String, String>,
}

impl Vars {
    fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(|key| key.as_str())
    }
}

// Add this to source and get yellow squiggles to go away - make sure it works
pub fn replace_variables(toml_str: &str) -> Result<String, VarReplaceError> {
    let vars: Vars = toml::from_str(toml_str).map_err(get_failed_vars_parse_err)?;
    let (vars_regex, mut missing_vars) = init_for_replace(VAR_PATTERN)?;

    let replaced = vars_regex.replace_all(toml_str, |captures: &Captures| {
        let target_var = &captures[1].to_lowercase();

        debug!(
            "Found match {} and searching for pre-defined variable {target_var} in toml.",
            &captures[0]
        );

        vars.get(target_var).unwrap_or_else(|| {
            missing_vars.push(target_var.to_string());
            MISSING
        })
    });

    finalize_replace(&replaced.to_string(), &missing_vars, get_missing_vars_err)
}

pub fn replace_env_vars(input: &str) -> Result<String, VarReplaceError> {
    let (env_var_regex, mut missing_vars) = init_for_replace(ENV_VAR_PATTERN)?;

    let replaced = env_var_regex.replace_all(input, |captures: &Captures| {
        // Note - duplicate using &captures[index] to prevent memory allocation
        debug!(
            "Found match {} and searching for env var {}",
            &captures[0], &captures[1]
        );
        env::var(&captures[1]).unwrap_or_else(|_| {
            missing_vars.push((captures[1]).to_string());
            String::from(MISSING)
        })
    });

    finalize_replace(
        &replaced.to_string(),
        &missing_vars,
        get_missing_env_vars_err,
    )
}

#[inline]
fn init_for_replace(pattern: &str) -> Result<(Regex, Vec<String>), VarReplaceError> {
    Ok((build_regex(pattern)?, Vec::<String>::new()))
}

#[inline]
fn build_regex(pattern: &str) -> Result<Regex, VarReplaceError> {
    Regex::new(pattern).map_err(get_regex_err)
}

#[inline]
fn get_regex_err(e: regex::Error) -> VarReplaceError {
    VarReplaceError::Base(format!(
        "Failed to create Regex object for var replacement. Error: {e}",
    ))
}

#[inline]
fn get_failed_vars_parse_err(e: Error) -> VarReplaceError {
    VarReplaceError::Base(format!("Failed to parse vars from toml file. Err {e}"))
}

fn finalize_replace(
    replaced: &str,
    missing_vars: &[String],
    err_func: fn(&[String]) -> Result<String, VarReplaceError>,
) -> Result<String, VarReplaceError> {
    match missing_vars.is_empty() {
        true => Ok(replaced.to_string()),
        false => err_func(missing_vars),
    }
}

#[inline]
fn get_missing_env_vars_err<T>(missing_env_vars: &[String]) -> Result<T, VarReplaceError> {
    Err(VarReplaceError::Base(format!(
        "Env vars {} not set for injection into request.",
        missing_env_vars.join(", ")
    )))
}

#[inline]
fn get_missing_vars_err<T>(missing_vars: &[String]) -> Result<T, VarReplaceError> {
    Err(VarReplaceError::Base(format!(
        "Vars {} could not be found in configuaration file.",
        missing_vars.join(", ")
    )))
}
