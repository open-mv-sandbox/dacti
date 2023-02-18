mod add;
mod create;

use clap::{Parser, Subcommand};

fn main() {
    let args = CliArgs::parse();

    let result = match args.command {
        Command::Create(c) => create::run(c),
        Command::Add(c) => add::run(c),
    };

    if let Err(error) = result {
        println!("failed:\n{:?}", error);
        std::process::exit(1);
    }
}

/// Pterodactil CLI toolkit for working with dacti packages.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Create(create::CreateCommand),
    Add(add::AddCommand),
}
