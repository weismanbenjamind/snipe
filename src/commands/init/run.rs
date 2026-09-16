use std::path::{Path, PathBuf};

use log::warn;

use super::{
    InitError,
    examples::{write_example_payload, write_example_snipe_cfg},
    gitignore::GitIgnore,
};
use crate::{
    commands::{SnipeResult, SuccessMsg},
    inputs::InitArgs,
};

enum ResolvedPaths<'a> {
    HasParent { cfg: PathBuf, snipe_dir: PathBuf },
    NoParent { cfg: &'a Path, snipe_dir: &'a Path },
}

impl<'a> ResolvedPaths<'a> {
    fn from_parent_opt(cfg: &'a Path, dir: &'a Path, parent: Option<&Path>) -> Self {
        match parent {
            Some(p) => Self::HasParent {
                cfg: p.join(cfg),
                snipe_dir: p.join(dir),
            },
            None => Self::NoParent {
                cfg,
                snipe_dir: dir,
            },
        }
    }

    fn cfg(&self) -> &Path {
        match self {
            Self::HasParent { cfg, .. } => cfg,
            Self::NoParent { cfg, .. } => cfg,
        }
    }

    fn snipe_dir(&self) -> &Path {
        match self {
            Self::HasParent { snipe_dir, .. } => snipe_dir,
            Self::NoParent { snipe_dir, .. } => snipe_dir,
        }
    }
}

pub(crate) fn run_init_cmd(args: InitArgs) -> SnipeResult {
    let resolved_paths =
        ResolvedPaths::from_parent_opt(args.cfg(), args.snipe_dir(), args.parent_dir());
    let resolved_cfg = resolved_paths.cfg();
    let resolved_snipe_dir = resolved_paths.snipe_dir();

    if resolved_cfg.exists() && !args.force() {
        let msg = format!(
            "Snipe config file already exists at {}. Pass --force (-f) to overwite this file.",
            resolved_cfg.display(),
        );
        return Ok(SuccessMsg(msg));
    }

    let payloads_dir = resolved_snipe_dir.join(args.payloads());
    let responses_dir = resolved_snipe_dir.join(args.responses());

    init_snipe_dir(resolved_snipe_dir)?;
    init_request_paylaods_dir(&payloads_dir)?;
    init_responses_dir(&responses_dir)?;
    init_snipe_cfg(
        resolved_cfg,
        args.snipe_dir(),
        args.payloads(),
        args.responses(),
    )?;

    if !args.skip_gitignore() {
        update_gitignore(args.parent_dir(), resolved_snipe_dir, resolved_cfg)?;
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
fn init_snipe_cfg(
    cfg: &Path,
    snipe_dir_root: &Path,
    payloads_dir: &Path,
    responses_dir: &Path,
) -> Result<(), InitError> {
    if let Some(parent) = cfg.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| InitError::build_dir_creation("parent dirs to config", parent.into(), e))?
    }

    let payloads_relative_dir = snipe_dir_root.join(payloads_dir);
    let responses_relative_dir = snipe_dir_root.join(responses_dir);

    write_example_snipe_cfg(cfg, &payloads_relative_dir, &responses_relative_dir)
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
