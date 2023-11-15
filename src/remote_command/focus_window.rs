use std::process::{Command, Output};

use crate::{Matcher, MatcherExt, Result};

use super::CommandOutput;

/// Represents the "focus-window" remote command: kitty @ focus-window
#[derive(Debug, PartialEq)]
pub struct FocusWindow {
    matcher: Option<Matcher>,
}

impl FocusWindow {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { matcher: None }
    }
}

impl MatcherExt for FocusWindow {
    fn matcher(&mut self, matcher: Matcher) -> &Self {
        self.matcher = Some(matcher);
        self
    }
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

impl<'a> From<&'a FocusWindow> for Command {
    fn from(value: &FocusWindow) -> Self {
        let mut cmd = Command::new("kitty");
        cmd.args(["@", "focus-window"]);

        if let Some(Matcher::Id(id)) = value.matcher {
            cmd.arg("--match");
            cmd.arg(format!("id:{}", id.0));
        }

        cmd
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

    use crate::{model::WindowId, remote_command::CommandOutput, Matcher, MatcherExt};

    use super::FocusWindow;

    #[test]
    fn test_focus_window_command_default() {
        let cmd = Command::from(&FocusWindow::new());

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "focus-window"]
        );
    }

    #[test]
    fn test_focus_window_command_match_id() {
        let cmd = Command::from(FocusWindow::new().matcher(Matcher::Id(WindowId(13))));

        assert_eq!(cmd.get_program(), "kitty");
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
