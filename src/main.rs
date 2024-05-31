// use std::io::{BufRead};

use std::fs;

use clap::{Parser, Subcommand};

use crate::branch_manager::BranchManager;
use crate::file_stager::FileStager;
use crate::head_manager::HeadManager;
use crate::stage_list_manager::StageListManager;

mod file_stager;
mod maze_solver;
mod branch_manager;
mod stage_list_manager;
mod head_manager;

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

    construct {},

    copy {
        from: std::path::PathBuf,
        to: std::path::PathBuf,
    },
}

fn main() {
    // TODO: Create new branch
    // TODO: Checkout the branch and updates the project folder
    let cli: CLI = CLI::parse();
    let branch_manager: BranchManager = BranchManager::new();
    let head_manager = HeadManager::new(&branch_manager);

    match &cli.command {
        Some(Commands::init {}) => branch_manager.init_repository("main".to_string()),
        Some(Commands::add { path }) => {
            let mut file_stager = FileStager::new(&branch_manager, &head_manager);
            file_stager.stage(path).unwrap()
        }
        // TODO: Remove this command later
        Some(Commands::copy { from, to }) => {
            println!("Copy from: {:?} to {:?}", from, to);
            fs::copy(from, to).unwrap();
        }
        Some(Commands::construct {}) => {
            let head_manager = HeadManager::new(&branch_manager);
            head_manager.construct_head();
        }

        Some(Commands::checkout { branch }) => {
            let head_manager = HeadManager::new(&branch_manager);
            head_manager.checkout(branch);
        }
        Some(Commands::commit { message }) => {
            let mut stage_list_manager: StageListManager = StageListManager::new(&branch_manager, &head_manager);
            stage_list_manager.consume(message);
        }
        None => (),
    }
}

