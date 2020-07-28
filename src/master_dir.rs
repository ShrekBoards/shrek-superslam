use std::fs;
use std::path::PathBuf;

use crate::console::Console;

/// Type representing a single entry in the game's MASTER.DIR file, which in
/// turn describes a single compressed file in the MASTER.DAT file.
pub struct MasterDirEntry {
    /// The offset of the file in the MASTER.DAT file
    pub offset: u32,

    /// The decompressed size of the file
    pub decomp_size: u32,

    /// The compressed size of the file
    pub comp_size: u32,

    /// The path of the file, relative from data/
    pub name: String,
}

/// Type representing the MASTER.DIR file - a collection of MasterDirEntry
pub struct MasterDir {
    /// The entries within the MASTETR.DIR
    pub entries: Vec<MasterDirEntry>,
    
    /// The console this MASTER.DIR is from or for
    console: Console,
}

impl MasterDirEntry {
    /// Create a new MasterDirEntry
    ///
    /// # Parameters
    ///
    /// - `entry`: The bytes making up the entry in the MASTER.DIR file
    /// - `param`: console The console this version of the file is from
    ///
    /// # Returns
    ///
    /// A new MASTER.DIR entry from the provided bytes
    pub fn new(entry : &[u8], console : Console) -> MasterDirEntry {
        let offset = console.read32(&entry[0..4]);
        let decomp_size = console.read32(&entry[4..8]);
        let comp_size = console.read32(&entry[8..12]);

        MasterDirEntry {
            offset: offset,
            decomp_size: decomp_size,
            comp_size: comp_size,
            name: String::from_utf8(entry[12..].to_vec()).unwrap()
        }
    }
}

impl MasterDir {
    /// Creates an empty MASTER.DIR object
    ///
    /// # Parameters
    ///
    /// - `console`: The console this version of the file is for
    ///
    /// # Returns
    ///
    /// A new `MasterDir` of the enumerated MASTER.DIR entries
    pub fn new(console: Console) -> MasterDir {
        MasterDir {
            entries: vec!(),
            console: console,
        }
    }

    /// Creates a new MasterDir object from the passed bytes
    ///
    /// # Parameters
    ///
    /// - `master_dir`: The bytes of the entire MASTER.DIR file
    /// - `console`: The console this version of the file is from
    ///
    /// # Returns
    ///
    /// A new MasterDir of the enumerated MASTER.DIR entries
    pub fn from_bytes(master_dir: &Vec<u8>, console: Console) -> MasterDir { 
        // The MASTER.DIR is split into two sections:
        // * The first is a list of 4-byte integers that serve as offsets in the
        //   file to each entry in the second section. It is terminated by an entry
        //   that is just 0.
        // * The second section is a list of entries, each describing a compressed
        //   file within the MASTER.DAT file
        //
        // The first entry in the first section marks the start of the second
        // section. We can then read every 4-byte integer between the start of the
        // file and that offset, and use the int as an offset within the file to
        // read each section.
        let first_section_length = console.read32(&master_dir[0..4]);

        let mut entries : Vec<MasterDirEntry> = vec!();
        for index in (0..first_section_length).step_by(4) {
            let i = index as usize;

            // Using the offset to this section and the next section, determine the
            // size of this section to read. If the next section offset is 0, then
            // we are on the last entry, which runs until the end of the file
            let entry_offset = console.read32(&master_dir[i..i + 4]) as usize;
            if entry_offset == 0 {
                continue;
            }
            let next_entry_offset = console.read32(&master_dir[i + 4..i + 8]) as usize;
            let entry_length = match next_entry_offset {
                0 => master_dir.len() - entry_offset,
                _ => next_entry_offset - entry_offset
            };

            // Get the bytes of the entry using the offset and the size
            let entry_bytes = &master_dir[entry_offset..entry_offset + entry_length];
            entries.push(MasterDirEntry::new(entry_bytes, console));
        }

        MasterDir {
            entries: entries,
            console: console,
        }
    }

    /// Creates a new MasterDir object from the passed file
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the MASTER.DIR file to read
    /// - `console`: The console this version of the file is from
    ///
    /// # Returns
    ///
    /// A new MasterDir of the enumerated MASTER.DIR entries
    pub fn from_file(path: &PathBuf, console: Console) -> MasterDir {
        // Read all of the file to a byte array
        let file_contents = fs::read(&path).expect("unable to read master.dir");

        // Parse the bytes to a MasterDir object
        return MasterDir::from_bytes(&file_contents, console);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_entries_platform_pc() {
        let data = vec!(
            0x0C, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
            0x61, 0x62, 0x63, 0x64, 0x04, 0x00, 0x00, 0x00,
            0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
            0x65, 0x66, 0x67, 0x68,
        );

        let master_dir = MasterDir::from_bytes(&data, Console::PC);

        assert_eq!(master_dir.entries.len(), 2);

        assert_eq!(master_dir.entries[0].offset, 0x01);
        assert_eq!(master_dir.entries[0].decomp_size, 0x02);
        assert_eq!(master_dir.entries[0].comp_size, 0x03);
        assert_eq!(master_dir.entries[0].name, "abcd");

        assert_eq!(master_dir.entries[1].offset, 0x04);
        assert_eq!(master_dir.entries[1].decomp_size, 0x05);
        assert_eq!(master_dir.entries[1].comp_size, 0x06);
        assert_eq!(master_dir.entries[1].name, "efgh");
    }

    #[test]
    fn two_entries_platform_gcn() {
        let data = vec!(
            0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x1C,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x61, 0x62, 0x63, 0x64, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06,
            0x65, 0x66, 0x67, 0x68,
        );

        let master_dir = MasterDir::from_bytes(&data, Console::Gamecube);

        assert_eq!(master_dir.entries.len(), 2);

        assert_eq!(master_dir.entries[0].offset, 0x01);
        assert_eq!(master_dir.entries[0].decomp_size, 0x02);
        assert_eq!(master_dir.entries[0].comp_size, 0x03);
        assert_eq!(master_dir.entries[0].name, "abcd");

        assert_eq!(master_dir.entries[1].offset, 0x04);
        assert_eq!(master_dir.entries[1].decomp_size, 0x05);
        assert_eq!(master_dir.entries[1].comp_size, 0x06);
        assert_eq!(master_dir.entries[1].name, "efgh");
    }
}
