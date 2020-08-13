use std::env;
use std::path::Path;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::{MasterDat, MasterDir};
use shrek_superslam::files::Bin;

mod args;
use args::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the MASTER.DIR and MASTER.DAT files
    let master_dir = MasterDir::from_file(&config.master_dir_path, config.console);
    let master_dat = MasterDat::from_file(&config.master_dat_path, master_dir);

    // Iterate over all objects in all .bin files, and print a sequential list
    // of the objects within each file and the offset of each of those objects
    for filepath in master_dat.files() {
        let trimmed = &filepath.trim_end_matches(char::from(0));
        if let Some(extension) = Path::new(&trimmed).extension() {
            if extension == "bin" {
                let bin = Bin::new(
                    master_dat.decompressed_file(&filepath).unwrap(),
                    config.console,
                );
                if !bin.objects().is_empty() {
                    println!("{} ({} objects)", trimmed, bin.objects().len());
                    for object in bin.objects() {
                        println!("\t+{:04x}: {}", object.offset, object.name);
                    }
                }
            }
        }
    }
}
