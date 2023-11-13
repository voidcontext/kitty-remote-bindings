#[derive(Debug, Default, PartialEq)]
pub struct LsOptions {
    // TODO: matchers can be combined using Boolean operators
    matcher: Option<Matcher>,
}

impl LsOptions {
    pub fn matcher(&mut self, matcher: Matcher) -> &Self {
        self.matcher = Some(matcher);
        self
    }
}

impl From<&LsOptions> for tokio::process::Command {
    fn from(value: &LsOptions) -> Self {
        let mut cmd = tokio::process::Command::new("kitty");
        cmd.args(["@", "ls"]);

        if let Some(Matcher::Id(id)) = value.matcher {
            cmd.arg("--match");
            cmd.arg(format!("id:{id}"));
        }

        cmd
    }
}

#[derive(Debug, Default, PartialEq)]
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

impl From<&SendTextOptions> for tokio::process::Command {
    fn from(value: &SendTextOptions) -> Self {
        let mut cmd = tokio::process::Command::new("kitty");
        cmd.args(["@", "send-text"]);

        if let Some(Matcher::Id(id)) = value.matcher {
            cmd.arg("--match");
            cmd.arg(format!("id:{id}"));
        }

        cmd
    }
}

// id, title, pid, cwd, cmdline, num, env, var, state, neighbor, and recent
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tokio::process::Command;

    use crate::{remote_command::Matcher, LsOptions, SendTextOptions};

    #[test]
    fn test_ls_options() {
        let cmd = Command::from(&LsOptions::default());
        let cmd = cmd.as_std();

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(cmd.get_args().collect::<Vec<_>>(), vec!["@", "ls"]);
    }

    #[test]
    fn test_ls_options_match_id() {
        let cmd = Command::from(LsOptions::default().matcher(Matcher::Id(13)));
        let cmd = cmd.as_std();

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "ls", "--match", "id:13"]
        );
    }

    #[test]
    fn test_send_text_options() {
        let cmd = Command::from(&SendTextOptions::default());
        let cmd = cmd.as_std();

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(cmd.get_args().collect::<Vec<_>>(), vec!["@", "send-text"]);
    }

    #[test]
    fn test_send_text_match_id() {
        let cmd = Command::from(SendTextOptions::default().matcher(Matcher::Id(13)));
        let cmd = cmd.as_std();

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "send-text", "--match", "id:13"]
        );
    }
}
