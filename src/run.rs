use log::info;

use crate::client::Client;
use crate::errors::RunError;
use crate::inputs::SnipeArgs;
use crate::targets::Targets;

pub async fn run(args: SnipeArgs) -> Result<(), RunError> {
    let targets = Targets::from_toml_file(args.cfg_path())?;

    let target = targets
        .get_target(args.target())
        .ok_or_else(|| RunError::Failure(format!("Failed to find target {}", args.target())))?;

    info!("Sending request for target {}", target.name());
    let response = Client::new()?.send_request(target).await?;

    // TODO - Might not want to force json representation - allow for different reprsentations. Forcing json might cause errors. Check convo with forge about this
    // TODO - Want to add a full vs. just body reprsentation
    // TODO - Might want to eprintln! if get bad response
    // TODO - allow for file output
    println!("{}", response.to_json()?);
    Ok(())
}

// TODO - pick up here
// fn get_output_string(response_wrapper: &ResponseData, grab: Grab) {
//     match grab {
//         Grab::StatusCode => format!("{}", response_wrapper.status_code()),
//         Grab::Headers => to_string_pretty(response_wrapper.headers()),
//     };
// }
