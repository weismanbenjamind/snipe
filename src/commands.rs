use crate::errors::RunError;

mod init;
mod list;
mod shoot;

pub use init::InitError;
pub(crate) use init::run_init_cmd;
pub(crate) use list::run_list_targets_cmd;
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
