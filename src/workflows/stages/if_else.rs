use std::collections::HashMap;

use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context, schema::WorkflowStageData};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IfElseStageInfo {
    pub left: String,
    pub right: String,
    pub if_stages: Vec<WorkflowStageData>,
    pub else_stages: Vec<WorkflowStageData>,
}

#[stage(IfElseStageInfo)]
pub struct IfElseStageRunner<'a> {
    template: &'a IfElseStageInfo,
}

impl<'a> IfElseStageRunner<'a> {
    pub fn new(template: &'a IfElseStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for IfElseStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let left_value = ctx.interpolate(&self.template.left)?.trim().to_string();
        let right_value = ctx.interpolate(&self.template.right)?.trim().to_string();

        let stages = if left_value == right_value {
            &self.template.if_stages
        } else {
            &self.template.else_stages
        };

        let mut variables: HashMap<String, StageOutput> = (*ctx.variables).clone();
        let mut last_output = StageOutput::None;
        
        for stage in stages {
            log::info!("Running stage in a condition: {}", stage.name);
            let runner = super::get_runner(stage);
            let output = {
                let condition_ctx = ctx.derive(&variables);
                runner.run(&condition_ctx).await?
            };
            log::info!("Stage {} finished", stage.name);
            log::debug!("Stage {} output: {:?}", stage.name, output);
            variables.insert(stage.name.clone(), output.clone());
            last_output = output;
        }
        log::debug!("Last output: {:?}", last_output);
        Ok(last_output)
    }
}
