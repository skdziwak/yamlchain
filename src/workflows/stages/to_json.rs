use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::llm;
use crate::schema::Model;
use crate::{
    error::Error,
    llm::{Message, Response},
    workflows::Context,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToJsonStageInfo {
    pub data: String,
    pub model: Model,
    pub example: String,
}

use super::{StageOutput, StageRunner};

#[stage(ToJsonStageInfo)]
pub struct ToJsonStageRunner<'a> {
    template: &'a ToJsonStageInfo,
}

impl<'a> ToJsonStageRunner<'a> {
    pub fn new(template: &'a ToJsonStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for ToJsonStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let result = llm::call_openai(
            vec![
                Message::system("Your task is to transform the data provided by the user into a JSON. You only output JSON."),
                Message::user(ctx.interpolate(&self.template.data)?),
                Message::system("Here's an example of what I want:"),
                Message::user(ctx.interpolate(&self.template.example)?),
                Message::user("Output:"),
            ],
            self.template.model.name(),
        )
        .await?;
        
        let text = match result {
            Response { text } => text,
        };
        let json_output = text_to_json(text)?;
        Ok(StageOutput::Text(json_output))
    }
}

fn text_to_json(text: String) -> Result<String, Error> {
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| Error::RuntimeError(e.to_string()))?;
    serde_json::to_string(&value).map_err(|e| Error::RuntimeError(e.to_string()))
}
