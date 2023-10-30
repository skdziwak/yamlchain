use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::{fs::File, path::Path};
use std::io::Write;
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SaveFileStageInfo {
    pub path: String,
    pub content: String,
}

#[stage(SaveFileStageInfo)]
pub struct SaveFileStageRunner<'a> {
    template: &'a SaveFileStageInfo,
}

impl<'a> SaveFileStageRunner<'a> {
    pub fn new(template: &'a SaveFileStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for SaveFileStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let path = ctx.interpolate(&self.template.path)?;
        let content = ctx.interpolate(&self.template.content)?;
        let workdir: &Path = ctx.workdir;
        let mut file = File::create(workdir.join(path)).map_err(|e| Error::RuntimeError(e.to_string()))?;
        file.write_all(content.as_bytes()).map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(StageOutput::None)
    }
}
