//! > **Kitty remote command bindings for rust**
//!
//! This crate provides access to the Kitty terminal's remote control functionality. At the moment !
//! this is achieved by creating `std::process::Command` objects through a convenient and type safe
//! API interface.
//!
//! ## Examples:
//!
//! Send text to Window 1
//! ```
//! use std::process::Command;
//!
//! use kitty_remote_bindings::{command::options::Matcher, command::SendText, model::WindowId};
//!
//! let mut send_text = SendText::new(r#"echo "Hello world""#.to_string())
//!     .matcher(Matcher::Id(WindowId(2)));
//!
//! let cmd = Command::from(&send_text);
//!
//! // then run command:
//! //
//! // cmd.status().expect("failed to execute send-text");
//! ```

pub mod command;
pub mod model;

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
