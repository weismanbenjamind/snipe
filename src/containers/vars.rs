use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Vars {
    vars: HashMap<String, String>,
}

impl Vars {
    pub(crate) fn get(&self, value: &str) -> Option<&str> {
        self.vars.get(value).map(|val| val.as_str())
    }
}
