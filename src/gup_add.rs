use std::{fs, io};
use std::fs::{File, metadata, OpenOptions, ReadDir};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use clap::Error;

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

        println!("Hei: {:?}", files_to_compare);
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
                .read(true)
                .create(true)
                .open("./.gup/stage_list.txt").unwrap(),
        };

        stage_list_manager.update_stage_list();

        stage_list_manager
    }

    fn update_stage_list(&mut self) {
        let mut content = String::new();
        self.stage_list_raw.read_to_string(&mut content).unwrap();
        for line in content.split("\n").collect::<Vec<_>>() {
            if line != "" {
                let split_line = line.split("-|-").collect::<Vec<_>>();
                self.stage_list.push(
                    StageList {
                        file_path: split_line[0].to_string(),
                        status: split_line[1].to_string(),
                        timestamp: split_line[2].parse::<u64>().unwrap(),
                    }
                );
            }
        }
    }


    pub fn push(&mut self, file: &Path, files_to_compare: &Vec<PathBuf>, compared_files: &mut Vec<PathBuf>) -> io::Result<()> {
        let mut stage_list_raw = &self.stage_list_raw;
        let path_bytes = file.to_str().unwrap();
        let timestamp_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let mut status = "CREATED";

        // Compare all the item from the head that needs comparing
        for compare_file in files_to_compare {
            if file.file_name().unwrap() == compare_file.file_name().unwrap() {
                let is_identical = self.compare_files(&file.to_path_buf(), compare_file).unwrap();
                if is_identical {
                    return Ok(());
                } else {
                    status = "UPDATED";
                    compared_files.push(file.to_path_buf());
                }
            }
        }

        for stage_list in &self.stage_list {
            if stage_list.file_path == file.to_str().unwrap() {
                // no need to compare because this will read the same file twice hence will be always true
                if &timestamp_epoch.as_secs() > &stage_list.timestamp {
                    status = "UPDATED";
                }
            }
        }
        writeln!(stage_list_raw, "{path_bytes}-|-{}-|-{}\n", status, timestamp_epoch.as_secs()).unwrap();

        self.update_stage_list();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn consume() {
        // TODO: Should dequeue the stage_list and add them to commit/branch/v(X)
    }

    pub fn compare_files(&self, file_a: &std::path::PathBuf, file_b: &std::path::PathBuf) -> Result<bool, Error> {
        let mut identical = false;

        let meta_a = metadata(file_a)?;
        let meta_b = metadata(file_b)?;


        // validate file size
        if meta_a.len() != meta_b.len() {
            println!("{:?} != {:?}", file_a, file_b);
            return Ok(false);
        }


        let x = File::open(file_a)?;
        let y = File::open(file_b)?;
        let mut x_reader = BufReader::with_capacity(10, x);
        let mut y_reader = BufReader::with_capacity(10, y);

        loop {
            let (x_length, y_length) = {
                let x_buffer = x_reader.fill_buf()?;
                let y_buffer = y_reader.fill_buf()?;

                if x_buffer != y_buffer {
                    break;
                }

                (x_buffer.len(), y_buffer.len())
            };

            if x_length <= 0 && y_length <= 0 {
                identical = true;
                break;
            }

            if x_length <= 0 || y_length <= 0 {
                break;
            }
            x_reader.consume(x_length);
            y_reader.consume(y_length);
        }

        if identical {
            println!("File is identical");
            return Ok(true);
        } else {
            println!("Nah Bro");
            return Ok(false);
        }
    }
}