use std::cmp;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Arc;

use crossbeam::thread;

extern crate shrek_superslam;
use shrek_superslam::files::Texpack;
use shrek_superslam::{MasterDat, MasterDir};

mod args;
use args::Config;

/// Create the destination directory for a given file from its MASTER.DIR entry.
///
/// # Parameters
///
/// - `path`: The path of the file from a MASTER.DIR entry to create a
///    directory for
///
/// # Returns
///
/// A `PathBuf` to the created directory
fn create_destination_directory(path: &str) -> PathBuf {
    let mut filepath = PathBuf::new();
    for part in path.split('\\') {
        filepath.push(part.trim_matches(char::from(0)));
    }
    fs::create_dir_all(filepath.parent().unwrap()).unwrap();
    filepath
}

/// Extract a texpack file to disk
///
/// # Parameters
///
/// - `path`: The path to the texpack file to extract
/// - `config`: The program config
fn extract_texpack(path: &Path, config: &Config) {
    // Create a new directory for the contents of the extracted texpack
    //
    // The path of the directory is the same as the texpack, with the
    // '-extracted' suffix. So "data\example.texpack" extracts to
    // "data\example.texpack-extracted\".
    let texpack = Texpack::from_file(&path, config.console).expect("could not read texpack");
    let extracted_dir = path.parent().unwrap().join(format!(
        "{}-extracted",
        path.file_name().unwrap().to_string_lossy()
    ));
    fs::create_dir_all(&extracted_dir).unwrap();

    // Extract each file in the texpack to the directory
    for texpack_file in texpack.files() {
        let output_path = extracted_dir.join(&texpack_file.filename());
        fs::write(&output_path, &texpack_file.data).expect("Unable to write file");
    }
}

/// Given a list of files and the MASTER.DAT file, pulls out each file from the
/// MASTER.DAT file, decompresses them if required, and saves them to disk.
///
/// # Parameters
///
/// - `master_dat`: The parsed MASTER.DAT file to dump the entries of
/// - `files`: The list of files to extract from the MASTER.DAT. Must be a
///    subset of `master_dat.files()`
/// - `config`: The program config
fn dump_entries(master_dat: &MasterDat, files: &[String], config: &Config) {
    for path in files {
        // Create the destination directory to write the file to
        let output_path = create_destination_directory(&path);

        // Decompress the entry if requested
        let output = if config.decompress {
            master_dat.decompressed_file(&path)
        } else {
            master_dat.compressed_file(&path)
        };

        // Write the data to disk
        if let Some(f) = output {
            fs::write(&output_path, f).expect("Unable to write file")
        }

        // If the file is a texpack, and we have decompressed it and requested
        // it be extracted, extract it to a new directory
        if config.decompress
            && config.extract_texpack
            && output_path.extension().unwrap_or_default() == "texpack"
        {
            extract_texpack(&output_path, &config);
        }
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the MASTER.DIR and MASTER.DAT files
    let master_dir = match MasterDir::from_file(&config.master_dir_path, config.console) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dir_path, e),
    };
    let master_dat = match MasterDat::from_file(&config.master_dat_path, master_dir) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dat_path, e),
    };

    // Split the list of files within the MASTER.DAT, and use a different thread
    // to decompress the files in each part
    let files = master_dat.files();
    let chunk_size = files.len() / cmp::max(1, num_cpus::get());
    let master_dat_arc = Arc::new(master_dat);
    let config_arc = Arc::new(config);
    thread::scope(|scope| {
        for entries in files.chunks(chunk_size) {
            let master_dat = master_dat_arc.clone();
            let config = config_arc.clone();
            scope.spawn(move |_| dump_entries(&master_dat, entries, &config));
        }
    })
    .unwrap();
}
