use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use crate::compression::{compress, decompress};
use crate::console::Console;
use crate::master_dir::{MasterDir, MasterDirEntry};

/// Structure representing the MASTER.DAT file, which contains all of the
/// compressed files for Shrek SuperSlam.
pub struct MasterDat {
    /// Mapping of the filenames within the MASTER.DAT to the compressed file data
    files: HashMap<String, Vec<u8>>,

    /// The associated MASTER.DIR file
    master_dir: MasterDir,
}

impl MasterDat {
    /// Returns a new empty `MasterDat` object for the given `console`.
    ///
    /// # Example
    ///
    /// ```
    /// use shrek_superslam::{Console, MasterDat};
    ///
    /// let master_dat = MasterDat::new(Console::PC);
    /// ```
    pub fn new(console: Console) -> MasterDat {
        MasterDat {
            files: HashMap::new(),
            master_dir: MasterDir::new(console),
        }
    }

    /// Load an existing MASTER.DAT from the bytes that make up the file, using
    /// the given `master_dir` file for the mapping.
    pub fn from_bytes(master_dat: &[u8], master_dir: MasterDir) -> MasterDat {
        // Iterate over the entries within the associated MASTER.DIR, and use it
        // to read out each compressed file from the MASTER.DAT
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        for entry in &master_dir.entries {
            let o = entry.offset as usize;
            let file = master_dat[o..o + entry.comp_size as usize].to_owned();
            files.insert(entry.name.trim_end_matches(char::from(0)).to_owned(), file);
        }

        MasterDat { files, master_dir }
    }

    /// Load an existing MASTER.DAT file from the given `path`, using the given
    /// `master_dir` file.
    ///
    /// # Errors
    ///
    /// Returns an `Err(std::io::Error)` if there is an error reading the file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    ///
    /// let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// ```
    pub fn from_file(path: &Path, master_dir: MasterDir) -> Result<MasterDat, Error> {
        // Read all of the file to a byte array
        let file_contents = fs::read(&path)?;

        // Parse the bytes to a MasterDir object
        Ok(MasterDat::from_bytes(&file_contents, master_dir))
    }

    /// Add a new file at the given `path` with the given `data` to the
    /// MASTER.DAT.
    ///
    /// # Example
    ///
    /// ```
    /// use shrek_superslam::{Console, MasterDat};
    ///
    /// let mut master_dat = MasterDat::new(Console::PC);
    /// master_dat.add_file("data\\test.dds".to_string(), &Vec::new());
    /// ```
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

    /// Returns the compressed file at the given `path` in the MASTER.DAT if
    /// it exists.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    ///
    /// let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// let compressed_file = master_dat.compressed_file("data\\players\\shrek\\player.db.bin").unwrap();
    /// ```
    pub fn compressed_file(&self, path: &str) -> Option<Vec<u8>> {
        match self.files.get(path) {
            Some(f) => Some(f.clone()),
            _ => None,
        }
    }

    /// Returns the decompressed file at the given `path` in the MASTER.DAT if
    /// it exists.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    ///
    /// let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// let decompressed_file = master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap();
    /// ```
    pub fn decompressed_file(&self, path: &str) -> Option<Vec<u8>> {
        match self.files.get(path) {
            Some(f) => Some(decompress(&f)),
            _ => None,
        }
    }

    /// Returns the filenames within the MASTER.DAT file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use shrek_superslam::{Console, MasterDat, MasterDir};
    /// #
    /// # let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// for filename in master_dat.files() {
    ///     let decompressed_file = master_dat.decompressed_file(&filename).unwrap();
    ///     println!("{} decompressed size {}", filename, decompressed_file.len());
    /// }
    /// ```
    pub fn files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    /// Returns the raw bytes of the MASTER.DAT file and its associated MASTER.DIR.
    ///
    /// # Remarks
    ///
    /// The returned tuple contains the MASTER.DAT first, then the MASTER.DIR bytes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::{Console, MasterDat};
    ///
    /// let mut master_dat = MasterDat::new(Console::PC);
    /// master_dat.add_file("data\\test.dds".to_string(), &Vec::new());
    /// let (master_dat_bytes, master_dir_bytes) = master_dat.to_bytes();
    /// ```
    pub fn to_bytes(&self) -> (Vec<u8>, Vec<u8>) {
        let mut master_dat_bytes = vec![];
        for master_dir_entry in &self.master_dir.entries {
            let trimmed = master_dir_entry.name.trim_end_matches(char::from(0));
            master_dat_bytes.extend(&pad(self.files.get(trimmed).unwrap()));
        }

        (master_dat_bytes, self.master_dir.to_bytes())
    }

    /// Update a file located at `path` contained within the MASTER.DAT with
    /// the new supplied `data`.
    ///
    /// # Notes
    ///
    /// Currently, the size new `data` must be the same size as the file it is
    /// replacing.
    ///
    /// # Errors
    ///
    /// Gives an `Err(usize)` with the size of the file being replaced, if the
    /// size of the supplied `data` does not match.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    /// use shrek_superslam::classes::attacks::AttackMoveType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Read the MASTER.DAT and MASTER.DIR pair
    /// let mut master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let mut master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    ///
    /// // Parse the Shrek character player.db.bin file, and get the last attack
    /// let mut bin = Bin::new(
    ///     master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap(),
    ///     Console::PC
    /// );
    /// let mut attacks = bin.get_all_objects_of_type::<AttackMoveType>();
    /// let (offset, mut attack) = attacks.pop().unwrap();
    ///
    /// // Modify the contents of the attack
    /// attack.damage1 = 100.0;
    ///
    /// // Write the new attack back to the .bin file
    /// bin.overwrite_object(offset, &attack);
    ///
    /// // Write the updated .bin file back to the MASTER.DAT
    /// master_dat.update_file("data\\players\\shrek\\player.db.bin", bin.raw()).unwrap();
    ///
    /// // Write the updated MASTER.DAT and MASTER.DIR pair to disk
    /// master_dat.write(Path::new("MASTER.DAT"), Path::new("MASTER.DIR"));
    ///
    /// // We have now overwritten the damage of Shrek's last attack!
    /// ```
    pub fn update_file(&mut self, path: &str, data: &[u8]) -> Result<(), usize> {
        // Ensure the file to replace exists in the first place
        let existing_length = match self.decompressed_file(path) {
            Some(f) => f.len(),
            _ => 0,
        };

        // Ensure the filesize of the new file is the same as the old
        if data.len() != existing_length {
            return Err(existing_length);
        }

        // Since our compression algorithm is not a perfect recreation of the
        // original, we will need to update the MASTER.DIR's record of the
        // compressed size too
        let compressed = compress(data);
        if let Some(e) = self
            .master_dir
            .entries
            .iter_mut()
            .find(|e| e.name.trim_end_matches(char::from(0)) == path)
        {
            e.comp_size = compressed.len() as u32;

            // Having updated the size of this entry, the offsets to all
            // subsequent entries will now be invalidated, as the new file
            // is larger and will take up more space. We therefore recalculate
            // each offset (it's easier to redo them all)
            let mut total_size: u32 = 0;
            for entry in self.master_dir.entries.iter_mut() {
                entry.offset = total_size;
                total_size += padded_size(entry.comp_size as usize) as u32;
            }
        }

        // Update the contents of the existing file
        self.files.insert(path.to_string(), compress(data));

        Ok(())
    }

    /// Write the MASTER.DAT to the `path` given, and its paired MASTER.DIR to
    /// the given `master_dir_path`.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a problem writing to either file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat};
    ///
    /// let mut master_dat = MasterDat::new(Console::PC);
    /// master_dat.add_file("data\\test.dds".to_string(), &Vec::new());
    /// master_dat.write(Path::new("MASTER.DAT"), Path::new("MASTER.DIR"));
    /// ```
    pub fn write(&self, path: &Path, master_dir_path: &Path) -> Result<(), Error> {
        let (master_dat_bytes, master_dir_bytes) = self.to_bytes();

        // Write the MASTER.DAT
        let mut master_dat_outfile = File::create(path)?;
        master_dat_outfile.write_all(&master_dat_bytes)?;

        // Write the MASTER.DIR
        let mut master_dir_outfile = File::create(master_dir_path)?;
        master_dir_outfile.write_all(&master_dir_bytes)?;

        Ok(())
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
const fn padded_size(size: usize) -> usize {
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
