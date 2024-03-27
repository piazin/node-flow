use anyhow::{Ok, Result};
use clap::Parser;
use node_flow::{pull_last_commit, read_config_file, resolve_placeholder_in_commands};

#[derive(Parser)]
struct Cli {
    command: String,

    #[arg(default_value = "./node-flow.json", short)]
    path_config_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let configs = read_config_file(&args.path_config_file)?;

    let parsed_commands =
        resolve_placeholder_in_commands(&configs.commands, "{pm}", &configs.package_manager)?;

    println!("parsed commands > {:?}", parsed_commands);

    let (have_new_commit, sha) = pull_last_commit(&configs.branch_name)?;
    println!("have new commit -> {}", have_new_commit);
    println!("sha -> {}", sha);

    Ok(())
}
