use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::WeaponType` object type.
///
/// This class represents a single weapon type.
pub struct WeaponType {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for WeaponType {
    /// Returns the hashcode for the `Game::WeaponType` in-game object.
    fn hash() -> u32 {
        0xFE392AB6
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::WeaponType"
    }

    /// Returns the size of a serialised `Game::WeaponType` object.
    fn size() -> usize {
        0x120
    }

    /// Return a new `WeaponType` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<WeaponType, Error> {
        Ok(WeaponType {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
