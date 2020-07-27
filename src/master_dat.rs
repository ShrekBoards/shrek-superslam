use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::compression::decompress;
use crate::console::Console;
use crate::master_dir::MasterDir;

/*
use std::char;
use std::panic;
use std::sync::Arc;

use crossbeam::thread;

use crate::args::Config;

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
        /*
        if !entry.name.contains("dutch.dds") {
            continue;
        }
        */
        println!("0x{:x}: {}", entry.offset, entry.name);

        // Create the destination directory to write the file to
        let filepath = create_destination_directory(&entry.name);

        // Get the compressed bytes for this entry
        let lower = entry.offset as usize;
        let upper = (entry.orig_size as usize) + lower;
        let entry = &master_dat[lower..upper];

        // Decompress the entry if requested
        let output = if config.decompress {
            let result = panic::catch_unwind(|| {
                decompress(entry)
            });
            result.unwrap_or_default()
        } else {
            entry.to_vec()
        };

        // Write the data to the filepath
        fs::write(filepath, output).expect("Unable to write file");
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
    /*
    let master_dat_arc = Arc::new(master_dat);

    thread::scope(|scope| {
        for entries in master_dir.entries.chunks(chunk_size) {
            let master_dat = master_dat_arc.clone();
            scope.spawn(move |_| dump_entries(&master_dat, entries, &config));
        }
    }).unwrap();
    */

    for entries in master_dir.entries.chunks(chunk_size) {
        dump_entries(&master_dat, entries, &config);
    }
}
*/

/// Structure representing the MASTER.DAT file, which contains all of the
/// compressed files for Shrek SuperSlam
pub struct MasterDat {
    /// Mapping of the filenames within the MASTER.DAT to the compressed file data
    files: HashMap<String, Vec<u8>>,

    /// The associated MASTER.DIR file
    master_dir: MasterDir,

    /// The console version this file is from or for
    console: Console,
}

impl MasterDat {
    /// Creates a new empty MasterDat object
    ///
    /// \param console The console this MASTER.DAT is for
    pub fn new(console : Console) -> MasterDat {
        MasterDat {
            files: HashMap::new(),
            master_dir: MasterDir::new(console),
            console: console,
        }
    }

    /// Loads an existing MASTER.DAT file
    ///
    /// \param path       The path to the MASTER.DAT file to load
    /// \param master_dir The associated MASTER.DIR file
    /// \param console    The console this MASTER.DAT file is from
    pub fn from_file(path: &PathBuf, master_dir: MasterDir, console: Console) -> MasterDat {
        let mut f = File::open(path).expect("unable to read master.dat");
        
        // Iterate over the entries within the associated MASTER.DIR, and use it to
        // read out each compressed file from the MASTER.DAT
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        for entry in &master_dir.entries {
            let mut file: Vec<u8> = Vec::with_capacity(entry.comp_size as usize);
            f.seek(SeekFrom::Start(entry.offset as u64));
            f.read(&mut file).expect("unable to read master.dat");
            files.insert(entry.name.clone(), file);
        }

        MasterDat {
            files: files,
            master_dir: master_dir,
            console: console,
        }
    }
}