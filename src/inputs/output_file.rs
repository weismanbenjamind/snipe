use clap::Args;
use std::path::{Path, PathBuf};

// * Note - Do not want public fields here. Only way in is via the CLI. Means input validation occured.
// * If ever update this struct to be built from something other than the CLI the ::new() or builder
// * should validate that output_file and skip_output_file are not passed at the same time
// * Clap handles this invariant for now and encapsulation ensures the only way to build this struct is via Clap
#[derive(Clone, Debug, Args)]
#[group(required = false)]
pub(crate) struct OutputFileArgs {
    #[arg(
        short,
        long,
        help = "Optional file that output should be written to. If passed contents will be written to this file and not stdout"
    )]
    pub(crate) output_file: Option<PathBuf>,

    #[arg(
        long,
        short,
        default_value = "false",
        conflicts_with = "output_file",
        help = "If writting to an output file should be omitted. Used when the config for a target specifies a write to an output file, but it should be disabled from the CLI. Cannot be passed with --output-file (-o)."
    )]
    pub(crate) skip_output_file: bool,
}

impl OutputFileArgs {
    pub(crate) fn into_parts(self) -> (Option<PathBuf>, bool) {
        (self.output_file, self.skip_output_file)
    }

    pub(crate) fn into_output_file(self) -> Option<PathBuf> {
        self.output_file
    }

    pub(crate) fn output_file(&self) -> Option<&Path> {
        self.output_file.as_deref()
    }
}
