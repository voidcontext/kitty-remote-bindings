pub(crate) mod ls;
pub(crate) mod options;
pub(crate) mod send_text;

use std::process::Output;

use crate::Result;

pub trait CommandOutput {
    type R;

    /// # Errors
    ///
    /// Returns an error when the output contains a non-zero exit code or the ouput cannot be decoded
    fn result(output: &Output) -> Result<Self::R>;
}
