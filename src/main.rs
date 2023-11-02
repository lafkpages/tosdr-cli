use clap::Parser;

use tosdr_cli::cli;

mod commands;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::CliCommands::Search { args, json } => {
            commands::search::main(&args, &json);
        }
    }
}
