use crate::console::Console;
use crate::errors::Error;

/// Structure representing the header (the first 40 bytes) of a .bin file.
pub(super) struct BinHeader {
    /// The size in bytes of the top-level `gf::DB` object, including the
    /// object header and the database entries.
    pub gf_db_size: u32,

    /// The number of 'sections' the .bin file has.
    pub sections: u32,

    /// Unknown field.
    pub unknown: u32,

    /// The number of dependencies the .bin file has.
    pub dependencies: u32,

    /// The number of entries in the fourth section.
    pub offset4_count: u32,
}

impl BinHeader {
    /// The size in bytes of a .bin header.
    pub(super) const fn size() -> usize {
        0x40
    }

    /// Create a new BinHeader struct from the given `raw` header bytes from
    /// the given `console` platform.
    pub(super) fn new(raw: &[u8], console: Console) -> Result<BinHeader, Error> {
        Ok(BinHeader {
            gf_db_size: console.read_u32(&raw[0x10..0x14])?,
            sections: console.read_u32(&raw[0x18..0x1C])?,
            unknown: console.read_u32(&raw[0x1C..0x20])?,
            dependencies: console.read_u32(&raw[0x24..0x28])?,
            offset4_count: console.read_u32(&raw[0x2C..0x30])?,
        })
    }

    /// Get a representation of the header as raw bytes.
    pub(super) fn to_bytes(&self, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = vec![0x00; 0x10];            // 0x00 - 0x10
        bytes.extend(console.write_u32(self.gf_db_size)?);    // 0x10 - 0x14
        bytes.extend(vec![0x00; 4]);                          // 0x14 - 0x18
        bytes.extend(console.write_u32(self.sections)?);      // 0x18 - 0x1C
        bytes.extend(console.write_u32(self.unknown)?);       // 0x1C - 0x20
        bytes.extend(vec![0x00; 4]);                          // 0x20 - 0x24
        bytes.extend(console.write_u32(self.dependencies)?);  // 0x24 - 0x28
        bytes.extend(vec![0x00; 4]);                          // 0x28 - 0x2C
        bytes.extend(console.write_u32(self.offset4_count)?); // 0x2C - 0x30
        bytes.extend(vec![0x00; 0x10]);                       // 0x30 - 0x40

        assert_eq!(Self::size(), bytes.len());

        Ok(bytes)
    }
}