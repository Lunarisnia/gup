use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Write;
use std::path::{Path, PathBuf};

// This will keep track of what branch currently active, all branches list, active head
#[derive(Clone)]
pub struct BranchManager {
    pub active_branch: String,
    pub branch_list: Vec<String>,
}

impl BranchManager {
    pub fn new() -> BranchManager {
        let mut branch_manager = BranchManager {
            active_branch: String::from("main"),
            branch_list: Vec::new(),
        };
        if Path::new("./.gup").exists() {
            branch_manager.fetch_branch_list();
            branch_manager.fetch_active_branch()
        }

        branch_manager
    }

    pub fn create_new_branch(&self, new_branch_name: &String) {
        // 1. Create commit and head folder for this new branch
        // 2. Copy the entire commit folder of current branches to the new branches
        // 3. Build the head

        self.init_branch(&new_branch_name);
        // Create the default branch head
        self.init_checkout(&new_branch_name);
        println!("TODO: Create new branch based on current_branch");
    }

    fn fetch_branch_list(&mut self) {
        let read_dir: ReadDir= fs::read_dir("./.gup/checkout").unwrap();
        for entry in read_dir {
            let dir_entry: DirEntry = entry.unwrap();
            let dir_path: PathBuf = dir_entry.path();
            let split_text: Vec<&str> = dir_path.to_str().unwrap().split("/").collect::<Vec<_>>();

            self.branch_list.push(format!("{}", split_text[split_text.len() - 1]));
        }
    }

    pub fn init_repository(&self, starting_branch: String) {
        match fs::create_dir("./.gup") {
            Ok(()) => (),
            Err(_) => return println!("This folder has been previously been initialized"),
        }
        match fs::create_dir("./.gup/commit") {
            Ok(()) => (),
            Err(e) => return println!("{}", e)
        }
        // create the default branch
        self.init_branch(&starting_branch);
        match fs::create_dir("./.gup/checkout") {
            Ok(()) => (),
            Err(_) => return println!("hah")
        }
        // Create the default branch head
        self.init_checkout(&starting_branch);
        println!("Gup repository initialized!");
    }

    pub fn init_branch(&self, branch_name: &String) {
        match fs::create_dir(format!("./.gup/commit/{branch_name}")) {
            Ok(()) => (),
            Err(_) => return println!("failed to initiate branch")
        }
    }

    pub fn init_checkout(&self, branch_name: &String) {
        match fs::create_dir(format!("./.gup/checkout/{branch_name}")) {
            Ok(()) => (),
            Err(_) => return println!("failed to init checkout head")
        }
        let mut active_branch = File::create("./.gup/active_branch.txt").unwrap();
        write!(active_branch, "{}", branch_name).unwrap();
    }

    fn fetch_active_branch(&mut self) {
        let active_branch = fs::read_to_string("./.gup/active_branch.txt").unwrap();
        self.active_branch = active_branch;
    }
}