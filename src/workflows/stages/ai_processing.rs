use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::Error, workflows::Context, llm::{Message, Response}, schema::Model};
use crate::llm;

use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AiProcessingStageInfo {
    pub prompt: String,
    pub system_message: String,
    pub model: Model,
}

#[stage(AiProcessingStageInfo)]
pub struct AiProcessingStageRunner<'a> {
    template: &'a AiProcessingStageInfo,
}

impl<'a> AiProcessingStageRunner<'a> {
    pub fn new(template: &'a AiProcessingStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for AiProcessingStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let result = llm::call_openai(
            vec![
                Message::system(ctx.interpolate(&self.template.system_message)?),
                Message::user(ctx.interpolate(&self.template.prompt)?),
            ],
            self.template.model.name(),
        ).await?;
        let text = match result {
            Response { text } => text,
        };
        Ok(StageOutput::Text(text))
    }
}
