use std::process::Output;

use kitty_remote_bindings_macros::KittyCommand;

use crate::{model::OsWindows, Result};

use super::{options::Matcher, CommandOutput};

/// Represents the "ls" remote command: kitty @ ls
#[derive(Debug, PartialEq, KittyCommand)]
#[kitty_command = "ls"]
pub struct Ls {
    #[top_level]
    /// Sets the `--to` top level option
    to: Option<String>,
    #[option = "match"]
    /// Sets the `--match` option
    matcher: Option<Matcher>,
}

impl CommandOutput for Ls {
    type R = OsWindows;
    fn result(output: &Output) -> Result<Self::R> {
        if output.status.success() {
            let ls_output = serde_json::from_slice::<OsWindows>(&output.stdout)?;

            Ok(ls_output)
        } else {
            Err(crate::Error::ErrorExit(format!(
                "kitty @ ls: {}",
                String::from_utf8_lossy(&output.stderr),
            )))
        }
    }
}

#[cfg(test)]
mod tests {

    use core::panic;
    use std::{
        os::unix::process::ExitStatusExt,
        process::{Command, ExitStatus, Output},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        command::{options::Matcher, CommandOutput},
        model::{test_fixture, WindowId},
    };

    use super::Ls;

    #[test]
    fn test_ls_command_default() {
        let cmd = Command::from(&Ls::new());

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(cmd.get_args().collect::<Vec<_>>(), vec!["@", "ls"]);
    }

    #[test]
    fn test_ls_command_to() {
        let cmd = Command::from(&Ls::new().to("unix:/path/to/kitty.sock".to_string()));

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "--to", "unix:/path/to/kitty.sock", "ls"]
        );
    }

    #[test]
    fn test_ls_command_match_id() {
        let cmd = Command::from(&Ls::new().matcher(Matcher::Id(WindowId(13))));

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "ls", "--match", "id:13"]
        );
    }

    #[test]
    fn test_ls_output_success() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: test_fixture::LS_OUTPUT_JSON.as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        let result = Ls::result(&output).expect("Succesful output was expected");

        assert_eq!(result, *test_fixture::LS_OUTPUT);
    }

    #[test]
    fn test_ls_output_error() {
        let output = Output {
            status: ExitStatus::from_raw(1),
            stdout: "some out put".as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        let result = Ls::result(&output);

        match result {
            Err(crate::Error::ErrorExit(process)) => assert_eq!(process, "kitty @ ls: debug info"),
            r => panic!("Unexpected result: {r:?}"),
        };
    }
}
