use std::path::PathBuf;

use kitty_remote_bindings_macros::KittyCommandOption;

use crate::model::WindowId;

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

/// Represents the possible values of th launch command's `--type` option
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

/// Represents the possible values of the `--cwd` option
#[derive(Clone, Debug, PartialEq, KittyCommandOption)]
pub enum Cwd {
    Current,
    LastReported,
    Root,
    Path(PathBuf),
}
