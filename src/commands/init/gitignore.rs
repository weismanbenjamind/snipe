use std::{marker::PhantomData, path::Path};

use super::InitError;
pub(super) struct Uninitialized;
pub(super) struct Initialized;

pub(super) struct GitIgnore<State> {
    contents: String,
    updated: bool,
    _state: PhantomData<State>,
}

impl GitIgnore<Uninitialized> {
    pub(super) fn from_file(path: &Path) -> Result<GitIgnore<Uninitialized>, InitError> {
        let contents = std::fs::read_to_string(path).map_err(|e| InitError::FileRead {
            path: path.into(),
            source: e,
        })?;

        Ok(GitIgnore::<Uninitialized> {
            contents,
            updated: false,
            _state: PhantomData::<Uninitialized>,
        })
    }

    pub(super) fn initialize(mut self) -> GitIgnore<Initialized> {
        let mut write_snipe_comment = true;

        if self.contents.trim().is_empty() {
            self.contents = String::new();
        } else if self.contents.ends_with("\n# snipe\n") {
            write_snipe_comment = false;
        } else if self.contents.ends_with("\n# snipe") {
            self.contents.push('\n');
            write_snipe_comment = false;
        } else if !self.contents.ends_with("\n") {
            self.contents.push_str("\n\n");
        } else if self.contents.ends_with("\n") && !self.contents.ends_with("\n\n") {
            self.contents.push('\n');
        }

        if write_snipe_comment {
            self.contents.push_str("# snipe\n");
        }

        GitIgnore::<Initialized> {
            contents: self.contents,
            updated: self.updated,
            _state: PhantomData::<Initialized>,
        }
    }
}

impl GitIgnore<Initialized> {
    pub(super) fn try_write_line(&mut self, line: &str) {
        let trimmed = line.trim_end_matches("\n").trim_start_matches("\n");
        let maybe_present_line = format!("\n{trimmed}\n");
        if !self.contents.contains(&maybe_present_line) {
            self.contents.push_str(trimmed);
            self.contents.push('\n');
            self.updated = true
        }
    }

    pub(super) fn to_file(&self, path: &Path) -> Result<(), InitError> {
        std::fs::write(path, &self.contents)
            .map_err(|e| InitError::build_write(".gitignore", path.into(), e))
    }

    pub(super) fn updated(&self) -> bool {
        self.updated
    }
}
