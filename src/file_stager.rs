use std::{fs};
use std::fs::{metadata, ReadDir};
use std::path::{Path, PathBuf};

use crate::branch_manager::BranchManager;
use crate::stage_list_manager::{StageListManager};

pub struct FileStager {
    stage_list_manager: StageListManager,
    branch_manager: BranchManager,
}

impl FileStager {
    pub fn new(branch_manager: BranchManager) -> FileStager {
        FileStager {
            stage_list_manager: StageListManager::new(branch_manager.clone()),
            branch_manager,
        }
    }

    fn fetch_head(&self, branch: &String) -> ReadDir {
        fs::read_dir(format!("./.gup/checkout/{branch}")).unwrap()
    }

    pub fn stage(&mut self, path_buf: &PathBuf) -> Result<(), &str> {
        if !Path::new("./.gup").exists() {
            println!("This is not a gup repository");
            return Err("This is not a gup repository");
        }
        let mut files_to_compare: Vec<PathBuf> = Vec::new();
        let mut compared_files: Vec<PathBuf> = Vec::new(); // TODO: Loop over this and files_to_compare and see if there is a diff, the diff is marked as deleted
        let head = self.fetch_head(&self.branch_manager.active_branch);
        for entry in head {
            let entry_path = entry.unwrap().path();
            files_to_compare.push(entry_path);
        }

        let path = path_buf.to_str().unwrap();
        let mut ignore_list: Vec<PathBuf> = Vec::new();
        Self::preprocess(&mut ignore_list);
        match path {
            "." => self._stage(path_buf, &ignore_list, &files_to_compare, &mut compared_files),
            _ => {}
        }

        Ok(())
    }

    fn _stage(&mut self, path: &Path, ignore_list: &Vec<PathBuf>, files_to_compare: &Vec<PathBuf>, compared_files: &mut Vec<PathBuf>) {
        let read_dir = fs::read_dir(path).unwrap();
        for entry in read_dir {
            let dir_path = entry.unwrap().path();
            if ignore_list.contains(&dir_path) {
                continue;
            };
            let meta = metadata(&dir_path).unwrap();
            if meta.is_file() {
                let _ = &self.stage_list_manager.push(&dir_path, files_to_compare, compared_files).unwrap();
                continue;
            }
            if meta.is_dir() {
                self._stage(&dir_path, &ignore_list, files_to_compare, compared_files);
            }
        }
    }

    fn preprocess(ignore_buf: &mut Vec<PathBuf>) {
        let gup_ignore = fs::read_to_string("./.gupignore").unwrap_or_else(|_| "".to_string());
        let mut ignore_list: Vec<PathBuf> = Vec::new();
        ignore_list.push(Path::new("./.gup").to_path_buf());
        for entry in gup_ignore.split("\n").collect::<Vec<_>>() {
            ignore_list.push(Path::new(entry).to_path_buf());
        }

        *ignore_buf = ignore_list;
    }
}
