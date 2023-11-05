use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context, schema::WorkflowStageData};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TryStageInfo {
    pub stages: Vec<WorkflowStageData>,
    pub ok_result: String,
    pub error_result: String,
}

#[stage(TryStageInfo)]
pub struct TryStageRunner<'a> {
    template: &'a TryStageInfo,
}

impl<'a> TryStageRunner<'a> {
    pub fn new(template: &'a TryStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for TryStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let mut variables: std::collections::HashMap<String, StageOutput> = (*ctx.variables).clone();

        for stage in &self.template.stages {
            let runner = super::get_runner(stage);
            let new_ctx = ctx.derive(&variables);
            match runner.run(&new_ctx).await {
                Ok(output) => {
                    variables.insert(stage.name.clone(), output.clone());
                },
                Err(_) =>  {
                    let result = new_ctx.interpolate(&self.template.error_result)?;
                    return Ok(StageOutput::Text(result));
                }
            }
        }

        let result = ctx.derive(&variables).interpolate(&self.template.ok_result)?;
        Ok(StageOutput::Text(result))
    }
}
