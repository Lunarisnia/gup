// use std::io::{BufRead};

use std::fs;
use clap::{Parser, Subcommand};

use crate::branch_manager::BranchManager;
use crate::file_stager::FileStager;
use crate::stage_list_manager::StageListManager;

mod file_stager;
mod maze_solver;
mod branch_manager;
mod stage_list_manager;

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

    commit {
        message: String,
    },

    copy {
        from: std::path::PathBuf,
        to: std::path::PathBuf,
    },
}

fn main() {
    let cli: CLI = CLI::parse();
    let branch_manager: BranchManager = BranchManager::new();

    match &cli.command {
        Some(Commands::init {}) => branch_manager.init_repository("main".to_string()),
        Some(Commands::add { path }) => {
            let mut file_stager = FileStager::new(branch_manager);
            file_stager.stage(path).unwrap()
        }
        Some(Commands::copy { from, to }) => {
            println!("Copy from: {:?} to {:?}", from, to);
            fs::copy(from, to).unwrap();
        }
        Some(Commands::checkout { branch }) => branch_manager.checkout(branch),
        Some(Commands::commit { message }) => {
            let mut stage_list_manager: StageListManager = StageListManager::new(branch_manager.clone());
            stage_list_manager.consume();
        }
        None => (),
    }
}

