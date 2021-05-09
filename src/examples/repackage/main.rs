use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use walkdir::WalkDir;

extern crate shrek_superslam;
use shrek_superslam::files::Texpack;
use shrek_superslam::{Console, MasterDat};

mod args;
use args::Config;

/// Repackage an extracted texpack located at the given `extracted_dir_path`
/// for the given `console`.
fn repackage_texpack(extracted_dir_path: &Path, texpack_path: &Path, console: Console) {
    // Create a new empty texpack.
    let mut texpack = Texpack::new(console);

    // Iterate over the extracted directory and add each file to the new texpack.
    for file in WalkDir::new(&extracted_dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|d| d.file_type().is_file())
    {
        let name = file
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let file_contents = fs::read(&file.path()).unwrap();
        texpack.add_file(name, &file_contents);
    }

    // Write the texpack to a file, overwriting the original
    let mut texpack_outfile = File::create(texpack_path).unwrap();
    texpack_outfile
        .write_all(&texpack.to_bytes().unwrap())
        .unwrap();
}

/// Perform additional operations on a file if required.
fn additonal_file_operations(path: &Path, console: Console) {
    match path.extension().unwrap_or(OsStr::new("")).to_str().unwrap() {
        // If the file is a .texpack, check if an extracted directory exists
        // for it, and repackage that directory into the .texpack file before
        // then adding the new file to the MASTER.DAT.
        "texpack" => {
            // The extract program creates directories next to the texpack
            // file with the same name as the texpack, with "-extract" appended.
            let extracted_texpack_path = path.with_file_name(format!(
                "{}-extracted",
                path.file_name().unwrap().to_str().unwrap()
            ));

            // If that directory exists, turn it back into the texpack.
            if extracted_texpack_path.exists() {
                repackage_texpack(&extracted_texpack_path, path, console);
            }
        }
        _ => {}
    };
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Create a new MASTER.DAT file
    let mut master_dat = MasterDat::new(config.console);

    // Add each file in the data/ directory to the MASTER.DAT
    for file in WalkDir::new(&config.data_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|d| d.file_type().is_file())
        .filter(|d| {
            !d.path()
                .parent()
                .unwrap_or(Path::new(""))
                .as_os_str()
                .to_str()
                .unwrap()
                .ends_with("-extracted")
        })
    {
        // Some files may require additional operations, such as repackaging
        // extracted files, before they are read and compressed into the
        // MASTER.DAT.
        additonal_file_operations(&file.path(), config.console);

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

        let relative_path = file
            .path()
            .strip_prefix(prefix)
            .unwrap()
            .to_str()
            .unwrap()
            .replace('\\', "/");

        // Add the file to the MASTER.DAT
        master_dat.add_file(relative_path, &contents);
    }

    // Write the new MASTER.DAT and MASTER.DIR to files
    if let Err(e) = master_dat.write(&PathBuf::from("MASTER.DAT"), &PathBuf::from("MASTER.DIR")) {
        panic!("failed to write \"MASTER.DAT\" or \"MASTER.DIR\": {}", e);
    }
}
