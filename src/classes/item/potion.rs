use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::PotionType` object type.
///
/// This class represents a single potion type.
pub struct PotionType {}

impl SerialisedShrekSuperSlamGameObject for PotionType {
    /// Returns the hashcode for the `Game::PotionType` in-game object.
    fn hash() -> u32 {
        0xF05C7BD3
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::PotionType"
    }

    /// Returns the size of a serialised `Game::PotionType` object.
    fn size() -> usize {
        0x100
    }

    /// Return a new `PotionType` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<PotionType, Error> {
        Ok(PotionType {})
    }
}
