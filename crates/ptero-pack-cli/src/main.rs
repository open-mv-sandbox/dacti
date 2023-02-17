mod create;

use clap::{Parser, Subcommand};

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Command::Create(c) => create::run(c),
    };

    if let Err(error) = result {
        println!("failed:\n{:?}", error);
        std::process::exit(1);
    }

    println!("completed successfully");
}

/// Pterodactil CLI toolkit for working with dacti packages.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Create(create::CreateCommand),
}
