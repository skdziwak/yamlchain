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
pub struct AiReshapeStageInfo {
    pub target: AiReshapeTarget,
    pub data: String,
    pub model: Model
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AiReshapeTarget {
    List
}

use super::{StageOutput, StageRunner};

#[stage(AiReshapeStageInfo)]
pub struct AiReshapeStageRunner<'a> {
    template: &'a AiReshapeStageInfo,
}

impl<'a> AiReshapeStageRunner<'a> {
    pub fn new(template: &'a AiReshapeStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for AiReshapeStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let message = match self.template.target {
            AiReshapeTarget::List => 
                r#"Your task is to reshape data provided by the user into a JSON list of strings. You only output JSON.
                Example: [
                    "a",
                    "b",
                ]"#,
        };
        let result = llm::call_openai(
            vec![
                Message::system("Your task is to transform the data provided by the user into a JSON. You only output JSON."),
                Message::user(ctx.interpolate(&self.template.data)?),
                Message::system(message),
                Message::user("Output:"),
            ],
            self.template.model.name(),
        )
        .await?;
        let text = match result {
            Response { text } => text,
        };
        let output = match self.template.target {
            AiReshapeTarget::List => StageOutput::List(text_to_json_list(text)?),
        };
        Ok(output)
    }
}

fn text_to_json_list(text: String) -> Result<Vec<String>, Error> {
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| Error::RuntimeError(e.to_string()))?;
    match value {
        serde_json::Value::Array(vec) => {
            let mut result = vec![];
            for value in vec {
                match value {
                    serde_json::Value::String(s) => result.push(s),
                    _ => return Err(Error::RuntimeError(format!("Expected a JSON array of strings, got {}", text))),
                }
            }
            Ok(result)
        }
        _ => return Err(Error::RuntimeError(format!("Expected a JSON array of strings, got {}", text))),
    }
}
