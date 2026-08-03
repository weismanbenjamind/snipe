use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vars {
    vars: HashMap<String, String>,
}

impl Vars {
    pub fn get(&self, value: &str) -> Option<&str> {
        self.vars.get(value).map(|val| val.as_str())
    }
}
