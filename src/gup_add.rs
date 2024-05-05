use std::fs;
use std::path::{Path, PathBuf};

pub fn add(path_buf: &PathBuf) {
    if path_buf.as_path() == Path::new(".") {
        // Read all files and implement the drawn logic
        println!("This is equal");
        // 1. validate if the current folder is a gup repo
        // 2. Read all files/folder except .gup
        // 3. Read filter out all path that match the .gupignore
        match fs::read_dir(".") {
            Ok(v) => {
                for x in v {
                    println!("X: {:?}", x.unwrap().path());
                }
            },
            Err(e) => println!("{}", e)
        };
    }
}