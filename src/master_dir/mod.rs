use std::str;

use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

use crate::console::Console;

/// Type representing a single entry in the game's MASTER.DIR file, which in
/// turn describes a single compressed file in the MASTER.DAT file.
pub struct MasterDirEntry {
    pub offset : u32,      // The offset of the file in the MASTER.DAT file
    pub malloc_size : u32, // A number related to the decompressed size
    pub orig_size : u32,   // The compressed size of the file
    pub name : String,     // The path of the file, relative from data/
}

/// Type representing the MASTER.DIR file - a collection of MasterDirEntry
pub struct MasterDir {
    pub entries: Vec<MasterDirEntry>,
}

impl MasterDirEntry {
    /// Create a new MasterDirEntry
    ///
    /// \param entry   The bytes making up the entry in the MASTER.DIR file
    /// \param console The console this version of the file is from
    ///
    /// \returns A new MASTER.DIR entry from the provided bytes
    pub fn new(entry : &[u8], console : &Console) -> MasterDirEntry {

        let offset = match console {
            Console::Gamecube => (&entry[0..4]).read_u32::<BigEndian>().unwrap(),
            _ => (&entry[0..4]).read_u32::<LittleEndian>().unwrap()
        };

        let malloc_size = match console {
            Console::Gamecube => (&entry[4..8]).read_u32::<BigEndian>().unwrap(),
            _ => (&entry[4..8]).read_u32::<LittleEndian>().unwrap()
        };

        let orig_size = match console {
            Console::Gamecube => (&entry[8..12]).read_u32::<BigEndian>().unwrap(),
            _ => (&entry[8..12]).read_u32::<LittleEndian>().unwrap()
        };

        MasterDirEntry {
            offset: offset,
            malloc_size: malloc_size,
            orig_size: orig_size,
            name: str::from_utf8(&entry[12..]).unwrap().to_string()
        }
    }
}

impl MasterDir {
    /// Creates a new MasterDir object from the MASTER.DIR file
    ///
    /// \param master_dir The bytes of the entire MASTER.DIR file
    /// \param console    The console this version of the file is from
    ///
    /// \returns A new MasterDir of the enumerated MASTER.DIR entries
    pub fn new(master_dir : Vec<u8>, console : &Console) -> MasterDir { 

        // The file begins as a series of 32-bit offsets to the real entries
        // later in the file. Reading the first one tells us how big this
        // section is (since the second section begins at the first offset).
        let mut offsets : Vec<u32> = vec!();
        let first_offset = match console {
            Console::Gamecube => (&master_dir[0..4]).read_u32::<BigEndian>().unwrap(),
            _ => (&master_dir[0..4]).read_u32::<LittleEndian>().unwrap()
        };
        offsets.push(first_offset);

        // We then read the file up to the point described in the first offset
        // to retrieve the others
	for mut offset in master_dir[4..(first_offset as usize)].chunks(4) {
            offsets.push(match console {
                Console::Gamecube => offset.read_u32::<BigEndian>().unwrap(),
                _ => offset.read_u32::<LittleEndian>().unwrap()
            });
        }
        offsets.pop();

        // These offsets can now be used to index each entry within the file
        let mut entries : Vec<MasterDirEntry> = vec!();
        let mut it = offsets.iter().enumerate().peekable();
        while let Some((_, offset)) = it.next() {
            let lower = *offset as usize;
            let upper = match it.peek() {
                Some((_, x)) => (**x - 1) as usize,
                _ => (master_dir.len() - 1) as usize
            };
            let entry_bytes = &master_dir[lower..upper];

            entries.push(MasterDirEntry::new(entry_bytes, &console));
        }

        MasterDir {
            entries: entries,
        }
    }
}
