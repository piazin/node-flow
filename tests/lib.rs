use std::path::Path;

use anyhow::{Ok, Result};
use assert_fs::prelude::*;
use node_flow::{get_sha, read_config_file, resolve_placeholder_in_commands};

#[test]
fn resolve_placeholder_in_commands_test() -> Result<()> {
    let placeholder: &str = "{pm}";
    let value_to_replace: &str = "pnpm";
    let commands: Vec<String> = vec![String::from("{pm} install"), String::from("{pm} run build")];
    let expected_commands = vec!["pnpm install", "pnpm run build"];

    let result = resolve_placeholder_in_commands(&commands, &placeholder, &value_to_replace)?;

    assert_eq!(result, expected_commands);

    Ok(())
}

#[test]
fn read_config_file_test() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("node-flow.json")?;
    file.write_file(Path::new("./tests/node-flow-test.json"))?;

    let config_file = read_config_file(file.path())?;

    assert_eq!(config_file.branch_name, "main");
    assert_eq!(config_file.package_manager, "pnpm");

    Ok(())
}

#[test]
fn get_sha_test() -> Result<()> {
    let branch_name = "main";

    let sha = get_sha(branch_name)?;

    assert_eq!(sha.len(), 40);

    Ok(())
}
