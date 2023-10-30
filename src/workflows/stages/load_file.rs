use async_trait::async_trait;
use macros::stage;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::{fs::File, path::Path};
use std::io::Read;
use crate::{error::Error, workflows::Context};
use super::{StageRunner, StageOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LoadFileStageInfo {
    pub paths: Vec<String>,
    pub include_names: bool,
}

#[stage(LoadFileStageInfo)]
pub struct LoadFileStageRunner<'a> {
    template: &'a LoadFileStageInfo,
}

impl<'a> LoadFileStageRunner<'a> {
    pub fn new(template: &'a LoadFileStageInfo) -> Self {
        Self { template }
    }
}

#[async_trait]
impl<'a> StageRunner for LoadFileStageRunner<'a> {
    async fn run<'b>(&self, ctx: &Context<'b>) -> Result<StageOutput, Error> {
        let mut contents = Vec::new();

        for path in &self.template.paths {
            let interpolated_path = ctx.interpolate(path)?;
            let workdir: &Path = ctx.workdir;
            let mut file = File::open(workdir.join(&interpolated_path))
                .map_err(|_| Error::RuntimeError(format!("File {} does not exist", &interpolated_path)))?;
            let mut content = String::new();
            if self.template.include_names {
                content.push_str(&format!("// {}\n", &interpolated_path));
            }
            file.read_to_string(&mut content).map_err(|_| Error::RuntimeError(format!("Failed to read file {}", &interpolated_path)))?;
            contents.push(content);
        }

        Ok(StageOutput::List(contents))
    }
}
