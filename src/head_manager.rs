use std::fs;
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
            // println!("Copying: {:?}", entry);
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
            let target = format!("./.gup/checkout/{}{}", self.branch_manager.active_branch, path.to_str().unwrap().trim_start_matches(parent));
            let target_path = Path::new(target.as_str());

            let target_parent = target_path.parent().unwrap();
            fs::create_dir_all(target_parent).unwrap();
            fs::copy(path, target_path).unwrap();
        }
    }

    pub fn construct_head(&self) {
        // Try sorting first then process the thing
        let mut paths: Vec<_> = fs::read_dir(format!("./.gup/commit/{}", self.branch_manager.active_branch)).unwrap()
            .map(|r| r.unwrap()).collect();
        paths.sort_by_key(|dir| dir.path());
        for entry in paths {
            let path: PathBuf = entry.path();
            self._construct_head(&path, path.to_str().unwrap());
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
}