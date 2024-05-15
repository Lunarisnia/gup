use std::fs;
use std::path::{Path, PathBuf};

pub struct FileStager {
    // TODO: need a branch manager struct that keep track of current active branch and all available
}

impl FileStager {
    pub fn stage(&self, path_buf: &PathBuf) -> Result<(), &str> {
        if !Path::new("./.gup").exists() {
            println!("This is not a gup repository");
            return Err("This is not a gup repository");
        }
        match path_buf.as_path().to_str().unwrap() {
            "." => Self::stage_all(),
            _ => {}
        }

        Ok(())
    }

    fn stage_all() {
        let gup_ignore = fs::read_to_string("./.gupignore").unwrap_or_else(|_| "".to_string());
        let mut ignore_list: Vec<PathBuf> = Vec::new();
        ignore_list.push(Path::new("./.gup").to_path_buf());
        for entry in gup_ignore.split("\n").collect::<Vec<_>>() {
            ignore_list.push(Path::new(entry).to_path_buf());
        }

        let read_dir = fs::read_dir(".").unwrap();

        let mut valid_files: Vec<PathBuf> = Vec::new();
        for entry_result in read_dir {
            // It's always this stupid borrow checker error oh my fucking god
            let entry_path = entry_result.unwrap().path();
            if !ignore_list.contains(&entry_path) {
                valid_files.push(entry_path);
            }
        }


        // TODO: Recursor function to walk each file and directory to compare them to the head
        for file in valid_files {
            println!("FILE: {:?}", file);
        }
    }
}
