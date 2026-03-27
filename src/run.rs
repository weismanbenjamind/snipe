use crate::errors::RunError;
use crate::inputs::SnipeArgs;
use crate::targets::Targets;

pub fn run(args: SnipeArgs) -> Result<(), RunError> {
    let targets = Targets::from_toml_file(args.targets_path())?;
    println!("{:#?}", targets);
    Ok(())
}