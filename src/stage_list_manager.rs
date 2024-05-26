use std::{fs, io};
use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fs::{DirEntry, File, metadata, OpenOptions, ReadDir};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::Error;

use crate::branch_manager::BranchManager;

#[allow(dead_code)]
#[derive(Debug)]
pub struct StageList {
    file_path: String,
    status: String,
    timestamp: u64,
}

pub struct StageListManager {
    stage_list_raw: File,
    stage_list: VecDeque<StageList>,
    branch_manager: BranchManager,
}

impl StageListManager {
    pub fn new(branch_manager: BranchManager) -> StageListManager {
        let mut stage_list_manager = StageListManager {
            stage_list: VecDeque::new(),
            stage_list_raw: OpenOptions::new()
                .append(true)
                .write(true)
                .read(true)
                .create(true)
                .open("./.gup/stage_list.txt").unwrap(),
            branch_manager,
        };

        stage_list_manager.update_stage_list();

        stage_list_manager
    }

    fn update_stage_list(&mut self) {
        self.stage_list = VecDeque::new();
        let mut content = String::new();
        self.stage_list_raw.read_to_string(&mut content).unwrap();
        for line in content.split("\n").collect::<Vec<_>>() {
            if line != "" {
                let split_line = line.split("-|-").collect::<Vec<_>>();
                self.stage_list.push_back(
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

        println!("HERE: {:?}", files_to_compare);
        // Compare all the item from the head that needs comparing
        for compare_file in files_to_compare {
            if file.file_name().unwrap() == compare_file.file_name().unwrap() {
                let is_identical = self.compare_files(&file.to_path_buf(), compare_file).unwrap();
                println!("Check for identical");
                if is_identical {
                    println!("IDENTICAL");
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
    pub fn consume(&mut self) {
        // TODO: Lets work on this
        // TODO: Should dequeue the stage_list and add them to commit/branch/v(X)
        match self.stage_list.pop_front() {
            None => {
                println!("Queue is empty");
                return;
            }
            Some(staged) => {
                // All of this will be copied to the same commit folder function
                // It will error if the directory doesn't exist, file does not matter
                // Create Commit dir and copy file into it
                // TODO: Copy file, make sure it can create if the destination don't exist
                let dirs: ReadDir = fs::read_dir(format!("./.gup/commit/{}", self.branch_manager.active_branch)).unwrap();
                let mut version_stack: u64 = 0;
                for _ in dirs {
                    version_stack += 1;
                }

                if version_stack > 0 {
                    println!("VSTACK: {:?}", version_stack);
                    println!("latestINDEX: {version_stack}");
                    self.create_commit_dir(version_stack);

                    println!("YOOOO: {:?}", staged);
                    let target =
                        format!("./.gup/commit/{}/v{}/{}",
                                self.branch_manager.active_branch,
                                version_stack,
                                staged.file_path.trim_start_matches("./"));
                    println!("TARGET: {target}");

                } else {
                    // Means there is no commit yet
                    self.create_commit_dir(0);
                }
            }
        }
    }

    fn create_commit_dir(&self, index: u64) {
        match fs::create_dir(format!("./.gup/commit/{}/{}", self.branch_manager.active_branch, index)) {
            Ok(()) => (),
            Err(e) => return println!("failed to create commit dir: {}", e)
        }
    }

    pub fn compare_files(&self, file_a: &PathBuf, file_b: &PathBuf) -> Result<bool, Error> {
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
            Ok(true)
        } else {
            println!("Nah Bro");
            Ok(false)
        }
    }
}
