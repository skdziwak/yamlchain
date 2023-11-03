use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoStageInfo {
    pub message: String,
}

#[stage(EchoStageInfo)]
pub struct EchoStageRunner<'a> {
    template: &'a EchoStageInfo,
}

impl<'a> EchoStageRunner<'a> {
    pub fn new(template: &'a EchoStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for EchoStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let input = ctx.interface.get_input(ctx.interpolate(&self.template.message)?).await?;
        ctx.interface.send_message(input.clone()).await?;
        Ok(StageOutput::Text(input))
    }
}
