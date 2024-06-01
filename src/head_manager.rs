use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use crate::branch_manager::BranchManager;

#[derive(Clone)]
pub struct HeadManager {
    branch_manager: BranchManager,
}

#[allow(dead_code)]
impl HeadManager {
    pub fn new(branch_manager: &BranchManager) -> HeadManager {
        HeadManager {
            branch_manager: branch_manager.clone(),
        }
    }

    fn _construct_head(&self, path: &Path, parent: &str) {
        let entries = path.read_dir().unwrap().map(|r| r.unwrap()).collect::<Vec<_>>();
        for entry in entries {
            let path: PathBuf = entry.path();
            if path.is_file() && path.file_name().unwrap() == ".message.txt" {
                // ignore message file
                // IMPROVEMENT: might want to combine them all into one file later for gup log
                continue;
            }
            if path.is_dir() {
                self._construct_head(&path, parent);
                return;
            }
            let target = format!("./.gup/checkout/{}{}", self.branch_manager.get_active_branch(), path.to_str().unwrap().trim_start_matches(parent));
            let target_path = Path::new(target.as_str());

            let target_parent = target_path.parent().unwrap();
            fs::create_dir_all(target_parent).unwrap();
            fs::copy(path, target_path).unwrap();
        }
    }

    pub fn construct_head(&self) {
        let paths: Vec<_> = fs::read_dir(format!("./.gup/commit/{}", self.branch_manager.get_active_branch())).unwrap()
            .collect();
        for i in 0..paths.len() {
            let path: PathBuf = Path::new(format!("./.gup/commit/{}/{}", self.branch_manager.get_active_branch(), i).as_str()).to_path_buf();
            self._construct_head(&path, path.to_str().unwrap());
        }
    }

    fn break_workdir(&self) {
        // TODO: probably want to compare the file and if its different dont delete
        // Probably also need to pass through the entire repo to check if everything is identical first before starting to delete
        let head: Vec<DirEntry> = fs::read_dir(format!("./.gup/checkout/{}", self.branch_manager.get_active_branch()).as_str())
            .unwrap().map(|r| r.unwrap()).collect();
        for entry in head {
            let path = entry.path();
            let delete = Path::new(format!("./{}", path.file_name().unwrap().to_str().unwrap()).as_str()).to_path_buf();
            if !delete.exists() {
                return;
            }
            if path.is_dir() {
                fs::remove_dir_all(delete).unwrap();
                return;
            }
            fs::remove_file(delete).unwrap();
        }
    }

    fn _construct_workdir(&self, path: &Path, parent: &str) {
        if path.is_dir() {
            let entries: Vec<DirEntry> = path.read_dir().unwrap().map(|r| r.unwrap()).collect();
            for entry in entries {
                self._construct_workdir(&entry.path(), parent);
            }
            return;
        }
        let target = format!("./{}", path.to_str().unwrap().trim_start_matches(parent));
        let target_path = Path::new(target.as_str());

        let target_parent = target_path.parent().unwrap();
        fs::create_dir_all(target_parent).unwrap();
        fs::copy(path, target_path).unwrap();
    }

    fn construct_workdir(&self) {
        // DFS the head
        let parent = format!("./.gup/checkout/{}", self.branch_manager.get_active_branch());
        let head: Vec<DirEntry> = fs::read_dir(&parent.as_str())
            .unwrap().map(|r| r.unwrap()).collect();
        for entry in head {
            self._construct_workdir(&entry.path(), parent.as_str());
        }
    }

    pub fn checkout(&self, branch: &String) {
        let read_dir = fs::read_dir("./.gup/checkout").unwrap();
        for entry in read_dir {
            let dir_entry = entry.unwrap();
            let dir_path = dir_entry.path();
            let split_text = &dir_path.to_str().unwrap().split("/").collect::<Vec<_>>();

            // Check if branch exist
            if split_text[split_text.len() - 1] == branch {
                // Delete the existing workdir
                self.break_workdir();

                fs::write("./.gup/active_branch.txt", split_text[split_text.len() - 1]).unwrap();
                // Build the head
                self.construct_head();
                self.construct_workdir();
                // TODO: Take the head and build the workdir
                println!("you are now at {}. Enjoy :)", branch);
                return;
            }
        }
        println!("branch {} not found", branch);
    }
}