use std::path::{Path, PathBuf};

use clap::Args;
use thiserror::Error;

#[derive(Args, Debug, Clone)]
#[command(about = "Initialize a repo for use with snipe")]
pub(super) struct RawInitArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config to create for target HTTP requests"
    )]
    pub(crate) cfg: PathBuf,

    #[arg(
        short = 'd',
        long,
        default_value = ".snipe",
        help = "Directory for snipe related inputs, outputs, and configurations"
    )]
    pub(crate) snipe_dir: PathBuf,

    #[arg(
        short,
        long,
        default_value = "payloads",
        help = "Relative directory within --dir (-d) option to house request payloads. Cannot be specified as an absolute path"
    )]
    pub(crate) payloads: PathBuf,

    #[arg(
        short,
        long,
        default_value = "responses",
        help = "Relative within --dir (-d) option to house snipe responses. Cannot be specified as an absolute path"
    )]
    pub(crate) responses: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If adding snipe releated files and directories to the repos .gitignore should be skipped"
    )]
    pub(crate) skip_gitignore: bool,

    #[arg(
        short = 'a',
        long,
        default_value = None,
        help = "Parent directory to which all operations should occur. Defaults to the present working directory"
    )]
    pub(crate) parent_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If should force initialization if the snipe config file specified by --cfg (-c) already exists"
    )]
    pub(crate) force: bool,
}

#[derive(Clone, Debug, Error)]
pub enum InitArgsError {
    #[error(
        "Cannot specify abosulte path for --parent-dir (-a) and {long_arg} ({short_arg}). Found --parent-dir={parent_dir} and {long_arg}={arg_path}"
    )]
    AbosolutePathWithParent {
        long_arg: &'static str,
        short_arg: &'static str,
        parent_dir: PathBuf,
        arg_path: PathBuf,
    },

    #[error(
        "{long_arg} ({short_arg}) must be a relative path. Expected to be a path relative to the --snipe-dir (-d) option. Found {path}"
    )]
    NonRelativePath {
        long_arg: &'static str,
        short_arg: &'static str,
        path: PathBuf,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct InitArgs {
    cfg: PathBuf,
    snipe_dir: PathBuf,
    payloads: PathBuf,
    responses: PathBuf,
    skip_gitignore: bool,
    parent_dir: Option<PathBuf>,
    force: bool,
}

impl InitArgs {
    pub(crate) fn cfg(&self) -> &Path {
        &self.cfg
    }

    pub(crate) fn snipe_dir(&self) -> &Path {
        &self.snipe_dir
    }

    pub(crate) fn payloads(&self) -> &Path {
        &self.payloads
    }

    pub(crate) fn responses(&self) -> &Path {
        &self.responses
    }

    pub(crate) fn skip_gitignore(&self) -> bool {
        self.skip_gitignore
    }

    pub(crate) fn parent_dir(&self) -> Option<&Path> {
        self.parent_dir.as_deref()
    }

    pub(crate) fn force(&self) -> bool {
        self.force
    }
}

impl TryFrom<RawInitArgs> for InitArgs {
    type Error = InitArgsError;
    fn try_from(value: RawInitArgs) -> Result<Self, InitArgsError> {
        if let Some(parent_dir) = (value.parent_dir.as_deref())
            && parent_dir.is_absolute()
        {
            let validator = ParentDirValidator::new(parent_dir);
            validator.validate_non_absolute_path(&value.cfg, "--cfg", "-c")?;
            validator.validate_non_absolute_path(&value.snipe_dir, "--snipe-dir", "-d")?;
            validator.validate_non_absolute_path(&value.payloads, "--payloads", "-p")?;
            validator.validate_non_absolute_path(&value.responses, "--responses", "-r")?;
        }

        validate_path_subdir(&value.payloads, "--payloads", "-p")?;
        validate_path_subdir(&value.responses, "--responses", "-r")?;

        Ok(Self {
            cfg: value.cfg,
            snipe_dir: value.snipe_dir,
            payloads: value.payloads,
            responses: value.responses,
            skip_gitignore: value.skip_gitignore,
            parent_dir: value.parent_dir,
            force: value.force,
        })
    }
}

struct ParentDirValidator<'a> {
    parent_dir: &'a Path,
}

impl<'a> ParentDirValidator<'a> {
    fn new(parent_dir: &'a Path) -> Self {
        Self { parent_dir }
    }

    fn validate_non_absolute_path(
        &self,
        path: &Path,
        long_arg: &'static str,
        short_arg: &'static str,
    ) -> Result<(), InitArgsError> {
        if path.is_absolute() {
            return Err(InitArgsError::AbosolutePathWithParent {
                long_arg,
                short_arg,
                parent_dir: self.parent_dir.into(),
                arg_path: path.into(),
            });
        }

        Ok(())
    }
}

fn validate_path_subdir(
    path: &Path,
    long_arg: &'static str,
    short_arg: &'static str,
) -> Result<(), InitArgsError> {
    if !path.is_relative() {
        return Err(InitArgsError::NonRelativePath {
            long_arg,
            short_arg,
            path: path.into(),
        });
    }

    Ok(())
}
