use encoding::all::ISO_8859_1;
use encoding::{Encoding, EncoderTrap};

use crate::console::Console;
use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `gf::LocalizedString` object type.
///
/// This type is a thin wrapper around regular string types.
pub struct LocalizedString {
    /// The contents of the string
    pub string: String,

    /// Unknown value that resides at +04, seems to be 0 if the string is empty
    unknown: u32,

    /// The raw bytes of the object.
    bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for LocalizedString {
    /// # Returns
    ///
    /// The hashcode for the gf::LocalizedString in-game object
    fn hash() -> u32 {
        0xBFC7788D
    }

    /// # Returns
    ///
    /// The name of the in-game class - "gf::LocalizedString"
    fn name() -> &'static str {
        "gf::LocalizedString"
    }

    /// # Returns
    ///
    /// The size of a serialised gf::LocalizedString object
    fn size() -> usize {
        0x0C
    }

    /// Constructor
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin containing the object
    /// - `offset`: The offset the object begins at within the .bin file
    fn new(bin: &Bin, offset: usize) -> Result<LocalizedString, Error> {
        let c = bin.console;
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();
        let x = c.read_u32(&bytes[0x04..0x08])?;
        let str_offset = c.read_u32(&bytes[0x08..0x0C])?;
        Ok(LocalizedString {
            string: bin.get_str_from_offset(str_offset)?,
            unknown: x,
            bytes: bytes,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for LocalizedString {
    /// Get the byte representation of the gf::LocalizedString object.
    fn to_bytes(&self, offset: usize, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();

        // Steal values +00 through to +08 from the original.
        bytes.extend(&self.bytes[0x00..0x08]);

        // The offset to the string is at +0C.
        // We will place the contents of the string immediately after the
        // object content, so the offset to the string is the given offset plus
        // the size of the object.
        bytes.extend(console.write_u32((offset + 0x0C) as u32)?);

        // Write the string immediately after, encoded as ISO 8859-1.
        bytes.extend(ISO_8859_1.encode(&self.string, EncoderTrap::Strict)?);

        Ok(bytes)
    }
}

impl LocalizedString {
    /// Determine if a gf::LocalizedString is empty or not
    ///
    /// # Returns
    ///
    /// True if the string is empty, otherwise false
    pub fn is_empty(&self) -> bool {
        self.unknown == 0
    }
}

/// Structure representing the in-game `Game::EffectStringReference` object type.
///
/// This type is a thin wrapper around a string that names an effect.
pub struct EffectStringReference {
    /// The contents of the string.
    pub string: String,

    /// The raw bytes of the object.
    bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for EffectStringReference {
    /// # Returns
    ///
    /// The hashcode for the Game::EffectStringReference in-game object
    fn hash() -> u32 {
        0xC43D420D
    }

    /// # Returns
    ///
    /// The name of the in-game class - "Game::EffectStringReference"
    fn name() -> &'static str {
        "Game::EffectStringReference"
    }

    /// # Returns
    ///
    /// The size of a serialised Game::EffectStringReference object
    fn size() -> usize {
        0x0C
    }

    /// Constructor
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin containing the object
    /// - `offset`: The offset the object begins at within the .bin file
    fn new(bin: &Bin, offset: usize) -> Result<EffectStringReference, Error> {
        let c = bin.console;
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();
        let str_offset = c.read_u32(&bytes[0x04..0x08])?;
        Ok(EffectStringReference {
            string: bin.get_str_from_offset(str_offset)?,
            bytes,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for EffectStringReference {
    /// Get the byte representation of the Game::EffectStringReference object.
    fn to_bytes(&self, offset: usize, console: Console) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();

        // Steal values +00 through to +08 from the original.
        bytes.extend(&self.bytes[0x00..0x08]);

        // The offset to the string is at +0C.
        // We will place the contents of the string immediately after the
        // object content, so the offset to the string is the given offset plus
        // the size of the object.
        bytes.extend(console.write_u32((offset + 0x0C) as u32)?);

        // Write the string immediately after, encoded as ISO 8859-1.
        bytes.extend(ISO_8859_1.encode(&self.string, EncoderTrap::Strict)?);

        Ok(bytes)
    }
}