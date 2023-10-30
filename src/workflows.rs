use std::{collections::HashMap, path::Path};
use crate::{error::Error, schema::Workflow, interface::Interface};
use stages::StageOutput;
use regex::Regex;

pub mod stages;

#[derive(Debug)]
pub struct Context<'a> {
    pub variables: &'a HashMap<String, StageOutput>,
    pub interface: &'a dyn Interface,
    pub workdir: &'a std::path::Path,
}

impl<'a> Context<'a> {

    pub fn derive<'b>(&'a self, variables: &'b HashMap<String, StageOutput>) -> Context<'b> where 'a: 'b {
        Context {
            variables,
            interface: self.interface,
            workdir: self.workdir,
        }
    }

    pub fn interpolate<S: Into<String>>(&self, s: S) -> Result<String, Error> {
        let s = s.into();
        let re = Regex::new(r"\$\{([^}]+)\}").unwrap();
        
        let result = re.replace_all(&s, |caps: &regex::Captures| {
            let var_name = &caps[1];
            match self.variables.get(var_name) {
                Some(StageOutput::Text(value)) => value.to_string(),
                Some(StageOutput::List(vec)) => vec.join("\n"),
                Some(StageOutput::None) => return "".to_string(),
                None => panic!("Variable not found: {}", var_name),
            }
        });

        Ok(result.to_string())
    }
    
    pub fn get_variable(&self, var_name: &str) -> Result<&StageOutput, Error> {
        self.variables.get(var_name).ok_or(Error::VariableNotFound(var_name.to_string()))
    }
}

pub async fn run_workflow(workflow: &Workflow, interface: &'_ dyn Interface, workdir: &Path) -> Result<HashMap<String, StageOutput>, Error> {
    let mut variables = HashMap::new();

    log::info!("Running workflow {}", workflow.name);
    for stage in &workflow.stages {
        log::info!("Running stage {}", stage.name);
        let runner = stages::get_runner(&stage);
        let output = runner.run(&Context { variables: &variables, interface, workdir }).await?;
        log::info!("Stage {} finished", stage.name);
        log::debug!("Stage {} output: {:?}", stage.name, output);
        variables.insert(stage.name.clone(), output);
    }

    Ok(variables)
}
