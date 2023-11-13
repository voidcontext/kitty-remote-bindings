use std::{io, process::Output};

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use tokio::process::Command;

#[cfg_attr(test, automock)]
#[async_trait]
pub(crate) trait Executor {
    async fn output(&self, command: &mut Command) -> io::Result<Output>;
}

pub(crate) struct TokioExecutor;

#[async_trait]
impl Executor for TokioExecutor {
    async fn output(&self, command: &mut Command) -> io::Result<Output> {
        command.output().await
    }
}
