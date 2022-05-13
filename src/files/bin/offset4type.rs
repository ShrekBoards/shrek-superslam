use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

use crate::console::Console;
use crate::errors::Error;

/// Length of the string within the structure.
const STRLEN: usize = 0x3C;

/// Structure that comes after the dependencies in the .bin file, don't yet
/// know what it does.
pub(super) struct BinOffset4Struct {
    /// Some kind of string.
    pub s: String,

    /// Some kind of number.
    pub n: u32,
}

impl BinOffset4Struct {
    /// Get the size in bytes of a single entry.
    pub(super) const fn size() -> usize {
        0x40
    }

    /// Create a collection of BinOffset4Struct structs from the given `raw`
    /// section bytes from a .bin file for the given `console` platform.
    pub(super) fn new(raw: &[u8], console: Console) -> Result<Vec<BinOffset4Struct>, Error> {
        raw
            .chunks(Self::size())
            .map(|dependecy_bytes| {
                Ok(BinOffset4Struct {
                    s: ISO_8859_1.decode(&dependecy_bytes[0x00..STRLEN].to_vec(), DecoderTrap::Strict)?,
                    n: console.read_u32(&dependecy_bytes[STRLEN..STRLEN + 4])?,
                })
            })
            .collect()
    }

    /// Convert the structure back to a byte representation.
    pub(super) fn to_bytes(&self, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes = ISO_8859_1.encode(&self.s, EncoderTrap::Strict)?;
        bytes.resize(STRLEN, 0x00);

        bytes.extend(console.write_u32(self.n)?);

        assert_eq!(Self::size(), bytes.len());

        Ok(bytes)
    }
}