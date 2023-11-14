pub mod model;
mod remote_command;

pub use remote_command::CommandOutput;

pub use remote_command::options::Matcher;
pub use remote_command::options::MatcherOption;

pub use remote_command::ls::Ls;
pub use remote_command::send_text::SendText;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("JsonDecoding error")]
    JsonDecodingError(#[from] serde_json::Error),
    #[error("The sub-process `{0}` exited with a non zero status")]
    ErrorExit(String),
}
