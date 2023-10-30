use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::process::{Command, Stdio};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShellScriptStage {
    pub script: String,
    pub shell: Option<String>,
    pub stdin: Option<String>,
}

#[stage(ShellScriptStage)]
pub struct ShellScriptStageRunner<'a> {
    template: &'a ShellScriptStage,
}

impl<'a> ShellScriptStageRunner<'a> {
    pub fn new(template: &'a ShellScriptStage) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for ShellScriptStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let shell = match &self.template.shell {
            Some(shell) => shell,
            None => "bash",
        };

        let script = ctx.interpolate(&self.template.script)?;
        let mut command = Command::new(shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("-c")
            .arg(script)
            .spawn().map_err(|e| Error::RuntimeError(e.to_string()))?;

        if let Some(stdin_str) = &self.template.stdin {
            let stdin_str = ctx.interpolate(stdin_str)?;
            let stdin = command.stdin.as_mut().ok_or(Error::RuntimeError("Failed to open stdin".to_string()))?;
            stdin.write_all(stdin_str.as_bytes()).map_err(|e| Error::RuntimeError(e.to_string()))?;
        }

        let stdout = command.wait_with_output().map_err(|e| Error::RuntimeError(e.to_string()))?.stdout;
        let stdout_str = String::from_utf8(stdout).map_err(|e| Error::RuntimeError(e.to_string()))?;


        Ok(StageOutput::Text(stdout_str))
    }
}
