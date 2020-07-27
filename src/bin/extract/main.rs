use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::master_dir::MasterDir;
use shrek_superslam::master_dat::MasterDat;

mod args;
use args::Config;

/// Create the destination directory for a given file from its MASTER.DIR entry
///
/// \param path The path of the file to create a directory for
fn create_destination_directory(path: &String) -> PathBuf {
    let mut filepath = PathBuf::new();
    for part in path.split('\\') {
        filepath.push(part.trim_matches(char::from(0)));
    }
    fs::create_dir_all(filepath.parent().unwrap()).unwrap();
    filepath
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    let master_dir = MasterDir::from_file(&config.master_dir_path, config.console);
    let master_dat = MasterDat::from_file(
        &config.master_dat_path,
        master_dir,
        config.console
    );
}
