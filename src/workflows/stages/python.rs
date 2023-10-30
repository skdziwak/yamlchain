use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::process::{Command, Output};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PythonScriptStageInfo {
    pub script: String,
}

#[stage(PythonScriptStageInfo)]
pub struct PythonScriptStageRunner<'a> {
    template: &'a PythonScriptStageInfo,
}

impl<'a> PythonScriptStageRunner<'a> {
    pub fn new(template: &'a PythonScriptStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for PythonScriptStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let script = &self.template.script;        
        let script = ctx.interpolate(script)?;
        let output: Output = Command::new("python")
            .arg("-c")
            .arg(script)
            .current_dir(ctx.workdir)
            .output()
            .map_err(|e| Error::RuntimeError(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::RuntimeError(format!("Script failed with error: {:?}", output.stderr)));
        }

        let stdout = String::from_utf8(output.stdout).map_err(|_| Error::RuntimeError("Failed to decode stdout".to_string()))?;

        Ok(StageOutput::Text(stdout))
    }
}
