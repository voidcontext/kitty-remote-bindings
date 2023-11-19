use std::process::Output;

use kitty_remote_bindings_macros::KittyCommand;

use crate::Result;

use super::{options::Matcher, CommandOutput};

/// Represents the "send-text" remote command: kitty @ send-text
#[derive(Debug, PartialEq, KittyCommand)]
#[kitty_command = "send-text"]
pub struct SendText {
    text: String, // TODO: this should be a non empty string
    #[top_level]
    /// Sets the `--to` top level option
    to: Option<String>,
    #[option = "match"]
    /// Sets the `--match` option
    matcher: Option<Matcher>,
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
    fn test_focus_widow_command_to() {
        let cmd = Command::from(
            &SendText::new("some-text".to_string()).to("unix:/path/to/kitty.sock".to_string()),
        );

        assert_eq!(cmd.get_program(), "kitty");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec![
                "@",
                "--to",
                "unix:/path/to/kitty.sock",
                "send-text",
                "some-text"
            ]
        );
    }

    #[test]
    fn test_send_text_command_match_id() {
        let cmd = Command::from(
            &SendText::new("some text".to_string()).matcher(Matcher::Id(WindowId(13))),
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
