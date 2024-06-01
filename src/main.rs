// use std::io::{BufRead};

use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};

use crate::branch_manager::BranchManager;
use crate::file_stager::FileStager;
use crate::head_manager::HeadManager;
use crate::stage_list_manager::StageListManager;

mod file_stager;
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

    branch {
        branch_name: String,
    },

    construct {},

    copy {
        from: std::path::PathBuf,
        to: std::path::PathBuf,
    },
}

fn check_valid_gup_repo<F>(f: F) where F: FnOnce() {
    if !Path::new("./.gup").exists() {
        println!("This is not a gup repository");
        return;
    }
    f();
}

fn main() {
    // TODO: Create new branch
    // TODO: Checkout the branch and updates the project folder
    let cli: CLI = CLI::parse();
    let branch_manager: BranchManager = BranchManager::new();
    let head_manager = HeadManager::new(&branch_manager);
    let mut stage_list_manager: StageListManager = StageListManager::new(&branch_manager, &head_manager);
    let mut file_stager = FileStager::new(&branch_manager, &head_manager);

    match &cli.command {
        Some(Commands::init {}) => branch_manager.init_repository("main".to_string()),
        Some(Commands::add { path }) => check_valid_gup_repo(|| file_stager.stage(path).unwrap()),
        Some(Commands::branch { branch_name }) => check_valid_gup_repo(|| branch_manager.create_new_branch(branch_name)),
        // TODO: Remove this command later
        Some(Commands::copy { from, to }) => {
            println!("Copy from: {:?} to {:?}", from, to);
            fs::copy(from, to).unwrap();
        }
        Some(Commands::construct {}) => check_valid_gup_repo(|| head_manager.construct_head()),

        Some(Commands::checkout { branch }) => check_valid_gup_repo(|| head_manager.checkout(branch)),
        Some(Commands::commit { message }) => check_valid_gup_repo(|| stage_list_manager.consume(message)),
        None => (),
    }
}

