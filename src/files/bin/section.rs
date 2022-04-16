use crate::console::Console;
use crate::errors::Error;

/// Poorly-named struct that represents the description a 'section' within a
/// .bin - a small 16-byte area that describes and points to a big list of
/// offsets to entries of a certain type within the file.
pub(super) struct BinSection {
    /// Determines the type of each thing being pointed to
    pub number: u32,

    /// The number of pointers in the section.
    pub count: u32,

    /// The offset the collection of pointers begins within the file.
    pub offset: u32,
}

impl BinSection {
    /// Get the size in bytes of a single section.
    pub(super) const fn size() -> usize {
        0x10
    }

    /// Create a collection of BinSection structs from the given `raw` section
    /// bytes from a .bin file for the given `console` platform.
    pub(super) fn new(raw: &[u8], console: Console) -> Result<Vec<BinSection>, Error> {
        raw
            .chunks(Self::size())
            .map(|section_bytes| {
                Ok(BinSection {
                    number: console.read_u32(&section_bytes[0x00..0x04])?,
                    count: console.read_u32(&section_bytes[0x04..0x08])?,
                    offset: 0,
                })
            })
            .collect()
    }

    /// Convert the structure back to a byte representation.
    pub(super) fn to_bytes(&self, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes = console.write_u32(self.number)?; // 0x00 - 0x04
        bytes.extend(console.write_u32(self.count)?);    // 0x04 - 0x08
        bytes.extend(console.write_u32(self.offset)?);   // 0x08 - 0x0C
        bytes.extend(vec![0x00; 4]);                     // 0x0C - 0x10

        assert_eq!(Self::size(), bytes.len());

        Ok(bytes)
    }
}