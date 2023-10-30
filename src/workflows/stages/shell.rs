use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use std::io::Write;
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShellCommandStageInfo {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub stdin: Option<String>,
}

#[stage(ShellCommandStageInfo)]
pub struct ShellCommandStageRunner<'a> {
    template: &'a ShellCommandStageInfo,
}

impl<'a> ShellCommandStageRunner<'a> {
    pub fn new(template: &'a ShellCommandStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for ShellCommandStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let command = ctx.interpolate(&self.template.command)?;
        let args: Result<Vec<_>, _> = self.template.args.iter()
            .map(|arg| ctx.interpolate(arg))
            .collect();
        let args = args?;
        
        let mut child = Command::new(&command)
            .args(&args)
            .current_dir(ctx.workdir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| Error::RuntimeError(e.to_string()))?;

        if let Some(input) = &self.template.stdin {
            let input = ctx.interpolate(input)?;
            let mut stdin = child.stdin.take().ok_or(Error::RuntimeError("Failed to open stdin".to_string()))?;
            stdin.write_all(input.as_bytes()).map_err(|e| Error::RuntimeError(e.to_string()))?;
        }

        let output = child.wait_with_output().map_err(|e| Error::RuntimeError(e.to_string()))?;
        let stdout = String::from_utf8(output.stdout).map_err(|_| Error::RuntimeError("Failed to decode stdout".to_string()))?;

        Ok(StageOutput::Text(stdout))
    }
}
