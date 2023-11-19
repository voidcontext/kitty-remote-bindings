use kitty_remote_bindings_macros::KittyCommandOption;

use crate::model::WindowId;

pub trait ToArg {
    fn to_arg(&self) -> String;
}

impl ToArg for String {
    fn to_arg(&self) -> String {
        self.clone()
    }
}

/// Represents the `--match` option
//e id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
#[derive(Clone, Debug, PartialEq, KittyCommandOption)]
pub enum Matcher {
    /// Match by windows id `--match id:windows_id`
    Id(WindowId),
    // Title(String),
    // Pid(u32),
    // Cwd(String),
    // CmdLine(String),
    // Num(u32),
    // Env(String),
    // Var(String),
    // State(String),
    // Neighbor(String),
    // Recent(u32),
}
