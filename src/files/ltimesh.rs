use std::fs;
use std::path::Path;

use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};

use crate::console::Console;
use crate::errors::Error;

/// Structure representing the header of an .ltimesh file
struct LtimeshHeader {
    /// Number of entries in the ltimesh.
    entries: u32,

    /// The offset within the whole file that the entries start at.
    begin_offset: u32,
}

impl LtimeshHeader {
    /// Create a new LtimeshHeader structure from the passed `raw` bytes from
    /// the given `console` version.
    fn new(raw: &[u8], console: Console) -> Result<LtimeshHeader, Error> {
        Ok(LtimeshHeader {
            entries: console.read_u32(&raw[0x08..0x0C])?,
            begin_offset: console.read_u32(&raw[0x0C..0x10])?,
        })
    }
}

/// Enumeration of the different types of object within an .ltimesh.
#[derive(Copy, Clone, PartialEq)]
pub enum LtimeshEntryType {
    A,
    B,
    C,
}

/// Structure representing an entry in the .ltimesh file, which immediately
/// follows the header and describes a later section of the file.
struct LtimeshEntry {
    //a: u16,
    //b: u8,

    // The type of data the entry refers to.
    entry_type: LtimeshEntryType,

    // Entry to the data of the entry, after the name at the beginning.
    data_offset: u32,

    /// The offset of the section within the file, which starts with the name.
    offset: u32,
}

impl LtimeshEntry {
    /// Create a new LtimeshEntry structure from the passed `raw` bytes from
    /// the given `console` version.
    fn new(raw: &[u8], console: Console) -> Result<LtimeshEntry, Error> {
        Ok(LtimeshEntry {
            //a: console.read_u32(&raw[0x00..0x04]),
            //b: raw[0x02],
            entry_type: match raw[0x03] {
                0x01 => LtimeshEntryType::A,
                0x02 => LtimeshEntryType::B,
                0x03 => LtimeshEntryType::C,
                _ => panic!("unknown type"),
            },
            data_offset: console.read_u32(&raw[0x04..0x08])?,
            offset: console.read_u32(&raw[0x08..0x0C])?,
        })
    }

    /// Return the size of an LtimeshEntry within the file.
    const fn size() -> usize {
        0x0C
    }
}

/// Structure representing a single 'file' (data?) contained within an .ltimesh
/// file, referenced by an individual LtimeshEntry.
pub struct LtimeshFile {
    /// The hash of the name of the file.
    pub hash: u32,

    /// The name of the file.
    pub name: String,

    /// The remaining data of the file.
    pub data: Vec<u8>,

    /// The type of the file.
    pub file_type: LtimeshEntryType,
}

impl LtimeshFile {
    /// Create a new LtimeshFile structure of the given `file_type` from the passed
    /// `raw` bytes from the given `console` version. The `data_offset` is the offset
    /// from the start of the raw bytes passed that the name section ends and the rest
    /// of the data begins.
    fn new(
        raw: &[u8],
        file_type: LtimeshEntryType,
        data_offset: usize,
        console: Console,
    ) -> Result<LtimeshFile, Error> {
        // These fields are constant no matter the type
        let hash = console.read_u32(&raw[0x00..0x04])?;
        let name = ISO_8859_1.decode(&raw[0x04..data_offset].to_vec(), DecoderTrap::Strict)?.trim_end_matches(char::from(0)).to_owned();

        // From there, it depends on the file type
        if file_type == LtimeshEntryType::A {
            // The first entry in the data is a backreference to the top,
            // where the hash is.
            let backreference = console.read_u32(&raw[data_offset..data_offset+0x04])?;

            // The next entry is a pointer to later in the file
            let later_reference = console.read_u32(&raw[data_offset+0x04..data_offset+0x08])?;
        } else if file_type == LtimeshEntryType::B {
            // The first entry in the data is an backreference offset from the
            // base of the file to the start of this file, which we already knew
            // from the header entry offset value. The game adds this offset
            // to the base address of the .ltimesh file, so this becomes a pointer
            // to itself.
            let backreference = console.read_u32(&raw[data_offset..data_offset+0x04])?;

            // Next, using a vtable the game works out to call sub_5C5A60, passing
            // the address of the beginning of the data (which was just written to
            // with the address of the start of the entry, 32 bytes back).
        }

        Ok(LtimeshFile {
            hash,
            name,
            data: raw[data_offset..].to_vec(),
            file_type
        })
    }
}

/// Structure representing an .ltimesh file, and the entries contained within
/// it.
pub struct Ltimesh {
    /// The header of the .ltimesh file.
    _header: LtimeshHeader,

    /// Each of the entries immediately following the header.
    _entries: Vec<LtimeshEntry>,

    /// Each of the files within the .ltimesh file.
    pub files: Vec<LtimeshFile>,
}

impl Ltimesh {
    /// Construct an Ltimesh from the passed `raw` bytes from the given `console`
    /// version.
    pub fn from_bytes(raw: &[u8], console: Console) -> Result<Ltimesh, Error> {
        // Read the header
        let header = LtimeshHeader::new(&raw[0x00..0x20], console)?;

        // Parse each entry
        let entries: Result<Vec<LtimeshEntry>, Error> = (0..header.entries as usize)
            .map(|i| {
                let begin = (i * LtimeshEntry::size()) + header.begin_offset as usize;
                let end = begin + LtimeshEntry::size();
                LtimeshEntry::new(&raw[begin..end], console)
            })
            .collect();
        let entries = entries?;

        // Use the entries to create a structure for each file
        let mut files: Vec<LtimeshFile> = vec![];
        for (i, entry) in entries.iter().enumerate() {
            // We need to know the offset of the next entry to determine where
            // this entry ends. If this is the last entry, then it ends at the
            // end of the file.
            let file_raw_bytes = if i < (entries.len() - 1) {
                &raw[entry.offset as usize..entries[i + 1].offset as usize]
            } else {
                &raw[entry.offset as usize..]
            };

            println!("offset: {:08X}, 2nd offset: {:08X}", entry.offset, entry.data_offset);

            files.push(LtimeshFile::new(
                file_raw_bytes,
                entry.entry_type,
                (entry.data_offset - entry.offset) as usize,
                console,
            )?);
        }

        Ok(Ltimesh {
            _header: header,
            _entries: entries,
            files,
        })
    }

    pub fn from_file(path: &Path, console: Console) -> Result<Ltimesh, Error> {
        let file_contents = fs::read(&path)?;
        Ltimesh::from_bytes(&file_contents, console)
    }
}
