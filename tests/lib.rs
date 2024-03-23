use anyhow::Result;

#[test]
fn resolve_placeholder_in_commands_test() -> Result<()> {
    let placeholder: &str = "{pm}";
    let value_to_replace: &str = "pnpm";
    let commands: Vec<String> = vec![String::from("{pm} install"), String::from("{pm} run build")];
    let expected_commands = vec!["pnpm install", "pnpm run build"];

    let result =
        node_flow::resolve_placeholder_in_commands(&commands, &placeholder, &value_to_replace)?;

    assert_eq!(result, expected_commands);

    Ok(())
}
