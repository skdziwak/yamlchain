use std::io::Write;

use clap::{Parser, ValueEnum};

use crate::interface::Interface;
mod error;
mod llm;
mod schema;
mod workflows;
mod interface;
mod generated;

#[derive(Parser)]
#[command(version = "1.0", author = "Szymon Dziwak <skdziwak@gmail.com>", about = "This is an application that allows you to create an AI assistant for a specific task.")]
struct Cli {
    #[arg(short, long, help = "Saves JSON schema for the workflows file")]
    schema: Option<String>,
    #[arg(short = 'f', long, help = "Path to the workflows file")]
    workflows_file: Option<String>,
    #[arg(name = "workflow_name", help = "Name of the workflow to run, if not specified and there is only one workflow in the file, it will be used")]
    name: Option<String>,
    #[arg(short, long, help = "Interface to use, if not specified, vim interface will be used.")]
    interface: Option<InterfaceSelection>,
    #[arg(short, long, help = "Working directory for the workflow, if not specified, current directory will be used.")]
    workdir: Option<String>,
    #[arg(short, long, help = "Enable debug logs")]
    debug: bool,
}

#[derive(Debug, ValueEnum, Clone)]
enum InterfaceSelection {
    Cli,
    Vim,
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {},
        Err(e) => {
            log::error!("{}", e);
        }
    }
}

async fn run() -> Result<(), String> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    let mut logger = env_logger::Builder::new();
    if cli.debug {
        logger.filter_level(log::LevelFilter::Debug);
    } else {
        logger.filter_level(log::LevelFilter::Info);
    }
    logger.init();

    if let Some(schema_path) = cli.schema {
        log::info!("Generating schema...");
        let schema = schemars::schema_for!(schema::Workflows);
        let schema_string = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        let mut file = std::fs::File::create(schema_path).map_err(|e| e.to_string())?;
        log::info!("Writing schema...");
        file.write_all(schema_string.as_bytes()).map_err(|e| e.to_string())?;
        log::info!("Schema saved!");
        return Ok(());
    }

    log::info!("Loading OpenAI token");
    llm::load_token().map_err(|e| e.to_string())?;
    log::info!("Loading workflow");
    let path = cli.workflows_file.unwrap_or("yc-workflows.yaml".to_string());
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let workflows: schema::Workflows = serde_yaml::from_reader(file).map_err(|e| e.to_string())?;
    let workdir = cli.workdir.unwrap_or(".".to_string());
    let workdir = std::path::Path::new(&workdir);

    let workflow: &schema::Workflow = if let Some(workflow_name) = cli.name {
        workflows.workflows.iter().find(|wf| wf.name == workflow_name)
            .expect(&format!("No workflow found with the name: {}", workflow_name))
    } else if workflows.workflows.len() == 1 {
        workflows.workflows.first()
            .expect("No workflows found")
    } else {
        panic!("No workflow name provided and there are multiple workflows in the file. Please provide the workflow name.")
    };
    let interface: Box<dyn Interface> = match cli.interface {
        Some(InterfaceSelection::Cli) => Box::new(interface::cli::CliInterface::new()),
        Some(InterfaceSelection::Vim) => Box::new(interface::vim::VimInterface::new()),
        None => Box::new(interface::vim::VimInterface::new()),
    };
    workflows::run_workflow(workflow, interface.as_ref(), &workdir).await.map_err(|e| e.to_string())?;
    Ok(())
}
