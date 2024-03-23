use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,

    #[arg(default_value = "./node-flow.json", short)]
    path_config_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!(
        "command: {}, path: {:?}",
        args.command, args.path_config_file
    );

    let configs = node_flow::read_config_file(&args.path_config_file)?;

    let parsed_commands = node_flow::resolve_placeholder_in_commands(
        &configs.commands,
        "{pm}",
        &configs.package_manager,
    )?;

    println!("config file > {:?}", configs);
    println!("parsed commands > {:?}", parsed_commands);

    Ok(())
}
