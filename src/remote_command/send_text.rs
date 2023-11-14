use std::process::{Command, Output};

use crate::{Matcher, MatcherOption, Result};

use super::CommandOutput;

#[derive(Debug, PartialEq)]
pub struct SendText {
    text: String, // TODO: this should be a non empty string
    matcher: Option<Matcher>,
}

impl SendText {
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text,
            matcher: None,
        }
    }
}

impl MatcherOption for SendText {
    fn matcher(&mut self, matcher: Matcher) -> &Self {
        self.matcher = Some(matcher);
        self
    }
}

impl CommandOutput for SendText {
    type R = ();

    fn result(output: &Output) -> Result<Self::R> {
        if output.status.success() {
            Ok(())
        } else {
            Err(crate::Error::ErrorExit("kitty @ send-text".to_string()))
        }
    }
}

impl<'a> From<&'a SendText> for Command {
    fn from(value: &SendText) -> Self {
        let mut cmd = Command::new("kitty");
        cmd.args(["@", "send-text"]);

        if let Some(Matcher::Id(id)) = value.matcher {
            cmd.arg("--match");
            cmd.arg(format!("id:{}", id.0));
        }

        cmd.arg(value.text.clone());

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

    use crate::{model::WindowId, remote_command::CommandOutput, Matcher, MatcherOption};

    use super::SendText;

    #[test]
    fn test_send_text_command_default() {
        let cmd = Command::from(&SendText::new("some text".to_string()));

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "send-text", "some text"]
        );
    }

    #[test]
    fn test_send_text_command_match_id() {
        let cmd = Command::from(
            SendText::new("some text".to_string()).matcher(Matcher::Id(WindowId(13))),
        );

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec!["@", "send-text", "--match", "id:13", "some text"]
        );
    }

    #[test]
    fn test_send_text_output_success() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: "some out put".as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        SendText::result(&output).expect("Succesful output was expected");
    }

    #[test]
    fn test_send_text_output_error() {
        let output = Output {
            status: ExitStatus::from_raw(1),
            stdout: "some out put".as_bytes().to_vec(),
            stderr: "debug info".as_bytes().to_vec(),
        };

        let result = SendText::result(&output);

        match result {
            Err(crate::Error::ErrorExit(process)) => assert_eq!(process, "kitty @ send-text"),
            r => panic!("Unexpected result: {r:?}"),
        };
    }
}
