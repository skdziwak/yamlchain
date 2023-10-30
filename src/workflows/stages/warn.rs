use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LogWarnStage {
    pub message: String,
}

#[stage(LogWarnStage)]
pub struct LogWarnStageRunner<'a> {
    template: &'a LogWarnStage,
}

impl<'a> LogWarnStageRunner<'a> {
    pub fn new(template: &'a LogWarnStage) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for LogWarnStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let message = ctx.interpolate(&self.template.message)?;
        log::warn!("{}", message);
        Ok(StageOutput::None)
    }
}
