use std::process::Output;

use kitty_remote_bindings_macros::KittyCommand;

use crate::Result;

use super::{options::Matcher, CommandOutput};

/// Represents the "focus-window" remote command: kitty @ focus-window
#[derive(Debug, PartialEq, KittyCommand)]
#[kitty_command = "focus-window"]
pub struct FocusWindow {
    #[top_level]
    /// Sets the `--to` top level option
    to: Option<String>,
    /// Sets the `--match` option
    #[option = "match"]
    matcher: Option<Matcher>,
}

impl CommandOutput for FocusWindow {
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

    use core::panic;
    use std::{
        os::unix::process::ExitStatusExt,
        process::{Command, ExitStatus, Output},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        command::{options::Matcher, CommandOutput},
        model::WindowId,
    };

    use super::FocusWindow;

    #[test]
    fn test_focus_window_command_default() {
        let cmd = Command::from(&FocusWindow::new());

        assert_eq!(cmd.get_program(), "kitten");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "focus-window"]
        );
    }

    #[test]
    fn test_focus_widow_command_to() {
        let cmd = Command::from(&FocusWindow::new().to("unix:/path/to/kitty.sock".to_string()));

        assert_eq!(cmd.get_program(), "kitten");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "--to", "unix:/path/to/kitty.sock", "focus-window"]
        );
    }

    #[test]
    fn test_focus_window_command_match_id() {
        let cmd = Command::from(&FocusWindow::new().matcher(Matcher::Id(WindowId(13))));

        assert_eq!(cmd.get_program(), "kitten");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "focus-window", "--match", "id:13"]
        );
    }

    #[test]
    fn test_focus_window_output_success() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: "some out put".as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        FocusWindow::result(&output).expect("Succesful output was expected");
    }

    #[test]
    fn test_focus_window_output_error() {
        let output = Output {
            status: ExitStatus::from_raw(1),
            stdout: "some out put".as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        let result = FocusWindow::result(&output);

        match result {
            Err(crate::Error::ErrorExit(process)) => assert_eq!(process, "kitty @ focus-window"),
            r => panic!("Unexpected result: {r:?}"),
        };
    }
}
