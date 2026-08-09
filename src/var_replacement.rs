use crate::containers::Vars;
use log::debug;
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::env;

use crate::errors::VarReplaceError;

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
const ENV_VAR_PATTERN_DEFAULT: &str = r"\$\{ENV\.([^}]+)\}";

// Raw string to not treat \ as escape characters
// \$ to match literal '$'
// \{VARS\. to match literal '{VARS.'
// '(' to open the group we want to extract - called a capture groups
// `[` - Open a character class - single unit that matches one character
// '^}` - Match 'Anything but the } character`
// `]+` - close the character group - `[^}]+` - says match one or more characters that is not a '}'
//  `)` - Close capture group - allows use to capture - ([^}]+) - basically capture strings that aren't `}` that appear one or more times
// \} to match literal '}'
// Putting it all together - Give me a string that starts with '${VARS.', followed by one or more characters that are not `}`, follow by `}`
const VAR_PATTERN_DEFAULT: &str = r"\$\{VARS\.([^}]+)\}";

const MISSING: &str = "MISSING";

pub(crate) fn resolve_vars(
    toml_str: &str,
    maybe_vars: Option<&Vars>,
    var_pattern: Option<&str>,
    env_pattern: Option<&str>,
) -> Result<String, VarReplaceError> {
    let mut replaced = replace_env_vars(toml_str, env_pattern)?;
    if let Some(vars) = maybe_vars {
        replaced = replace_vars(&replaced, vars, var_pattern)?;
    }
    Ok(replaced)
}

fn replace_vars(
    input: &str,
    vars: &Vars,
    var_pattern: Option<&str>,
) -> Result<String, VarReplaceError> {
    let (vars_regex, mut missing_vars) =
        init_for_replace(var_pattern.unwrap_or(VAR_PATTERN_DEFAULT))?;

    let replaced = vars_regex.replace_all(input, |captures: &Captures| {
        let target_var = &captures[1].to_lowercase();

        debug!(
            "Found match {} and searching for pre-defined variable {target_var} in toml.",
            &captures[0]
        );
        vars.get(target_var).unwrap_or_else(|| {
            missing_vars.insert(target_var.to_string());
            MISSING
        })
    });

    finalize_replace(replaced.as_ref(), &missing_vars, |vars| {
        format!(
            "Var(s) [{}] could not be found in configuaration file.",
            vars.join(", ")
        )
    })
}

fn replace_env_vars(input: &str, env_pattern: Option<&str>) -> Result<String, VarReplaceError> {
    let (env_var_regex, mut missing_vars) =
        init_for_replace(env_pattern.unwrap_or(ENV_VAR_PATTERN_DEFAULT))?;

    let replaced = env_var_regex.replace_all(input, |captures: &Captures| {
        // Note - duplicate using &captures[index] to prevent memory allocation
        debug!(
            "Found match {} and searching for env var {}.",
            &captures[0], &captures[1]
        );
        env::var(&captures[1]).unwrap_or_else(|_| {
            missing_vars.insert((captures[1]).to_string());
            String::from(MISSING)
        })
    });

    finalize_replace(replaced.as_ref(), &missing_vars, |vars| {
        format!(
            "Env var(s) [{}] not set for injection into request.",
            vars.join(", ")
        )
    })
}

#[inline]
fn init_for_replace(pattern: &str) -> Result<(Regex, HashSet<String>), VarReplaceError> {
    Ok((
        Regex::new(pattern).map_err(get_regex_err)?,
        HashSet::<String>::new(),
    ))
}

#[inline]
fn get_regex_err(e: regex::Error) -> VarReplaceError {
    VarReplaceError::Base(format!(
        "Failed to create Regex object for var replacement. Error: {e}",
    ))
}

fn finalize_replace(
    replaced: &str,
    missing_vars: &HashSet<String>,
    err_msg_factory: impl Fn(&[&str]) -> String,
) -> Result<String, VarReplaceError> {
    match missing_vars.is_empty() {
        true => Ok(replaced.to_string()),
        false => {
            let mut missing_vars: Vec<&str> = missing_vars.iter().map(|ele| ele.as_str()).collect();
            missing_vars.sort();
            Err(VarReplaceError::Base(err_msg_factory(&missing_vars)))
        }
    }
}
