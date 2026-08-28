use clap::Args;

// * Note - Do not want public fields here. Only way in is via the CLI. Means input validation occured.
// * If ever update this struct to be built from something other than the CLI the ::new() or builder
// * should validate that pretty and no_pretty are not passed at the same time
// * Clap handles this invariant for now and encapsulation ensures the only way to build this struct is via Clap
#[derive(Clone, Copy, Debug, Args)]
#[group(required = false)]
pub(crate) struct PrettyArgs {
    #[arg(
        short,
        long,
        default_value = "false",
        help = "If the output should be pretty printed. Only valid is the `--format json` (`-f json`) option is passed. No-op if `--format pretty-json` (`-f pretty-json`) is passed"
    )]
    pretty: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        conflicts_with = "pretty",
        help = "If pretty printing should be omitted. Used when the config for a target enables pretty, but it should be disabled from the CLI. Cannot be passed with --pretty (-p)"
    )]
    no_pretty: bool,
}

impl PrettyArgs {
    pub(crate) fn pretty(&self) -> bool {
        self.pretty
    }

    pub(crate) fn no_pretty(&self) -> bool {
        self.no_pretty
    }
}
