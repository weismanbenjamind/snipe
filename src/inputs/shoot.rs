use crate::inputs::format::RawFormat;
use crate::inputs::grab::RawGrab;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub(crate) struct ShootArgs {
    #[arg(help = "Target HTTP request to send")]
    pub(crate) target: String,

    #[command(flatten)]
    pub(crate) grab: Option<RawGrab>,

    #[arg(short, long, help = "Format style for response data")]
    pub(crate) format: Option<RawFormat>,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed. No-op if `--format pretty-json` (`-f pretty-json`) is passed"
    )]
    pub(crate) pretty: bool,

    // TODO - May want to add encap here to prevent someone from setting --pretty and --no-pretty outside the CLI
    #[arg(
        short,
        long,
        default_value = "false",
        conflicts_with = "pretty",
        help = "If pretty printing should be omitted. Used when the config for a target enables pretty, but it should be disabled from the CLI. Cannot be passed with --pretty (-p)."
    )]
    pub(crate) no_pretty: bool,

    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout"
    )]
    pub(crate) output_file: Option<PathBuf>,

    // TODO - May want to add encap here to prevent someone from setting --skip-output-file and --output-file outside the CLI
    #[arg(
        long,
        short,
        default_value = "false",
        conflicts_with = "output_file",
        help = "If writting to an output file should be omitted. Used when the config for a target specifies a write to an output file, but it should be disabled from the CLI. Cannot be passed with --output-file (-o)."
    )]
    pub(crate) skip_output_file: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If request should be built but not sent"
    )]
    pub(crate) dry_run: bool,
}
