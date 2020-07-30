use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use walkdir::WalkDir;

extern crate shrek_superslam;
use shrek_superslam::master_dat::MasterDat;

mod args;
use args::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Create a new MASTER.DAT file
    let mut master_dat = MasterDat::new(config.console);

    // Add each file in the data/ directory to the MASTER.DAT
    for file in WalkDir::new(&config.data_path).into_iter()
        .filter_map(|e| e.ok()).filter(|d| d.file_type().is_file())
    {
        // Read the file
        let contents = fs::read(file.path()).expect("could not open file");

        // Convert the path to a Unix-style path, anchored at the data folder
        let mut prefix = PathBuf::new();
        for part in file.path() {
            if part == "data" {
                break;
            }
            prefix = prefix.join(part);
        }

        let relative_path = file.path().strip_prefix(prefix)
            .unwrap().to_str().unwrap().replace('\\', "/");

        // Add the file to the MASTER.DAT
        master_dat.add_file(relative_path, &contents);
    }

    // Write the new MASTER.DAT and MASTER.DIR to files
    master_dat.write(&PathBuf::from("MASTER.DAT"), &PathBuf::from("MASTER.DIR"));
}
