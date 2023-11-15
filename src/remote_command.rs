pub(crate) mod focus_window;
pub(crate) mod ls;
pub(crate) mod options;
pub(crate) mod send_text;

use std::process::Output;

use crate::Result;

/// Parse and decode the output of Kitty's remote commands.
///
/// For some commands the output, [`CommandOutput::R`] is just (), for some commands it's actual data
/// like [`OsWindows`](crate::model::OsWindows) for the [Ls](crate::Ls) command.
pub trait CommandOutput {
    /// The decoded output's type
    type R;

    /// Handle the exit status, and parse/decode the standard output
    ///
    /// # Errors
    ///
    /// Returns an error when the output contains a non-zero exit code or the ouput cannot be decoded
    fn result(output: &Output) -> Result<Self::R>;
}
