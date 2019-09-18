use std::fs;
use std::path;

use crate::args::Config;
use shrek_superslam::compression::decompress;
use shrek_superslam::master_dir::{MasterDir, MasterDirEntry};

/// Create the destination directory for a given file from its MASTER.DIR entry
///
/// \param path The path of the file to create a directory for
fn create_destination_directory(path : &String) -> path::PathBuf {
    let mut filepath = path::PathBuf::new();
    for part in path.split('\\') {
        filepath.push(part.trim_matches(char::from(0)));
    }
    fs::create_dir_all(filepath.parent().unwrap());
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
fn dump_entries(master_dat : &Vec<u8>, entries : &[MasterDirEntry], config : &Config) {

    for entry in entries {
        println!("0x{:x}: {}", entry.offset, entry.name);

        // Create the destination directory to write the file to
        let filepath = create_destination_directory(&entry.name);

        // Get the compressed bytes for this entry
        let lower = entry.offset as usize;
        let upper = (entry.orig_size as usize) + lower;
        let entry = &master_dat[lower..upper];

        // Decompress the entry if requested
        if config.decompress {
            let decompressed = decompress(entry);
        }

        // Write the data to the filepath
        fs::write(filepath, entry).expect("Unable to write file");
    }
}

/// Dumps the extracted contents of a MASTER.DAT file to the desired output
/// directory.
///
/// \param master_dat The bytes of the entire MASTER.DAT file
/// \param master_dir The parsed entries from the MASTER.DIR file
/// \param config     The program config
pub fn dump_master_dat(master_dat : Vec<u8>, master_dir : MasterDir, config : &Config) {

    let chunk_size = master_dir.entries.len() / 4;

    for chunk in master_dir.entries.chunks(chunk_size) {
        dump_entries(&master_dat, chunk, config);
    }
}
