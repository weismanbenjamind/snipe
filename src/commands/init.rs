#![allow(dead_code)] // TODO - remove allow statement
use crate::inputs::InitArgs;
use bon::Builder;
use serde::Serialize;
use std::fmt::Write;
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

    #[error("Failed to serialize config template. Error: {0}")]
    CfgSerialization(#[from] toml::ser::Error),

    // TODO - Make builer function for this enum case
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

#[derive(Builder, Clone, Copy, Debug, Serialize)]
struct Template<'a> {
    payloads_dir: &'a Path,
    responses_dir: &'a Path,
}

impl<'a> Template<'a> {
    fn into_toml_string(self) -> Result<String, InitError> {
        toml::to_string_pretty(&self).map_err(InitError::from)
    }
}

fn run_init(args: InitArgs) -> Result<String, InitError> {
    let snipe_dir = match &args.parent_dir {
        Some(parent) => parent.join(&args.dir),
        None => args.dir,
    };

    init_snipe_dir(&snipe_dir)?;

    let payloads_dir = snipe_dir.join(&args.payloads);
    let responses_dir = snipe_dir.join(&args.responses);

    init_request_paylaods_dir(&payloads_dir)?;
    init_responses_dir(&responses_dir)?;
    init_snipe_cfg(&args.cfg, &payloads_dir, &responses_dir)?;

    if !args.skip_gitignore {
        update_gitignore(args.parent_dir.as_deref(), &snipe_dir, &args.cfg)?;
    }

    Ok("Successfully initialized snipe".to_string())
}

fn init_snipe_dir(dir: &Path) -> Result<(), InitError> {
    std::fs::create_dir_all(dir)
        .map_err(|e| InitError::build_dir_creation("snipe directory", dir.into(), e))
}

fn init_request_paylaods_dir(payloads_dir: &Path) -> Result<(), InitError> {
    init_subdir(payloads_dir, "request payloads directory")
}

fn init_responses_dir(responses_dir: &Path) -> Result<(), InitError> {
    init_subdir(responses_dir, " responses directory")
}

fn init_subdir(subdir: &Path, description: &'static str) -> Result<(), InitError> {
    std::fs::create_dir_all(subdir)
        .map_err(|e| InitError::build_dir_creation(description, subdir.into(), e))
}

// None on a .parent() means we're in absolute root or relative root
// In this case the path in question is a file - no need to create directories
fn init_snipe_cfg(cfg: &Path, payloads_dir: &Path, responses_dir: &Path) -> Result<(), InitError> {
    if let Some(parent) = cfg.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| InitError::build_dir_creation("parent dirs to config", parent.into(), e))?
    }

    let contents = Template::builder()
        .payloads_dir(payloads_dir)
        .responses_dir(responses_dir)
        .build()
        .into_toml_string()?;

    std::fs::write(cfg, contents).map_err(|e| InitError::Write {
        description: "template config file",
        path: cfg.into(),
        source: e,
    })
}

fn update_gitignore(parent: Option<&Path>, snipe_dir: &Path, cfg: &Path) -> Result<(), InitError> {
    let gitignore_path = parent.unwrap_or_else(|| Path::new(".")).join(".gitignore");

    // TODO - this if block should be a function
    // TODO - If two conditions below are not satisfied may want to warn
    if gitignore_path.exists() && gitignore_path.is_file() {
        let mut contents =
            std::fs::read_to_string(&gitignore_path).map_err(|e| InitError::FileRead {
                path: gitignore_path.clone(), // TODO - remove clone
                source: e,
            })?;

        // TODO - Create a function to make lines
        let snipe_dir_line = format!("\n{}\n", snipe_dir.display());
        let cfg_line = format!("\n{}\n", cfg.display());

        if !contents.ends_with("\n") {
            contents.push('\n')
        }

        let mut updated_gitignore = false;

        // Writing to a string cannot fail
        // use `let _ =` notation
        // TODO - Two if statements below could be a function
        if !contents.contains(&snipe_dir_line) {
            let _ = write!(contents, "{snipe_dir_line}");
            updated_gitignore = true;
        }

        if !contents.contains(&cfg_line) {
            let _ = write!(contents, "{cfg_line}");
            updated_gitignore = true;
        }

        if updated_gitignore {
            std::fs::write(&gitignore_path, contents).map_err(|e| InitError::Write {
                description: ".gitignore",
                path: gitignore_path,
                source: e,
            })?;
        }
    }

    Ok(())
}
