use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Failed to create {description} at path {path}. Error: {source}")]
    DirCreation {
        description: &'static str,
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("Failed to serialize config template. Error: {0}")]
    CfgSerialization(#[from] toml::ser::Error),

    #[error("Failed to write {description} to path {path}. Error: {source}")]
    Write {
        description: &'static str,

        path: PathBuf,

        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read contents of file at {path}. Error: {source}")]
    FileRead {
        path: PathBuf,

        #[source]
        source: std::io::Error,
    },
}

impl InitError {
    pub(super) fn build_dir_creation(
        description: &'static str,
        path: PathBuf,
        source: std::io::Error,
    ) -> Self {
        Self::DirCreation {
            description,
            path,
            source,
        }
    }

    pub(super) fn build_write(
        description: &'static str,
        path: PathBuf,
        source: std::io::Error,
    ) -> Self {
        Self::Write {
            description,
            path,
            source,
        }
    }
}
