use async_trait::async_trait;

use crate::{
    model::LsOutput,
    remote_command::{LsOptions, SendTextOptions},
    Result,
};

#[async_trait]
pub trait KittyTerminal {
    async fn ls(&self, options: &LsOptions) -> Result<LsOutput>;
    async fn send_text(&self, options: &SendTextOptions, args: &[&str]) -> Result<()>;
}
