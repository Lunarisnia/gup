use std::path::{Path, PathBuf};

pub fn add(path_buf: &PathBuf) {
    if path_buf.as_path() == Path::new(".") {
        // Read all files and implement the drawn logic
        println!("This is equal");
    }
}