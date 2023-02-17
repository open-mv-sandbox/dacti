use clap::Parser;

fn main() {
    let _args = Args::parse();
}

/// dacti-pack CLI utility tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}
