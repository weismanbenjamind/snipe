use crate::containers::{SecretString, global_replaceable::GlobalReplaceableLocal};
use std::collections::HashMap;

pub type Headers = HashMap<String, SecretString>;

impl GlobalReplaceableLocal for Headers {
    fn has_local(&self) -> bool {
        !self.is_empty()
    }
}
