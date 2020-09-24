use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::files::Bin;

/// Structure representing the in-game `gf::LocalizedString` type, which is a
/// thin wrapper around regular string types.
pub struct LocalizedString {
    /// The contents of the string
    pub string: String,

    /// Unknown value that resides at +04, seems to be 0 if the string is empty
    unknown: u32,
}

/// Structure representing the in-game `Game::EffectStringReference` type,
/// which is a thin wrapper around a string that names an effect.
pub struct EffectStringReference {
    /// The contents of the string
    pub string: String,
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
    fn new(bin: &Bin, offset: usize) -> LocalizedString {
        let x = bin.console.read_u32(&bin.raw[offset + 0x04..offset + 0x08]);
        let str_offset = bin.console.read_u32(&bin.raw[offset + 0x08..offset + 0x0C]);
        LocalizedString {
            string: bin.get_str_from_offset(str_offset).unwrap(),
            unknown: x,
        }
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
    fn new(bin: &Bin, offset: usize) -> EffectStringReference {
        let str_offset = bin.console.read_u32(&bin.raw[offset + 0x04..offset + 0x08]);
        EffectStringReference {
            string: bin.get_str_from_offset(str_offset).unwrap(),
        }
    }
}