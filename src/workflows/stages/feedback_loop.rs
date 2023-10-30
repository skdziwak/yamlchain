use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context, llm::{Message, Response}, schema::Model};
use crate::llm;
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeedbackLoopStageInfo {
    pub initial_input: String,
    pub info_message: String,
    pub model: Model,
}

#[stage(FeedbackLoopStageInfo)]
pub struct FeedbackLoopStageRunner<'a> {
    template: &'a FeedbackLoopStageInfo,
}

impl<'a> FeedbackLoopStageRunner<'a> {
    pub fn new(template: &'a FeedbackLoopStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for FeedbackLoopStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let mut current_input = ctx.interpolate(&self.template.initial_input)?;

        loop {
            let info_message = ctx.interpolate(&self.template.info_message)?;
            let message = format!(r#"
            {current_input}
            If you are satisfied, respond with 'ok'. Otherwise, describe what should be improved."#);
            let feedback = ctx.interface.get_input(message).await?;

            if feedback.trim().to_lowercase() == "ok" {
                break;
            }

            let result = llm::call_openai(
                vec![
                    Message::system(info_message),
                    Message::system("This is the current state:"),
                    Message::user(current_input),
                    Message::system("Improve the following based on feedback:"),
                    Message::user(feedback),
                    Message::system("You only output improved state without any other text."),
                ],
                &self.template.model.name(),
            ).await?;

            current_input = match result {
                Response { text } => text,
            };
        }

        Ok(StageOutput::Text(current_input))
    }
}
