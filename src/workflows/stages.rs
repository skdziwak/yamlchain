use async_trait::async_trait;

use crate::error::Error;
pub use crate::generated::get_runner;

use super::Context;

#[derive(Debug, Clone)]
pub enum StageOutput {
    Text(String),
    List(Vec<String>),
    None,
}

#[async_trait]
pub trait StageRunner: Send + Sync {
    async fn run<'a>(&self, ctx: &Context<'a>) -> Result<StageOutput, Error>;
}

pub mod user_input;
pub mod ai_processing;
pub mod ai_reshape;
pub mod print;
pub mod shell;
pub mod to_json;
pub mod for_each;
pub mod feedback_loop;
pub mod until;
pub mod python;
pub mod if_else;
pub mod save_file;
pub mod load_file;
pub mod set;
pub mod shell_script;
pub mod warn;
pub mod split;
pub mod echo;
