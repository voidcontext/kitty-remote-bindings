use kitty_remote_bindings_macros::match_fields;

// #[derive(Default, OptionsBuilder, IntoCommand)]
#[derive(Default)]
pub struct LsOptions {
    // TODO: matchers can be combined using Boolean operators
    matcher: Option<Matcher>,
}

// derive OptionsBuilder

impl LsOptions {
    pub fn matcher(&mut self, matcher: Matcher) -> &Self {
        self.matcher = Some(matcher);
        self
    }
}

// derive IntoCommand
impl Into<tokio::process::Command> for &LsOptions {
    fn into(self) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new("kitty");
        cmd.args(&["@", "ls"]);
        cmd
    }
}

// async fn test() {
//     let mut cmd: tokio::process::Command = LsOptions::default().matcher(Matcher::Id(1u32)).into();
//     cmd.output().await.unwrap();
// }

// #### ------------------------------------------------
pub struct SendTextOptions {
    // TODO: matchers can be combined using Boolean operators
    matcher: Option<Matcher>,
}

// derive IntoCommand
impl Into<tokio::process::Command> for &SendTextOptions {
    fn into(self) -> tokio::process::Command {
        todo!()
    }
}

// id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
pub enum Matcher {
    Id(u32),
    Title(String),
    Pid(u32),
    Cwd(String),
    CmdLine(String),
    Num(u32),
    Env(String),
    Var(String),
    State(String),
    Neighbor(String),
    Recent(u32),
}
