use anyhow::{Context, Ok, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    command: String,

    #[arg(default_value = "./node-flow.json", short)]
    path_config_file: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct NodeFlowConfigFileJson {
    branch_name: String,
    package_manager: String,
    commands: Vec<String>,
    start_command: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!(
        "command: {}, path: {:?}",
        args.command, args.path_config_file
    );

    let configs = read_config_file(&args.path_config_file)?;

    let parsed_commands =
        resolve_placeholder_in_commands(&configs.commands, "{pm}", &configs.package_manager)?;

    println!("config file > {:?}", configs);
    println!("parsed commands > {:?}", parsed_commands);

    Ok(())
}

fn read_config_file(path_config_file: &std::path::PathBuf) -> Result<NodeFlowConfigFileJson> {
    let file_content = std::fs::read_to_string(&path_config_file)
        .with_context(|| format!("Failed to read config file `{:?}`", &path_config_file))?;
    let configs: NodeFlowConfigFileJson = serde_json::from_str(&file_content)
        .with_context(|| format!("Failed to parse JSON in file `{:?}`", &path_config_file))?;
    Ok(configs)
}

fn resolve_placeholder_in_commands(
    commands: &[String], // &Vec<T> você está passando uma ref para todo o vetor, já &[T] apenas uma fatia
    placeholder: &str,
    value: &str,
) -> Result<Vec<String>> {
    let parsed_commands: Vec<String> = commands
        .iter()
        .map(|c| c.replace(&placeholder, &value))
        .collect();

    Ok(parsed_commands)
}
