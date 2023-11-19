use std::path::PathBuf;

use kitty_remote_bindings_macros::KittyCommandOption;

use crate::model::WindowId;

pub trait ToArg {
    fn to_arg(&self) -> Vec<String>;
}

impl ToArg for String {
    fn to_arg(&self) -> Vec<String> {
        vec![self.clone()]
    }
}

impl ToArg for PathBuf {
    fn to_arg(&self) -> Vec<String> {
        vec![self.to_string_lossy().to_string()]
    }
}

impl<T: ToArg> ToArg for &T {
    fn to_arg(&self) -> Vec<String> {
        (*self).to_arg()
    }
}

impl<T: ToArg> ToArg for Vec<T> {
    fn to_arg(&self) -> Vec<String> {
        self.iter().flat_map(ToArg::to_arg).collect()
    }
}

/// Represents the `--match` option
//e id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
#[derive(Clone, Debug, PartialEq, KittyCommandOption)]
pub enum Matcher {
    /// Match by windows id `--match id:windows_id`
    #[prefix]
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

#[derive(Clone, Debug, PartialEq, KittyCommandOption)]
pub enum LaunchType {
    Window,
    Tab,
    OsWindow,
    Overlay,
    OverlayMain,
    Background,
    Clipboard,
    Primary,
}

#[derive(Clone, Debug, PartialEq, KittyCommandOption)]
pub enum Cwd {
    Current,
    LastReported,
    Root,
    Path(PathBuf),
}
