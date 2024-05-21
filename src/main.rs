use std::fs::{File, metadata};
use std::io::{BufRead, BufReader};

use clap::{Error, Parser, Subcommand};

use crate::branch_manager::BranchManager;
use crate::gup_add::FileStager;

mod gup_init;
mod gup_add;
mod maze_solver;
mod branch_manager;

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
        path: std::path::PathBuf,
    },

    checkout {
        branch: String,
    },
}

fn main() {
    let cli = CLI::parse();
    let branch_manager = BranchManager::new();

    match &cli.command {
        Some(Commands::init {}) => branch_manager.init_repository("main".to_string()),
        Some(Commands::add { path }) => {
            let mut file_stager = FileStager::new(branch_manager);
            file_stager.stage(path).unwrap()
        }
        Some(Commands::checkout { branch }) => branch_manager.checkout(branch),
        None => (),
    }
}

