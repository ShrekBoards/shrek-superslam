use std::env;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::files::{Ltimesh, LtimeshEntryType};

mod args;
use args::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the .ltimesh file
    let ltimesh = Ltimesh::from_file(&config.ltimesh_path, config.console).unwrap();

    // Print information on the file
    println!("{} entries:", ltimesh.files.len());
    for file in &ltimesh.files {
        println!(
            "\t{} (0x{:08X}): type {}, {} bytes",
            &file.name,
            file.hash,
            match file.file_type {
                LtimeshEntryType::A => 1,
                LtimeshEntryType::B => 2,
                LtimeshEntryType::C => 3,
            },
            file.data.len(),
        );
    }
}
