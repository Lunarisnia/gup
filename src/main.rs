use std::fs::{File, metadata};
use std::io::{BufRead, BufReader, Read};

use clap::{Error, Parser, Subcommand};
use crate::gup_add::FileStager;

mod gup_init;
mod gup_add;

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    // Init the folder as a gup repository
    init {},

    add {
        path: std::path::PathBuf,
    },

    compare {
        file_a: std::path::PathBuf,
        file_b: std::path::PathBuf,
    },
}

fn main() {
    let cli = CLI::parse();
    let file_stager = FileStager{

    };

    match &cli.command {
        Some(Commands::init {}) => gup_init::init_repository("main".to_string()),
        Some(Commands::add { path }) => file_stager.stage(path).unwrap(),
        Some(Commands::compare { file_a, file_b }) => compare_files(file_a, file_b).unwrap(),
        None => (),
    }
}

fn compare_files(file_a: &std::path::PathBuf, file_b: &std::path::PathBuf) -> Result<(), Error> {
    let mut identical = false;

    let meta_a = metadata(file_a)?;
    let meta_b = metadata(file_b)?;


    // validate file size
    if meta_a.len() != meta_b.len() {
        println!("{:?} != {:?}", file_a, file_b);
        return Ok(());
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
    } else {
        println!("Nah Bro");
    }

    Ok(())
}
