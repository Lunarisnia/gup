use std::fs;
use std::path::{Path, PathBuf};

pub fn add(path_buf: &PathBuf) {
    if !Path::new("./.gup").exists() {
        return println!("This is not a gup repository");
    }
    if path_buf.as_path() == Path::new(".") {
        let gup_ignore = fs::read_to_string("./.gupignore").unwrap_or_else(|_| "".to_string());
        let mut ignore_list : Vec<&Path> = Vec::new();
        ignore_list.push(Path::new("./.gup"));
        for entry in gup_ignore.split("\n").collect::<Vec<_>>() {
            ignore_list.push(Path::new(entry));
        }

        match fs::read_dir(".") {
            Ok(readDir) => {
                for dir in readDir {
                    let dir_entry = dir.unwrap().path();
                    let current_path = dir_entry.as_path();
                    if !ignore_list.contains(&current_path) {
                        // TODO: Staging files logic 1 here
                        println!("NOT GUP: {:?}", current_path)
                    }
                }
            }
            Err(e) => println!("{}", e)
        };
    }
}