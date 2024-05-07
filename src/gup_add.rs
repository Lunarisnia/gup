use std::fs;
use std::path::{Path, PathBuf};


pub fn add(path_buf: &PathBuf) {
    let ignore_list : Vec<Path> = Vec::new();
    if !Path::new("./.gup").exists() {
        return println!("This is not a gup repository");
    }
    if path_buf.as_path() == Path::new(".") {
        // Read all files and implement the drawn logic
        // 1. validate if the current folder is a gup repo
        // 2. Read all files/folder except .gup
        // 3. Read filter out all path that match the .gupignore
        match fs::read_dir(".") {
            Ok(readDir) => {
                for dir in readDir {
                    if !dir.unwrap().path().as_path() == Path::new("./.gup") {
                        // TODO: Ignore list here
                    }
                    // println!("X: {:?}", dir.unwrap().path());
                }
            }
            Err(e) => println!("{}", e)
        };
    }
}