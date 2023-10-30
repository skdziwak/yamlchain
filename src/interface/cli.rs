use async_trait::async_trait;

use crate::error::Error;

use super::Interface;


pub struct CliInterface;

impl CliInterface {
    pub fn new() -> CliInterface {
        log::info!("Creating CLI interface");
        CliInterface
    }
}

#[async_trait]
impl Interface for CliInterface {
    async fn send_message(&self, msg: String) -> Result<(), Error> {
        println!("{}", msg);
        Ok(())
    }
    async fn get_input(&self, msg: String) -> Result<String, Error> {
        let mut input = String::new();
        println!("{}", msg);
        std::io::stdin().read_line(&mut input).map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(input)
    }
}
