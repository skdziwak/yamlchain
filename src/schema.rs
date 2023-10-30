use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::generated::WorkflowStage;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Workflows {
    pub workflows: Vec<Workflow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<WorkflowStageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowStageData {
    pub name: String,
    pub description: Option<String>,
    pub stage: WorkflowStage,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Model {
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-32k")]
    Gpt4_32K,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
}

impl Model {
    pub fn name(&self) -> &'static str {
        match self {
            Model::Gpt4 => "gpt-4",
            Model::Gpt4_32K => "gpt-4-32k",
            Model::Gpt35Turbo => "gpt-3.5-turbo",
            Model::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
        }
    }
}
