use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeFlowConfigFileJson {
    pub branch_name: String,
    pub package_manager: String,
    pub commands: Vec<String>,
    start_command: String,
}

pub fn read_config_file(path_config_file: &std::path::PathBuf) -> Result<NodeFlowConfigFileJson> {
    let file_content = std::fs::read_to_string(&path_config_file)
        .with_context(|| format!("Failed to read config file `{:?}`", &path_config_file))?;
    let configs: NodeFlowConfigFileJson = serde_json::from_str(&file_content)
        .with_context(|| format!("Failed to parse JSON in file `{:?}`", &path_config_file))?;
    Ok(configs)
}

pub fn resolve_placeholder_in_commands(
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
