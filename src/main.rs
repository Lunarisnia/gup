use std::env::current_dir;
use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Init the folder as a gup repository
    init {}
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::init {}) => {
            match fs::create_dir("./.gup") {
                Ok(()) => (),
                Err(e) => println!("{e}")
            };
        }
        None => (),
    }
}
