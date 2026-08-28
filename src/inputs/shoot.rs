use crate::inputs::format::RawFormat;
use crate::inputs::grab::RawGrab;
use crate::inputs::output_file::OutputFileArgs;
use crate::inputs::pretty::PrettyArgs;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[command(about = "Make a specific API request")]
pub(crate) struct ShootArgs {
    #[arg(help = "Target HTTP request to send")]
    pub(crate) target: String,

    #[command(flatten)]
    pub(crate) grab: Option<RawGrab>,

    #[arg(short, long, help = "Format style for response data")]
    pub(crate) format: Option<RawFormat>,

    #[command(flatten)]
    pub(crate) pretty_args: PrettyArgs,

    #[command(flatten)]
    pub(crate) output_file_args: OutputFileArgs,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If request should be built but not sent"
    )]
    pub(crate) dry_run: bool,
}
