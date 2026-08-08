use serde::{Deserialize, Serialize};

use crate::containers::SecretString;
use crate::containers::globals::GlobalReplaceableLocal;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Headers(HashMap<String, SecretString>);

// Allow for 'for ... in headers' without consuming headers (since implemented on &'a Headers)
// Also gives the .into_iter() on &Headers
impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a SecretString);
    type IntoIter = std::collections::hash_map::Iter<'a, String, SecretString>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl GlobalReplaceableLocal for Headers {
    fn has_local(&self) -> bool {
        !self.0.is_empty()
    }
}
