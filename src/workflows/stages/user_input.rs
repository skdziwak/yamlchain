use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserInputStageInfo {
    pub message: String,
}

#[stage(UserInputStageInfo)]
pub struct UserInputStageRunner<'a> {
    template: &'a UserInputStageInfo,
}

impl<'a> UserInputStageRunner<'a> {
    pub fn new(template: &'a UserInputStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for UserInputStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let input = ctx.interface.get_input(ctx.interpolate(&self.template.message)?).await?;
        Ok(StageOutput::Text(input))
    }
}
