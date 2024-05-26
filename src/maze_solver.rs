use std::fs;
use std::fs::{File, metadata};
use std::io::{Read};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub struct MazeSolver {
    pub maze_path: PathBuf,
}

#[allow(dead_code)]
impl MazeSolver {
    pub fn solve(&self) {
        println!("Solving Maze");
        self.dive(&self.maze_path);
    }

    fn dive(&self, diving_spot: &Path) -> bool {
        if metadata(&diving_spot).unwrap().is_file() {
            let p = File::open(&diving_spot);
            let mut content: String = String::new();
            let _ = p.unwrap().read_to_string(&mut content);
            if content == "REAL" {
                println!("The goal is here: {:?}", &diving_spot);
                return true;
            }
            return false;
        }
        let read_dir = fs::read_dir(diving_spot).unwrap();
        for entry in read_dir {
            let dir_path: PathBuf = entry.unwrap().path();
            let found = self.dive(&dir_path);
            if found {
                return true;
            }
        }
        return false;
    }
}
