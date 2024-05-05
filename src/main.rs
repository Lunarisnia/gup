mod gup_init;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    // Init the folder as a gup repository
    init {},

    add {
        path: String,
    },
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::init {}) => gup_init::init_repository("main".to_string()),
        Some(Commands::add { path }) => {
            if path == "." {
                println!("Adding the entire thing");
            } else {
                println!("Adding Individual files not working yet");
            }
        }
        None => (),
    }
}
