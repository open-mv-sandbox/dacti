mod add;
mod create;

use clap::{Parser, Subcommand};
use tracing::{event, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let args = CliArgs::parse();

    let result = match args.command {
        Command::Create(c) => create::run(c),
        Command::Add(c) => add::run(c),
    };

    if let Err(error) = result {
        event!(Level::ERROR, "failed:\n{:?}", error);
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
