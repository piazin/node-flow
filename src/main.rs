use anyhow::{Ok, Result};
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    command: String,

    #[arg(default_value = "./node-flow.json", short)]
    path_config_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let configs = node_flow::read_config_file(&args.path_config_file)?;

    let parsed_commands = node_flow::resolve_placeholder_in_commands(
        &configs.commands,
        "{pm}",
        &configs.package_manager,
    )?;

    println!("parsed commands > {:?}", parsed_commands);

    let output = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg(configs.branch_name)
        .output()
        .expect("Erro to exec `git status`");

    println!("{:?}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
