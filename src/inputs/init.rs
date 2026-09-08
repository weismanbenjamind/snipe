use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(about = "Initialize a repo for use with snipe")]
pub(crate) struct InitArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets",
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
        help = "Subdirectory within --dir (-d) option to house request payloads"
    )]
    pub(crate) payloads: PathBuf,

    #[arg(
        short,
        long,
        default_value = "responses",
        help = "Subdirectory within --dir (-d) option to house snipe responses"
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
        short,
        long,
        default_value = None,
        help = "Parent directory to which all operations should occur. Defaults to the present working directory"
    )]
    pub(crate) parent_dir: Option<PathBuf>,
}
