// use kitty_remote_bindings_macros::match_fields;

// #[derive(Default, OptionsBuilder, IntoCommand)]
#[derive(Default)]
pub struct LsOptions {
    // TODO: matchers can be combined using Boolean operators
    // Matchers are not supported in v0.28.1
    // matcher: Option<Matcher>,
}

// derive OptionsBuilder

// impl LsOptions {
//     pub fn matcher(&mut self, matcher: Matcher) -> &Self {
//         self.matcher = Some(matcher);
//         self
//     }
// }

// derive IntoCommand
impl From<&LsOptions> for  tokio::process::Command {
    fn from(_value: &LsOptions) -> Self {
        let mut cmd = tokio::process::Command::new("kitty");
        cmd.args(["@", "ls"]);
        cmd
    }
}


// #### ------------------------------------------------
#[derive(Default)]
pub struct SendTextOptions {
    // TODO: matchers can be combined using Boolean operators
    matcher: Option<Matcher>,
}

impl SendTextOptions {
    pub fn matcher(&mut self, matcher: Matcher) -> &Self {
        self.matcher = Some(matcher);
        self
    }
}

// derive IntoCommand
impl From<&SendTextOptions> for  tokio::process::Command{
    fn from(value: &SendTextOptions) -> Self {
        let mut cmd = tokio::process::Command::new("kitty");
        cmd.args(["@", "send-text"]);

        if let Some(Matcher::Id(id)) = value.matcher {
            cmd.arg("--match".to_string());
            cmd.arg(format!("id:{id}"));
        }

        cmd
    }
}

// id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
pub enum Matcher {
    Id(u32),
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
