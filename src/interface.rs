use async_trait::async_trait;
use crate::error::Error;

pub mod cli;
pub mod vim;

#[async_trait]
pub trait Interface: Send + Sync {
    async fn send_message(&self, msg: String) -> Result<(), Error>;
    async fn get_input(&self, msg: String) -> Result<String, Error>;
}

impl<'a> std::fmt::Debug for &'a dyn Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}

