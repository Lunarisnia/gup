use std::fs;

pub fn init_repository(starting_branch: String) {
    match fs::create_dir("./.gup") {
        Ok(()) => (),
        Err(_) => return println!("This folder has been previously been initialized"),
    }
    match fs::create_dir("./.gup/commit") {
        Ok(()) => (),
        Err(e) => return println!("{}", e)
    }
    // create the default branch
    init_branch(&starting_branch);
    match fs::create_dir("./.gup/checkout") {
        Ok(()) => (),
        Err(_) => return println!("hah")
    }
    // Create the default branch head
    init_checkout(&starting_branch);
    println!("Gup repository initialized!");
}

pub fn init_branch(branch_name: &String) {
    match fs::create_dir(format!("./.gup/commit/{branch_name}")) {
        Ok(()) => (),
        Err(_) => return println!("failed to initiate branch")
    }
    match fs::create_dir(format!("./.gup/commit/{branch_name}/v1")) {
        Ok(()) => (),
        Err(_) => return println!("failed to initiate branch versioning")
    }
}

pub fn init_checkout(branch_name: &String) {
    match fs::create_dir(format!("./.gup/checkout/{branch_name}")) {
        Ok(()) => (),
        Err(_) => return println!("failed to init checkout head")
    }
}