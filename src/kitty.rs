use async_trait::async_trait;

use crate::{
    executor::{Executor, TokioExecutor},
    model::LsOutput,
    remote_command::{LsOptions, SendTextOptions},
    KittyTerminal, Result,
};

pub struct Kitty {
    executor: Box<dyn Executor + Send + Sync + 'static>,
}

impl Kitty {
    pub fn new() -> Self {
        Self {
            executor: Box::new(TokioExecutor),
        }
    }
}

#[async_trait]
impl KittyTerminal for Kitty {
    async fn ls(&self, options: &LsOptions) -> Result<LsOutput> {
        let mut cmd: tokio::process::Command = options.into();
        let output = self.executor.output(&mut cmd).await?;
        let ls_output = serde_json::from_slice::<LsOutput>(&output.stdout)?;

        Ok(ls_output)
    }
    async fn send_text(&self, options: &SendTextOptions, args: &[&str]) -> Result<()> {
        let mut cmd: tokio::process::Command = options.into();
        cmd.args(args);

        cmd.output().await.unwrap();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::process::{ExitStatus, Output};

    use super::Kitty;
    use super::KittyTerminal;
    use crate::remote_command::LsOptions;
    use crate::{executor::MockExecutor, model::test_fixture};
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_ls() {
        let mut executor = MockExecutor::new();
        executor
            .expect_output()
            .withf(|cmd| format!("{:?}", cmd.as_std()) == r#""kitty" "@" "ls""#)
            .times(1)
            .returning(|_| {
                Ok(Output {
                    status: ExitStatus::default(),
                    stdout: test_fixture::LS_OUTPUT_JSON.as_bytes().to_vec(),
                    stderr: Vec::new(),
                })
            });

        let kitty = Kitty {
            executor: Box::new(executor),
        };

        let result = kitty
            .ls(&LsOptions::default())
            .await
            .expect("ls() returned an error");

        assert_eq!(result, *test_fixture::LS_OUTPUT);
    }
}