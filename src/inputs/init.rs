use std::path::{Path, PathBuf};

use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(about = "Initialize a repo for use with snipe")]
pub(crate) struct InitArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config to create for target HTTP requests"
    )]
    pub(crate) cfg: PathBuf,

    #[arg(
        short,
        long,
        default_value = ".snipe",
        help = "Directory for snipe related inputs, outputs, and configurations"
    )]
    pub(crate) dir: PathBuf,

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

#[derive(Clone, Debug)]
struct _ValidatedInitArgs {
    cfg: PathBuf,
    dir: PathBuf,
    payloads: PathBuf,
    responses: PathBuf,
    skip_gitignore: bool,
    parent_dir: Option<PathBuf>,
    force: bool,
}

impl From<InitArgs> for _ValidatedInitArgs {
    fn from(value: InitArgs) -> Self {
        if value.parent_dir.as_deref().is_some_and(|p| p.is_absolute()) {
            _validate_args_for_absolute_parent(
                &value.cfg,
                &value.dir,
                &value.payloads,
                &value.responses,
            );
        }

        _validate_path_subdir(&value.payloads, "--payloads", "-p");
        _validate_path_subdir(&value.responses, "--responses", "-r");

        Self {
            cfg: value.cfg,
            dir: value.dir,
            payloads: value.payloads,
            responses: value.responses,
            skip_gitignore: value.skip_gitignore,
            parent_dir: value.parent_dir,
            force: value.force,
        }
    }
}

fn _validate_args_for_absolute_parent(cfg: &Path, dir: &Path, payloads: &Path, responses: &Path) {
    if cfg.is_absolute() {
        panic!("Cannot specify abosolute path for --parent-dir (-a) and --cfg (-c)")
    }

    if dir.is_absolute() {
        panic!("Cannot specify absolute path for --parent-dir (-a) and --dir (-d)")
    }

    if payloads.is_absolute() {
        panic!("Cannot specify absolute path for --parent-dir (-a) and --paylaods (-p)")
    }

    if responses.is_absolute() {
        panic!("Cannot specify absolute path for --parent-dir (-a) and --responses (-r)")
    }
}

fn _validate_path_subdir(path: &Path, long_arg: &str, short_arg: &str) {
    if !path.is_relative() {
        let msg = format!("{long_arg} ({short_arg}) must be a relative path");
        panic!("{msg}")
    }
}
