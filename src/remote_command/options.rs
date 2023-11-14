use crate::model::WindowId;

/// Represents the `--match` option
//e id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
#[derive(Clone, Debug, PartialEq)]
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

/// Provides convenience function to set the matcher of commands that supports this option.
pub trait MatcherExt {
    fn matcher(&mut self, matcher: Matcher) -> &Self;
}
