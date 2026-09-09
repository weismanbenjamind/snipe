pub(super) fn resolve_cfg_env(cfg_env: &str) -> Option<String> {
    match cfg_env.to_lowercase().as_str() {
        "skip" => None,
        _ => Some(cfg_env.into()),
    }
}
