mod executor;
mod kitty;
mod kitty_terminal;
pub mod model;
mod remote_command;

pub use kitty_terminal::KittyTerminal;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("JsonDecoding error")]
    JsonDecodingError(#[from] serde_json::Error),
}
