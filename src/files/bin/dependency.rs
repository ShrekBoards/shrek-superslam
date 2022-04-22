use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

use crate::console::Console;
use crate::errors::Error;

/// Length of both strings within the structure.
const STRLEN: usize = 0x3C;

/// Structure describing the dependency of a .bin file to another .bin file.
pub(super) struct BinDependency {
    /// The path within the MASTER.DAT to the dependency.
    pub path: String,

    /// Another string.
    pub desc: String,

    /// Some kind of offset.
    pub offset: u32,
}

impl BinDependency {
    /// Get the size in bytes of a single dependency descriptor.
    pub(super) const fn size() -> usize {
        0x80
    }

    /// Create a collection of BinDependency structs from the given `raw` section
    /// bytes from a .bin file for the given `console` platform.
    pub(super) fn new(raw: &[u8], console: Console) -> Result<Vec<BinDependency>, Error> {
        raw
            .chunks(Self::size())
            .map(|dependecy_bytes| {
                Ok(BinDependency {
                    path: ISO_8859_1.decode(&dependecy_bytes[0x00..STRLEN].to_vec(), DecoderTrap::Strict)?,
                    desc: ISO_8859_1.decode(&dependecy_bytes[STRLEN..STRLEN * 2].to_vec(), DecoderTrap::Strict)?,
                    offset: console.read_u32(&dependecy_bytes[0x7C..0x80])?,
                })
            })
            .collect()
    }

    /// Convert the structure back to a byte representation.
    pub(super) fn to_bytes(&self, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes = ISO_8859_1.encode(&self.path, EncoderTrap::Strict)?;
        bytes.resize(STRLEN, 0x00);

        let mut second_str = ISO_8859_1.encode(&self.desc, EncoderTrap::Strict)?;
        second_str.resize(STRLEN, 0x00);
        bytes.extend(second_str);

        bytes.extend(vec![0x00; 4]);
        bytes.extend(console.write_u32(self.offset)?);

        assert_eq!(Self::size(), bytes.len());

        Ok(bytes)
    }
}