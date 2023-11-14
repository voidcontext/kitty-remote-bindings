use crate::model::WindowId;

// id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
#[derive(Clone, Debug, PartialEq)]
pub enum Matcher {
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

pub trait MatcherOption {
    fn matcher(&mut self, matcher: Matcher) -> &Self;
}
