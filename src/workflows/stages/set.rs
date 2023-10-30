use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SetStageInfo {
    pub value: String,
}

#[stage(SetStageInfo)]
pub struct SetStageRunner<'a> {
    template: &'a SetStageInfo,
}

impl<'a> SetStageRunner<'a> {
    pub fn new(template: &'a SetStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for SetStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let parameter = ctx.interpolate(&self.template.value)?;
        Ok(StageOutput::Text(parameter))
    }
}
