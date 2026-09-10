use std::path::Path;

use log::warn;

use super::{
    GitIgnore, InitError,
    examples::{write_example_payload, write_example_snipe_cfg},
};
use crate::{
    commands::{SnipeResult, SuccessMsg},
    inputs::InitArgs,
};

pub(crate) fn run_init_cmd(args: InitArgs) -> SnipeResult {
    if args.cfg.exists() && !args.force {
        let msg = format!(
            "Snipe config file already exists at {}. Pass --force (-f) to overwite this file.",
            args.cfg.display(),
        );
        return Ok(SuccessMsg(msg));
    }

    let snipe_dir = match &args.parent_dir {
        Some(parent) => parent.join(&args.dir),
        None => args.dir,
    };

    init_snipe_dir(&snipe_dir)?;

    let payloads_dir = snipe_dir.join(&args.payloads);
    let responses_dir = snipe_dir.join(&args.responses);

    init_request_paylaods_dir(&payloads_dir)?;
    init_responses_dir(&responses_dir)?;
    init_snipe_cfg(&args.cfg, &payloads_dir, &responses_dir)?;

    if !args.skip_gitignore {
        update_gitignore(args.parent_dir.as_deref(), &snipe_dir, &args.cfg)?;
    }

    Ok(SuccessMsg("Successfully initialized snipe.".to_string()))
}

fn init_snipe_dir(dir: &Path) -> Result<(), InitError> {
    std::fs::create_dir_all(dir)
        .map_err(|e| InitError::build_dir_creation("snipe directory", dir.into(), e))
}

fn init_request_paylaods_dir(payloads_dir: &Path) -> Result<(), InitError> {
    init_subdir(payloads_dir, "request payloads directory")?;
    write_example_payload(payloads_dir)
}

fn init_responses_dir(responses_dir: &Path) -> Result<(), InitError> {
    init_subdir(responses_dir, " responses directory")
}

fn init_subdir(subdir: &Path, description: &'static str) -> Result<(), InitError> {
    std::fs::create_dir_all(subdir)
        .map_err(|e| InitError::build_dir_creation(description, subdir.into(), e))
}

// None on a .parent() means we're in absolute root or relative root
// In this case the path in question is a file - no need to create directories
fn init_snipe_cfg(cfg: &Path, payloads_dir: &Path, responses_dir: &Path) -> Result<(), InitError> {
    if let Some(parent) = cfg.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| InitError::build_dir_creation("parent dirs to config", parent.into(), e))?
    }
    write_example_snipe_cfg(cfg, payloads_dir, responses_dir)
}

fn update_gitignore(parent: Option<&Path>, snipe_dir: &Path, cfg: &Path) -> Result<(), InitError> {
    let gitignore_path = parent.unwrap_or_else(|| Path::new(".")).join(".gitignore");

    match (gitignore_path.exists(), gitignore_path.is_file()) {
        (true, true) => write_gitignore_update(&gitignore_path, snipe_dir, cfg)?,
        (false, true) | (false, false) => warn!(
            ".gitignore at path {} does not exist. Skipping update.",
            gitignore_path.display()
        ),
        (true, false) => warn!(
            ".gitignore at path {} is a directory. Skipping update.",
            gitignore_path.display()
        ),
    }

    Ok(())
}

fn write_gitignore_update(
    gitignore_path: &Path,
    snipe_dir: &Path,
    cfg: &Path,
) -> Result<(), InitError> {
    let mut gitignore = GitIgnore::from_file(gitignore_path)?.initialize();

    let snipe_dir_line = path_to_string(snipe_dir, true);
    let cfg_line = path_to_string(cfg, false);

    gitignore.try_write_line(&snipe_dir_line);
    gitignore.try_write_line(&cfg_line);

    if gitignore.updated() {
        gitignore.to_file(gitignore_path)?;
    }

    Ok(())
}

fn path_to_string(path: &Path, is_dir: bool) -> String {
    match is_dir {
        true => format!("/{}", path.display()),
        false => format!("{}", path.display()),
    }
}
