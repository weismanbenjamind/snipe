use crate::errors::ArgsValidationError;
use crate::inputs::format::{Format, RawFormat};
use crate::inputs::grab::{Grab, RawGrab};
use clap::Args;
use std::path::{Path, PathBuf};

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub struct RawShootArgs {
    #[arg(
        short,
        long,
        default_value = ".snipe_targets.toml",
        help = "Path to config for target HTTP requests"
    )]
    cfg: PathBuf, // TODO - this struct should no longer hold this value

    #[arg(short, long, help = "Target HTTP request to send")]
    target: String,

    #[command(flatten)]
    grab: RawGrab,

    #[arg(
        short,
        long,
        default_value = "http",
        help = "Format style for response data"
    )]
    format: RawFormat,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed"
    )]
    pretty: bool,

    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout"
    )]
    output_file: Option<PathBuf>,

    #[arg(
        short = 'e',
        long,
        default_value = "SNIPE_TARGETS",
        help = "Environment variable whose value will be used to look for cfg the file if the path pointed to by the --cfg (-c) argument does not exist. Pass 'skip' to disable searching for this env var"
    )]
    cfg_env: String, // TODO - this struct should no longer hold this value
}

impl RawShootArgs {
    pub fn new(
        cfg: PathBuf, // TODO - this struct should no longer hold this value
        target: String,
        grab: RawGrab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
        cfg_env: String, // TODO - this struct should no longer hold this value
    ) -> Self {
        Self {
            cfg, // TODO - this struct should no longer hold this value
            target,
            grab,
            format,
            pretty,
            output_file,
            cfg_env, // TODO - this struct should no longer hold this value
        }
    }

    pub fn cfg_path(&self) -> &Path {
        // TODO - this struct should no longer hold this value
        &self.cfg
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> RawGrab {
        self.grab
    }

    pub fn format(&self) -> RawFormat {
        self.format
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }

    pub fn cfg_env(&self) -> &str {
        // TODO - this struct should no longer hold this value
        &self.cfg_env
    }
}

#[derive(Clone, Debug)]
pub struct ShootArgs {
    cfg: PathBuf, // TODO - this struct should no longer hold this value
    target: String,
    grab: Grab,
    format: Format,
    pretty: bool,
    output_file: Option<PathBuf>,
    cfg_env: Option<String>, // TODO - this struct should no longer hold this value
}

impl ShootArgs {
    pub fn new(
        cfg: PathBuf, // TODO - this struct should no longer hold this value
        target: String,
        grab: Grab,
        format: RawFormat,
        pretty: bool,
        output_file: Option<PathBuf>,
        cfg_env: Option<String>, // TODO - this struct should no longer hold this value
    ) -> Result<Self, ArgsValidationError> {
        Ok(Self {
            cfg, // TODO - this struct should no longer hold this value
            target,
            grab,
            format: Format::new(format, pretty)?,
            pretty,
            output_file,
            cfg_env, // TODO - this struct should no longer hold this value
        })
    }

    pub fn cfg_path(&self) -> &Path {
        // TODO - this struct should no longer hold this value
        &self.cfg
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn grab(&self) -> Grab {
        self.grab
    }

    pub fn format(&self) -> RawFormat {
        self.format.into()
    }

    pub fn pretty(&self) -> bool {
        self.pretty
    }

    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }

    pub fn cfg_env(&self) -> Option<&str> {
        // TODO - this struct should no longer hold this value
        self.cfg_env.as_deref()
    }

    fn resolve_cfg_env(cfg_env: String) -> Option<String> {
        // TODO - this struct should no longer hold this value
        match cfg_env.to_lowercase().as_str() {
            "skip" => None,
            _ => Some(cfg_env.to_string()),
        }
    }
}

impl TryFrom<RawShootArgs> for ShootArgs {
    type Error = ArgsValidationError;
    fn try_from(value: RawShootArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            cfg: value.cfg, // TODO - this struct should no longer hold this value
            target: value.target,
            grab: Grab::from(value.grab),
            format: Format::new(value.format, value.pretty)?,
            pretty: value.pretty,
            output_file: value.output_file,
            cfg_env: Self::resolve_cfg_env(value.cfg_env), // TODO - this struct should no longer hold this value
        })
    }
}
