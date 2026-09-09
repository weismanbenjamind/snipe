#![allow(dead_code)] // TODO - remove allow statement
use crate::inputs::InitArgs;
use bon::Builder;
use gitignore::GitIgnore;
use log::warn;
use serde::Serialize;
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

// TODO - This should return the result struct for a run operation
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

    match (gitignore_path.exists(), gitignore_path.is_file()) {
        (true, true) => write_gitignore_update(&gitignore_path, snipe_dir, cfg)?,
        (false, true) | (false, false) => warn!(
            ".gitignore at path {} does not exist. Skipping update.",
            gitignore_path.display()
        ),
        (true, false) => warn!(
            ".gitignore at path {} is a directory. Skipping update.",
            gitignore_path.display()
        ),
    }

    Ok(())
}

fn write_gitignore_update(
    gitignore_path: &Path,
    snipe_dir: &Path,
    cfg: &Path,
) -> Result<(), InitError> {
    let mut gitignore = GitIgnore::from_file(gitignore_path)?.initialize();

    let snipe_dir_line = build_gitignore_line(snipe_dir);
    let cfg_line = build_gitignore_line(cfg);

    gitignore.try_write_line(&snipe_dir_line);
    gitignore.try_write_line(&cfg_line);

    if gitignore.updated() {
        gitignore.to_file(gitignore_path)?;
    }

    Ok(())
}

fn build_gitignore_line(path: &Path) -> String {
    format!("\n{}\n", path.display())
}

mod gitignore {
    use super::InitError;
    use std::marker::PhantomData;
    use std::path::Path;
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
            if !self.contents.ends_with("\n") {
                self.contents.push('\n');
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
            if !self.contents.contains(line) {
                self.contents.push_str(line);
                self.updated = true
            }
        }

        pub(super) fn to_file(&self, path: &Path) -> Result<(), InitError> {
            std::fs::write(path, &self.contents).map_err(|e| InitError::Write {
                description: ".gitignore",
                path: path.into(),
                source: e,
            })?;

            Ok(())
        }

        pub(super) fn updated(&self) -> bool {
            self.updated
        }
    }
}
