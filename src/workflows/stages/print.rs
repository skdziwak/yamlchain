use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::{error::Error, workflows::Context};

use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrintStageInfo {
    pub output: String,
}

#[stage(PrintStageInfo)]
pub struct PrintStageRunner {
    message: String,
}

impl PrintStageRunner {
    pub fn new(template: &PrintStageInfo) -> Self {
        Self { message: template.output.clone() }
    }
}

#[async_trait]
impl StageRunner for PrintStageRunner {
    async fn run<'a>(&self, ctx: &Context<'a>) -> Result<StageOutput, Error> {
        ctx.interface.send_message(ctx.interpolate(&self.message)?).await?;
        Ok(StageOutput::None)
    }
}
