use std::collections::HashMap;

use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::{error::Error, workflows::Context, schema::WorkflowStageData};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ForEachStageInfo {
    pub stages: Vec<WorkflowStageData>,
    pub list: String,
    pub variable: String,
}

#[stage(ForEachStageInfo)]
pub struct ForEachStageRunner<'a> {
    template: &'a ForEachStageInfo,
}

impl<'a> ForEachStageRunner<'a> {
    pub fn new(template: &'a ForEachStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for ForEachStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let variable = ctx.interpolate(&self.template.variable)?;
        let mut outputs = Vec::new();
        let list_name = ctx.interpolate(&self.template.list)?;
        let list = match ctx.get_variable(&list_name)? {
            StageOutput::List(l) => l,
            _ => return Err(Error::VariableTypeMismatch(format!("{} is not a list", list_name))),
        };
        for item in list {
            let mut variables: HashMap<String, StageOutput> = (*ctx.variables).clone();
            variables.insert(variable.clone(), StageOutput::Text(item.clone()));
            let mut last_output = StageOutput::None;
            for stage in &self.template.stages {
                log::info!("Running stage in a loop: {}", stage.name);
                let runner = super::get_runner(stage);
                let output = {
                    let loop_ctx = ctx.derive(&variables);
                    runner.run(&loop_ctx).await?
                };
                log::info!("Stage {} finished", stage.name);
                log::debug!("Stage {} output: {:?}", stage.name, output);
                variables.insert(stage.name.clone(), output.clone());
                last_output = output;
            }
            log::debug!("Last output: {:?}", last_output);
            outputs.push(match last_output {
                StageOutput::Text(s) => s,
                _ => return Err(Error::VariableTypeMismatch(format!("{} is not a text", variable))),
            });
        }

        Ok(StageOutput::List(outputs))
    }
}
