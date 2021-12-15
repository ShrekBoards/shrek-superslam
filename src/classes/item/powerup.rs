use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::PowerupType` object type.
///
/// This class represents a single powerup type.
pub struct PowerupType {
}

impl SerialisedShrekSuperSlamGameObject for PowerupType {
    /// Returns the hashcode for the `Game::PowerupType` in-game object.
    fn hash() -> u32 {
        0xBE7B44BA
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::PowerupType"
    }

    /// Returns the size of a serialised `Game::PowerupType` object.
    fn size() -> usize {
        0xE4
    }

    /// Return a new `PowerupType` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<PowerupType, Error> {
        Ok(PowerupType {})
    }
}