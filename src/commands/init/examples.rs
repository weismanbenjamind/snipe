use std::path::Path;

use log::warn;

use crate::commands::InitError;

const EXAMPLE_PAYLOAD: &str = include_str!("./examples/example_payload.json");
const SNIPE_CFG_TEMPLATE: &str = include_str!("./examples/snipe_example.toml");
const PAYLOADS_DELIMITER: &str = "${{PAYLOADS}}";
const RESPONSES_DELIMITER: &str = "${{RESPONSES}}";
const EXAMPLE_PAYLOAD_FILE_DELIMTER: &str = "${{PAYLOAD_FILE}}";
const EXAMPLE_PAYLOAD_FILE_NAME: &str = "example.json";

pub(super) fn write_example_payload(payloads_dir: &Path) -> Result<(), InitError> {
    let resolved_path = payloads_dir.join(EXAMPLE_PAYLOAD_FILE_NAME);
    match resolved_path.exists() {
        true => {
            warn!(
                "Payload file already exists at {}. Skipping initialization of example payload. Some example functionality may not work as expected.",
                resolved_path.display()
            );
            Ok(())
        }
        false => std::fs::write(&resolved_path, EXAMPLE_PAYLOAD)
            .map_err(|e| InitError::build_write("example payload", resolved_path, e)),
    }
}

pub(super) fn write_example_snipe_cfg(
    cfg: &Path,
    payloads_dir: &Path,
    responses_dir: &Path,
) -> Result<(), InitError> {
    let contents = build_example_snipe_cfg(payloads_dir, responses_dir);

    std::fs::write(cfg, &contents)
        .map_err(|e| InitError::build_write("template config file", cfg.into(), e))
}

fn build_example_snipe_cfg(payloads_dir: &Path, responses_dir: &Path) -> String {
    let payloads_dir = path_to_string(payloads_dir);
    let responses_dir = path_to_string(responses_dir);

    ReplaceableString::new(SNIPE_CFG_TEMPLATE)
        .replace(PAYLOADS_DELIMITER, &payloads_dir, "payloads directory")
        .replace(RESPONSES_DELIMITER, &responses_dir, "responses directory")
        .replace(
            EXAMPLE_PAYLOAD_FILE_DELIMTER,
            EXAMPLE_PAYLOAD_FILE_NAME,
            "example payload file name",
        )
        .contents
}

fn path_to_string(path: &Path) -> String {
    format!("{}", path.display())
}

struct ReplaceableString {
    contents: String,
}

impl ReplaceableString {
    fn new(contents: &str) -> Self {
        Self {
            contents: contents.into(),
        }
    }

    fn replace(mut self, from: &str, to: &str, desc: &str) -> Self {
        let contents = &self.contents;
        match contents.contains(from) {
            true => self.contents = contents.replace(from, to),
            false => {
                warn!(
                    "Failed to replace {desc} in example file. Some example functionality may not work as expected."
                )
            }
        }

        self
    }
}
