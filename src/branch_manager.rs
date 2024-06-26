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
            branch_manager.update_active_branch()
        }

        branch_manager
    }

    fn copy_commit(&self, source_commit: &Path, target_commit: &Path, source_parent: &Path) {
        let entries = source_commit
            .read_dir()
            .unwrap()
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        for entry in entries {
            let path: PathBuf = entry.path();
            if path.is_dir() {
                self.copy_commit(&path, &target_commit, &source_commit);
                return;
            }
            let target_parent = format!(
                "{}{}",
                target_commit.to_str().unwrap(),
                path.parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_start_matches(source_parent.to_str().unwrap())
            );

            fs::create_dir_all(&target_parent).unwrap();
            let target = format!(
                "{}{}",
                &target_parent.as_str(),
                path.to_str()
                    .unwrap()
                    .trim_start_matches(&path.parent().unwrap().to_str().unwrap())
            );
            fs::copy(path, target).unwrap();
        }
    }

    pub fn create_new_branch(&self, new_branch_name: &String) {
        // 0. Check if the branch already existed, if yes then tell the user
        let target_head: bool =
            Path::new(format!("./.gup/checkout/{}", &new_branch_name).as_str()).exists();
        if target_head {
            println!("Branch already existed man");
            return;
        }

        // 1. Create commit and head folder for this new branch
        self.init_branch(&new_branch_name);
        self.init_checkout(&new_branch_name);

        // 2. Copy the entire commit folder of current branches to the new branches
        let source_commit: PathBuf =
            Path::new(format!("./.gup/commit/{}", self.get_active_branch()).as_str()).to_path_buf();
        let source_commits: Vec<_> = source_commit
            .read_dir()
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        for commits in source_commits {
            let commit: PathBuf = commits.path();
            let dir_name = commit.file_name().unwrap();

            let target_commit = Path::new(
                format!(
                    "./.gup/commit/{}/{}",
                    &new_branch_name,
                    dir_name.to_str().unwrap()
                )
                .as_str(),
            )
            .to_path_buf();
            fs::create_dir(&target_commit).unwrap();
            self.copy_commit(&commit, &target_commit, &commit);
        }

        // Head will be built when the user checks out to the actual branch
        println!("Branch {} has been created.", new_branch_name);
    }

    fn fetch_branch_list(&mut self) {
        let read_dir: ReadDir = fs::read_dir("./.gup/checkout").unwrap();
        for entry in read_dir {
            let dir_entry: DirEntry = entry.unwrap();
            let dir_path: PathBuf = dir_entry.path();
            let split_text: Vec<&str> = dir_path.to_str().unwrap().split("/").collect::<Vec<_>>();

            self.branch_list
                .push(format!("{}", split_text[split_text.len() - 1]));
        }
    }

    pub fn init_repository(&self, starting_branch: String) {
        match fs::create_dir("./.gup") {
            Ok(()) => (),
            Err(_) => return println!("This folder has been previously been initialized"),
        }
        match fs::create_dir("./.gup/commit") {
            Ok(()) => (),
            Err(e) => return println!("{}", e),
        }
        // create the default branch
        self.init_branch(&starting_branch);
        match fs::create_dir("./.gup/checkout") {
            Ok(()) => (),
            Err(_) => return println!("hah"),
        }
        // Create the default branch head
        self.init_checkout(&starting_branch);
        self.checkout_to(&starting_branch);
        println!("Gup repository initialized!");
    }

    pub fn init_branch(&self, branch_name: &String) {
        match fs::create_dir(format!("./.gup/commit/{branch_name}")) {
            Ok(()) => (),
            Err(_) => return println!("failed to initiate branch"),
        }
    }

    pub fn init_checkout(&self, branch_name: &String) {
        match fs::create_dir(format!("./.gup/checkout/{branch_name}")) {
            Ok(()) => (),
            Err(_) => return println!("failed to init checkout head"),
        }
    }

    pub fn checkout_to(&self, branch_name: &String) {
        let mut active_branch = File::create("./.gup/active_branch.txt").unwrap();
        write!(active_branch, "{}", branch_name).unwrap();
    }

    fn update_active_branch(&mut self) {
        let active_branch = fs::read_to_string("./.gup/active_branch.txt").unwrap();
        self.active_branch = active_branch;
    }

    pub fn get_active_branch(&self) -> String {
        fs::read_to_string("./.gup/active_branch.txt").unwrap()
    }
}
