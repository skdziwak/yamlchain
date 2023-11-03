use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::{
    error::Error,
    workflows::Context,
    workflows::stages::StageOutput,
    workflows::stages::StageRunner,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SplitStageInfo {
    pub data: String,
    pub delimiter: String,
    pub trim: bool,
    pub remove_empty: bool,
}

#[stage(SplitStageInfo)]
pub struct SplitStageRunner<'a> {
    template: &'a SplitStageInfo,
}

impl<'a> SplitStageRunner<'a> {
    pub fn new(template: &'a SplitStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for SplitStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let data = ctx.interpolate(&self.template.data)?;
        let mut parts: Vec<String> = data.split(&self.template.delimiter).map(String::from).collect();

        if self.template.trim {
            parts = parts.into_iter().map(|s| s.trim().to_string()).collect();
        }

        if self.template.remove_empty {
            parts = parts.into_iter().filter(|s| !s.is_empty()).collect();
        }

        Ok(StageOutput::List(parts))
    }
}
