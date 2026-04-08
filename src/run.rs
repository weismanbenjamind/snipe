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

    // TODO - Add the json output option
    // TODO - Figure out what do if get image or something? Will current base64 encoding handle it?
    // TODO - allow for file output
    println!("{}", response.to_http_string(args.grab()));
    Ok(())
}
