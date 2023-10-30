use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use async_trait::async_trait;
use tempfile::NamedTempFile;

use crate::error::Error;

use super::Interface;

pub struct VimInterface;

impl VimInterface {
    pub fn new() -> VimInterface {
        log::info!("Creating Vim interface");
        VimInterface
    }

    fn prepare_message(&self, msg: &str) -> String {
        msg.lines().map(|line| format!("# {}", line)).collect::<Vec<String>>().join("\n")
    }
}

#[async_trait]
impl Interface for VimInterface {
    async fn send_message(&self, msg: String) -> Result<(), Error> {
        println!("{}", msg);
        Ok(())
    }

    async fn get_input(&self, msg: String) -> Result<String, Error> {
        let prepared_message = self.prepare_message(&msg);

        let mut tmp_file = NamedTempFile::new().map_err(|e| Error::RuntimeError(e.to_string()))?;
        tmp_file.write_all(prepared_message.as_bytes()).map_err(|e| Error::RuntimeError(e.to_string()))?;

        Command::new("vim")
            .arg(tmp_file.path())
            .status()
            .map_err(|e| Error::RuntimeError(e.to_string()))?;

        let mut content = String::new();
        File::open(tmp_file.path()).and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| Error::RuntimeError(e.to_string()))?;

        let result = content.lines()
            .filter(|line| !line.starts_with("#"))
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(result)
    }
}
