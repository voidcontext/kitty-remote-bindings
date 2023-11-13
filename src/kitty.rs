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
    #[must_use]
    pub fn new() -> Self {
        Self {
            executor: Box::new(TokioExecutor),
        }
    }
}

impl Default for Kitty {
    fn default() -> Self {
        Self::new()
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

        self.executor.output(&mut cmd).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::process::{ExitStatus, Output};

    use super::Kitty;
    use super::KittyTerminal;
    use crate::remote_command::LsOptions;
    use crate::remote_command::Matcher;
    use crate::remote_command::SendTextOptions;
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

    #[tokio::test]
    async fn test_send_text() {
        let mut executor = MockExecutor::new();
        executor
            .expect_output()
            .withf(|cmd| {
                format!("{:?}", cmd.as_std())
                    == r#""kitty" "@" "send-text" "--match" "id:1" "some text""#
            })
            .times(1)
            .returning(|_| {
                Ok(Output {
                    status: ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            });

        let kitty = Kitty {
            executor: Box::new(executor),
        };

        let mut options = SendTextOptions::default();
        options.matcher(Matcher::Id(1));

        kitty
            .send_text(&options, &["some text"])
            .await
            .expect("send_text() returned an error");
    }
}
