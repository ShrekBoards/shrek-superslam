use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use crossbeam::thread;

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

/// Given a list of MASTER.DIR entries and the MASTER.DAT file, pulls out each
/// entry from the file, decompresses them if required, and saves them to the
/// required output directory.
///
/// \param master_dat The bytes of the entire MASTER.DAT file
/// \param entries    A list of MASTER.DIR entries representing the file to
///                   pull out
/// \param config     The program config
fn dump_entries(master_dat: &MasterDat, files: &[&String], config: &Config) {
    for path in files {
        // Create the destination directory to write the file to
        let output_path = create_destination_directory(&path);

        // Decompress the entry if requested
        /*
        let output = if config.decompress {
            master_dat.decompressed_file(&path)
        } else {
            master_dat.compressed_file(&path)
        };
        */
        let output = master_dat.compressed_file(&path);

        // Write the data to disk
        match output {
            Some(f) => fs::write(output_path, f).expect("Unable to write file"),
            _ => ()
        };
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the MASTER.DIR and MASTER.DAT files
    let master_dir = MasterDir::from_file(&config.master_dir_path, config.console);
    let master_dat = MasterDat::from_file(
        &config.master_dat_path,
        master_dir,
        config.console
    );

    // Split the list of files within the MASTER.DAT, and use a different thread
    // to decompress the files in each part
    let chunk_size = master_dat.files().len() / 4;
    /*
    let master_dat_arc = Arc::new(master_dat);
    thread::scope(|scope| {
        for entries in master_dat.files().chunks(chunk_size) {
            let master_dat = master_dat_arc.clone();
            scope.spawn(move |_| dump_entries(&master_dat, entries, &config));
        }
    }).unwrap();
    */

    for entries in master_dat.files().chunks(chunk_size) {
        dump_entries(&master_dat, entries, &config);
    }
}
