use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// This will keep track of what branch currently active, all branches list, active head
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


    fn fetch_branch_list(&mut self) {
        let read_dir = fs::read_dir("./.gup/checkout").unwrap();
        for entry in read_dir {
            let dir_entry = entry.unwrap();
            let dir_path = dir_entry.path();
            let split_text = dir_path.to_str().unwrap().split("/").collect::<Vec<_>>();

            self.branch_list.push(format!("{}", split_text[split_text.len() - 1]));
        }
    }

    pub fn checkout(&self, branch: &String) {
        let read_dir = fs::read_dir("./.gup/checkout").unwrap();
        for entry in read_dir {
            let dir_entry = entry.unwrap();
            let dir_path = dir_entry.path();
            let split_text = &dir_path.to_str().unwrap().split("/").collect::<Vec<_>>();

            if split_text[split_text.len() - 1] == branch {
                fs::write("./.gup/active_branch.txt", split_text[split_text.len() - 1]).unwrap();
            }
        }
    }

    #[allow(dead_code)]
    pub fn fetch_head(&self) -> PathBuf {
        let read_dir = fs::read_dir(format!("./.gup/checkout/{}", self.active_branch)).unwrap();
        for entry in read_dir {
            let dir_entry = entry.unwrap();
            return dir_entry.path();
        }
        PathBuf::new()
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
        match fs::create_dir(format!("./.gup/commit/{branch_name}/v1")) {
            Ok(()) => (),
            Err(_) => return println!("failed to initiate branch versioning")
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