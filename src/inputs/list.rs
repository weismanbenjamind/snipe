use super::cfg_env::resolve_cfg_env;
use std::path::PathBuf;

use clap::Args;

#[derive(Clone, Debug, Args)]
#[command(about = "List all potential API requests to make")]
pub(crate) struct RawListArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config for target HTTP requests"
    )]
    pub(crate) cfg: PathBuf,

    #[arg(
        short,
        long,
        default_value = "SNIPE_TARGETS",
        help = "Environment variable whose value will be used to look for cfg the file if the path pointed to by the --cfg (-c) argument does not exist. Pass 'skip' to disable searching for this env var"
    )]
    cfg_env: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ListArgs {
    pub(crate) cfg: PathBuf,
    pub(crate) cfg_env: Option<String>,
}

impl From<RawListArgs> for ListArgs {
    fn from(value: RawListArgs) -> Self {
        let cfg_env = resolve_cfg_env(&value.cfg_env);

        Self {
            cfg: value.cfg,
            cfg_env,
        }
    }
}
