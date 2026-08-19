use crate::errors::RunError;

mod list_targets;
mod shoot;

pub(crate) use list_targets::run_list_targets_cmd;
pub(crate) use shoot::run_shoot_cmd;

pub struct SuccessMsg(pub(crate) String);

impl SuccessMsg {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SuccessMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) type SnipeResult = Result<SuccessMsg, RunError>;
