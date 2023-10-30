use std::collections::HashMap;
use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context, schema::WorkflowStageData};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UntilStageInfo {
    pub stages: Vec<WorkflowStageData>,
    pub value: String,
    pub expected_value: String,
    pub max_iterations: Option<usize>,
}

#[stage(UntilStageInfo)]
pub struct UntilStageRunner<'a> {
    template: &'a UntilStageInfo,
}

impl<'a> UntilStageRunner<'a> {
    pub fn new(template: &'a UntilStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for UntilStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let mut iterations = 0;
        let mut variables: HashMap<String, StageOutput> = (*ctx.variables).clone();
        let mut last_value: String;

        loop {
            if let Some(max) = self.template.max_iterations {
                if iterations >= max {
                    return Err(Error::MaxIterationsExceeded);
                }
            }

            for stage in &self.template.stages {
                log::info!("Running stage in the Until loop: {}", stage.name);
                let runner = super::get_runner(stage);
                let output = {
                    let loop_ctx = ctx.derive(&variables);
                    runner.run(&loop_ctx).await?
                };
                log::info!("Stage {} finished", stage.name);
                log::debug!("Stage {} output: {:?}", stage.name, output);
                variables.insert(stage.name.clone(), output);
            }

            last_value = ctx.interpolate(&self.template.value)?;
            let expected_value = ctx.interpolate(&self.template.expected_value)?;

            if last_value.trim() == expected_value.trim() {
                break;
            }

            iterations += 1;
        }

        Ok(StageOutput::Text(last_value))
    }
}
