use std::{fs, io};
use std::fs::{File, metadata, OpenOptions};
use std::io::{BufRead, Read, read_to_string, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::branch_manager::BranchManager;

pub struct FileStager {
    // TODO: need a branch manager struct that keep track of current active branch and all available
    stage_list_manager: StageListManager,
    branch_manager: BranchManager,
}

impl FileStager {
    pub fn new(branch_manager: BranchManager) -> FileStager {
        FileStager {
            stage_list_manager: StageListManager::new(),
            branch_manager,
        }
    }
    pub fn stage(&mut self, path_buf: &PathBuf) -> Result<(), &str> {
        if !Path::new("./.gup").exists() {
            println!("This is not a gup repository");
            return Err("This is not a gup repository");
        }
        let path = path_buf.to_str().unwrap();
        let mut ignore_list: Vec<PathBuf> = Vec::new();
        Self::preprocess(&mut ignore_list);
        match path {
            "." => self._stage(path_buf, &ignore_list),
            _ => {}
        }

        Ok(())
    }

    fn _stage(&mut self, path: &Path, ignore_list: &Vec<PathBuf>) {
        let read_dir = fs::read_dir(path).unwrap();
        for entry in read_dir {
            let dir_path = entry.unwrap().path();
            if ignore_list.contains(&dir_path) {
                continue;
            };
            let meta = metadata(&dir_path).unwrap();
            if meta.is_file() {
                let _ = &self.stage_list_manager.push(&self.branch_manager, &dir_path).unwrap();
                continue;
            }
            if meta.is_dir() {
                self._stage(&dir_path, &ignore_list);
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

struct StageList {
    file_path: String,
    status: String,
    timestamp: u64,
}

struct StageListManager {
    stage_list_raw: File,
    stage_list: Vec<StageList>,
}

impl StageListManager {
    pub fn new() -> StageListManager {
        let mut stage_list_manager = StageListManager {
            stage_list: Vec::new(),
            stage_list_raw: OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open("./.gup/stage_list.txt").unwrap(),
        };

        println!("Went Here");
        // TODO: This goes into infinite loop for no reason
        stage_list_manager.update_stage_list();

        stage_list_manager
    }

    fn update_stage_list(&mut self) {
        for line in read_to_string(&self.stage_list_raw).unwrap().lines() {
            let split_line = line.split("-|-").collect::<Vec<_>>();
            self.stage_list.push(
                StageList {
                    file_path: split_line[0].to_string(),
                    status: split_line[1].to_string(),
                    timestamp: u64::from(split_line[2]),
                }
            );
        }
    }

    pub fn push(&mut self, branch_manager: &BranchManager, file: &Path) -> io::Result<()> {
        // TODO: Maybe do this from BranchManager
        // TODO: Check if exist on head
        // TODO: Check if already exist on stage_list
        let mut stage_list = &self.stage_list_raw;
        let path_bytes = file.to_str().unwrap();
        let timestamp_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        writeln!(stage_list, "{path_bytes}-|-{}-|-{}", "CREATED", timestamp_epoch.as_secs()).unwrap();

        self.update_stage_list();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn consume() {
        // TODO: Should dequeue the stage_list and add them to commit/branch/v(X)
    }
}