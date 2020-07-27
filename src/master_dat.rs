use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::compression::decompress;
use crate::console::Console;
use crate::master_dir::MasterDir;

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
            let mut file: Vec<u8> = vec![0; entry.comp_size as usize];
            f.seek(SeekFrom::Start(entry.offset as u64)).expect("failed to seek");
            f.read_exact(&mut file).expect("unable to read master.dat");
            files.insert(entry.name.clone(), file);
        }

        MasterDat {
            files: files,
            master_dir: master_dir,
            console: console,
        }
    }

    /// Get a compressed file from the MASTER.DAT
    ///
    /// \param path The path of the file to retrieve
    ///
    /// \returns A reference to the bytes of the compressed file if it exists
    ///          in the MASTER.DAT, otherwise None
    pub fn compressed_file(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }

    /// Get and decompress a file from the MASTER.DAT
    ///
    /// \param path The path of the file to retrieve
    ///
    /// \returns The decompressed bytes of the file if it exists in the
    ///          MASTER.DAT, otherwise None
    pub fn decompressed_file(&self, path: &str) -> Option<Vec<u8>> {
        match self.files.get(path) {
            Some(f) => Some(decompress(&f)),
            _ => None
        }
    }

    /// Get the filenames within the MASTER.DAT
    ///
    /// \returns The filenames within the MASTER.DAT
    pub fn files(&self) -> Vec<&String> {
        self.files.keys().collect()
    }
}