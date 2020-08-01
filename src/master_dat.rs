use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::compression::{compress, decompress};
use crate::console::Console;
use crate::master_dir::{MasterDir, MasterDirEntry};

/// Structure representing the MASTER.DAT file, which contains all of the
/// compressed files for Shrek SuperSlam
pub struct MasterDat {
    /// Mapping of the filenames within the MASTER.DAT to the compressed file data
    files: HashMap<String, Vec<u8>>,

    /// The associated MASTER.DIR file
    master_dir: MasterDir,
}

impl MasterDat {
    /// Creates a new empty MasterDat object
    ///
    /// # Parameters
    ///
    /// - `console`: The console this MASTER.DAT is for
    pub fn new(console: Console) -> MasterDat {
        MasterDat {
            files: HashMap::new(),
            master_dir: MasterDir::new(console),
        }
    }

    /// Loads an existing MASTER.DAT file
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the MASTER.DAT file to load
    /// - `master_dir`: The associated MASTER.DIR file
    /// - `console`: The console this MASTER.DAT file is from
    pub fn from_file(path: &Path, master_dir: MasterDir) -> MasterDat {
        let mut f = File::open(path).expect("unable to read master.dat");

        // Iterate over the entries within the associated MASTER.DIR, and use it to
        // read out each compressed file from the MASTER.DAT
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        for entry in &master_dir.entries {
            let mut file: Vec<u8> = vec![0; entry.comp_size as usize];
            f.seek(SeekFrom::Start(entry.offset as u64))
                .expect("failed to seek");
            f.read_exact(&mut file).expect("unable to read master.dat");
            files.insert(entry.name.clone(), file);
        }

        MasterDat { files, master_dir }
    }

    /// Adds a file to the MASTER.DAT
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the file within the MASTER.DAT
    /// - `data`: The uncompressed data of the file to add
    pub fn add_file(&mut self, path: String, data: &[u8]) {
        // Compress the file
        let compressed = compress(data);

        // Create an entry for the file in the MASTER.DAT. The offset of the
        // file within the MASTER.DAT (which is a field in the MASTER.DIR) is
        // determined by the sum of all the padded sizes of the current files.
        let offset = self
            .files
            .values()
            .fold(0, |acc, x| acc + padded_size(x.len()));
        self.master_dir.entries.push(MasterDirEntry {
            offset: offset as u32,
            decomp_size: data.len() as u32,
            comp_size: compressed.len() as u32,
            name: path.clone(),
        });

        // Add the compressed file to the MASTER.DAT
        self.files.insert(path, compress(data));
    }

    /// Get a compressed file from the MASTER.DAT
    ///
    /// # Parameters
    ///
    /// - `path`: The path of the file to retrieve
    ///
    /// # Returns
    ///
    /// A copy of the bytes of the compressed file if it exists in the
    /// MASTER.DAT, otherwise `None`
    pub fn compressed_file(&self, path: &str) -> Option<Vec<u8>> {
        match self.files.get(path) {
            Some(f) => Some(f.clone()),
            _ => None,
        }
    }

    /// Get and decompress a file from the MASTER.DAT
    ///
    /// # Parameters
    ///
    /// - `path`: The path of the file to retrieve
    ///
    /// # Returns
    ///
    /// The decompressed bytes of the file if it exists in the MASTER.DAT,
    /// otherwise `None`
    pub fn decompressed_file(&self, path: &str) -> Option<Vec<u8>> {
        match self.files.get(path) {
            Some(f) => Some(decompress(&f)),
            _ => None,
        }
    }

    /// # Returns
    ///
    /// The filenames within the MASTER.DAT
    pub fn files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    /// Write the MASTER.DAT and its MASTER.DIR to new files
    ///
    /// # Parameters
    ///
    /// - `path`: The path for the destination file
    /// - `master_dir_path`: The path for the destination MASTER.DIR file
    pub fn write(&self, path: &Path, master_dir_path: &Path) {
        self.master_dir.write(&master_dir_path);
        let mut f = File::create(path).expect("unable to create file");
        for master_dir_entry in &self.master_dir.entries {
            f.write_all(&pad(self.files.get(&master_dir_entry.name).unwrap()))
                .expect("unable to write");
        }
    }
}

/// Get the padded size of a compressed file
///
/// # Parameters
///
/// - `size`: The size of the compressed file in bytes
///
/// # Returns
///
/// The size the compressed file would be after padding in the MASTER.DAT
fn padded_size(size: usize) -> usize {
    size + (2048 - (size % 2048))
}

/// Pads a compressed file from the MASTER.DAT
///
/// # Parameters
///
/// - `data`: The bytes to pad
///
/// # Returns
///
/// The padded bytes
fn pad(data: &[u8]) -> Vec<u8> {
    const PADDING: [u8; 4] = [b'S', b'H', b'A', b'B'];

    let mut padded: Vec<u8> = vec![];
    padded.extend(data);
    padded.extend(
        PADDING
            .iter()
            .cycle()
            .take(padded_size(data.len()) - data.len()),
    );

    padded
}
