use std::fs;

pub fn init_repository() {
    match fs::create_dir("./.gup") {
        Ok(()) => (),
        Err(_) => return println!("This folder has been previously been initialized"),
    }
    match fs::create_dir("./.gup/commit") {
        Ok(()) => (),
        Err(_) => return println!("wot")
    }
    match fs::create_dir("./.gup/checkout") {
        Ok(()) => (),
        Err(_) => return println!("hah")
    }
    println!("Gup repository initialized!");
}
