use crate::console::Console;

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
    fn new(raw: &[u8], console: Console) -> LtimeshHeader {
        LtimeshHeader {
            entries: console.read_u32(&raw[0x08..0x0C]),
            begin_offset: console.read_u32(&raw[0x0C..0x10]),
        }
    }
}

/// Structure representing an entry in the .ltimesh file, which immediately
/// follows the header and describes a later section of the file.
struct LtimeshEntry {
    a: u32,
    b: u32,

    /// The offset of the section within the file
    offset: u32,
}

impl LtimeshEntry {
    /// Create a new LtimeshEntry structure from the passed `raw` bytes from
    /// the given `console` version.
    fn new(raw: &[u8], console: Console) -> LtimeshEntry {
        LtimeshEntry {
            a: console.read_u32(&raw[0x00..0x04]),
            b: console.read_u32(&raw[0x04..0x08]),
            offset: console.read_u32(&raw[0x08..0x0C]),
        }
    }

    /// Return the size of an LtimeshEntry within the file.
    const fn size() -> usize {
        0x0C
    }
}

/// Structure representing an .ltimesh file, and the entries contained within
/// it.
pub struct Ltimesh {
    /// The header of the .ltimesh file.
    header: LtimeshHeader,

    /// Each of the entries immediately following the header.
    entries: Vec<LtimeshEntry>,
}

impl Ltimesh {
    /// Construct an Ltimesh from the passed `raw` bytes from the given `console`
    /// version.
    pub fn from_bytes(raw: &[u8], console: Console) -> Ltimesh {
        // Read the header
        let header = LtimeshHeader::new(&raw[0x00..0x20], console);

        // Parse each entry
        let entries: Vec<LtimeshEntry> = (0..header.entries as usize)
            .map(|i| {
                let begin = (i * LtimeshEntry::size()) + header.begin_offset as usize;
                let end = begin + LtimeshEntry::size();
                LtimeshEntry::new(&raw[begin..end], console)
            })
            .collect();

        Ltimesh {
            header,
            entries,
        }
    }
}