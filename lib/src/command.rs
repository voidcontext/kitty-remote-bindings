pub(crate) mod focus_window;
pub(crate) mod launch;
pub(crate) mod ls;
pub mod options;
pub(crate) mod send_text;

use std::process::Output;

pub use focus_window::FocusWindow;
pub use launch::Launch;
pub use ls::Ls;
pub use send_text::SendText;

use crate::Result;

// enum RemoteCommand {
//     FocusWindow(focus_window::FocusWindow),
//     Ls(ls::Ls),
//     SendText(send_text::SendText)
// }

/// Parse and decode the output of Kitty's remote commands.
///
/// For some commands the output, [`CommandOutput::R`] is just (), for some commands it's actual data
/// like [`OsWindows`](crate::model::OsWindows) for the [Ls] command.
#[allow(clippy::module_name_repetitions)]
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
