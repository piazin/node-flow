use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,

    #[arg(default_value = "./node-flow.json", short)]
    path_config_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!(
        "command: {}, path: {:?}",
        args.command, args.path_config_file
    );
}
