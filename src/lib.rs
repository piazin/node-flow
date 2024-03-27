use std::process::Command;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeFlowConfigFileJson {
    pub branch_name: String,
    pub package_manager: String,
    pub commands: Vec<String>,
    start_command: String,
}

pub fn read_config_file(path_config_file: &std::path::Path) -> Result<NodeFlowConfigFileJson> {
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

pub fn pull_last_commit(branch_name: &str) -> Result<(bool, String)> {
    let mut have_new_commit: bool = true;

    let output_git_status = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(&branch_name)
        .output()
        .expect("Erro to exec `git status`");

    let biding = String::from_utf8_lossy(&output_git_status.stdout);
    let stdout = biding.trim();

    if stdout.contains("Already up to date.") {
        have_new_commit = false;
    }

    if have_new_commit {}
    let sha = get_sha(branch_name)?;

    Ok((have_new_commit, sha))
}

pub fn get_sha(branch_name: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg(branch_name)
        .output()
        .expect("Error to exec `git log`");

    let biding = String::from_utf8_lossy(&output.stdout);
    let stdout = biding.trim().to_string();

    Ok(stdout)
}
