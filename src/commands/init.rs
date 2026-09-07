#![allow(dead_code)] // TODO - remove allow statement
use crate::inputs::InitArgs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum InitError {
    #[error("Failed to create {description} at path {path}. Error: {source}")]
    DirCreation {
        description: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directories to request payloads directory at path {0}. Error: {1}")]
    RequestPayloadsDirCreation(PathBuf, #[source] std::io::Error),
}

impl InitError {
    fn build_dir_creation(
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
}

fn run_init(args: InitArgs) -> Result<(), InitError> {
    init_snipe_dir(&args.dir)?;
    init_request_paylaods_dir(&args.dir, &args.payloads)?;
    init_responses_dir(&args.dir, &args.responses)
}

fn init_snipe_dir(dir: &Path) -> Result<(), InitError> {
    std::fs::create_dir_all(dir)
        .map_err(|e| InitError::build_dir_creation("snipe directory", dir.into(), e))
}

fn init_request_paylaods_dir(root_dir: &Path, payloads_dir: &Path) -> Result<(), InitError> {
    init_subdir(root_dir, payloads_dir, "request payloads directory")
}

fn init_responses_dir(root_dir: &Path, responses_dir: &Path) -> Result<(), InitError> {
    init_subdir(root_dir, responses_dir, " responses directory")
}

fn init_subdir(root_dir: &Path, subdir: &Path, description: &'static str) -> Result<(), InitError> {
    let resolved_dir = root_dir.join(subdir);
    std::fs::create_dir_all(&resolved_dir)
        .map_err(|e| InitError::build_dir_creation(description, resolved_dir, e))
}

// None on a .parent() means we're in absolute root or relative root
// In this case the path in question is a file - no need to create directories
fn init_snipe_cfg() {}

fn update_gitignore() {}
