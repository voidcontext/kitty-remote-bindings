use crate::{CommandOutput, LaunchType, Matcher, Result};
use kitty_remote_bindings_macros::KittyCommand;
use std::process::Output;

use super::options::Cwd;

/// Represents the "launch" remote command: kitty @ launch
#[derive(Debug, PartialEq, KittyCommand)]
#[kitty_command = "launch"]
pub struct Launch {
    #[top_level]
    to: Option<String>,
    #[option = "match"]
    matcher: Option<Matcher>,
    #[option = "type"]
    launch_type: Option<LaunchType>,
    cwd: Option<Cwd>,
    args: Vec<String>,
}

impl CommandOutput for Launch {
    type R = ();

    fn result(output: &Output) -> Result<Self::R> {
        if output.status.success() {
            Ok(())
        } else {
            Err(crate::Error::ErrorExit("kitty @ focus-window".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use crate::remote_command::options::Cwd;

    use super::Launch;

    #[test]
    fn test_launch_command() {
        let cmd = Launch::new(vec!["program".to_string(), "arg1".to_string()])
            .launch_type(crate::LaunchType::OsWindow)
            .cwd(Cwd::Path(PathBuf::from("/home/user")));

        let std_cmd = std::process::Command::from(&cmd);

        assert_eq!(
            std_cmd.get_args().collect::<Vec<_>>(),
            vec![
                "@",
                "launch",
                "--type",
                "os-window",
                "--cwd",
                "/home/user",
                "program",
                "arg1"
            ]
        );
    }
}
