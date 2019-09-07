use crate::args::Config;
use shrek_superslam::compression::decompress;
use shrek_superslam::master_dir::{MasterDir, MasterDirEntry};

/// Create the destination directory for a given file from its MASTER.DIR entry
///
/// \param path The path of the file to create a directory for
fn create_destination_directory(path : String) {
}

/// Given a single MASTER.DIR entry and the MASTER.DAT file, pulls out the
/// specific entry from the file, decompresses it if required, and saves it to
/// the required output directory.
///
/// \param master_dat       The bytes of the entire MASTER.DAT file
/// \param master_dir_entry The single MASTER.DIR entry representing the file to
///                         pull out
/// \param config           The program config
fn dump_entry(master_dat : &Vec<u8>, master_dir_entry : MasterDirEntry, config : &Config) {

    // Get the compressed bytes for this entry
    let lower = master_dir_entry.offset as usize;
    let upper = (master_dir_entry.orig_size as usize) + lower;
    let entry = &master_dat[lower..upper];

    // Decompress the entry if requested
    if config.decompress {
        let decompressed = decompress(entry);
    }
}

/// Dumps the extracted contents of a MASTER.DAT file to the desired output
/// directory.
///
/// \param master_dat The bytes of the entire MASTER.DAT file
/// \param master_dir The parsed entries from the MASTER.DIR file
/// \param config     The program config
pub fn dump_master_dat(master_dat : Vec<u8>, master_dir : MasterDir, config : &Config) {

    // Iterate over each MASTER.DIR entry, and use it to calculate offsets into
    // the MASTER.DAT file, and the sizes of each entry
    for entry in master_dir.entries {
        println!("{} (0x{:x}, 0x{:x}, 0x{:x})",
            entry.name,
            entry.offset,
            entry.malloc_size,
            entry.orig_size
        );

        dump_entry(&master_dat, entry, &config);
    }
}
