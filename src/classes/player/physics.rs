use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::PhysicsFighting` object type.
///
/// This object is mostly used to store a character's values at runtime.
pub struct PhysicsFighting {
}

impl SerialisedShrekSuperSlamGameObject for PhysicsFighting {
    /// Returns the hashcode for the `Game::PhysicsFighting` in-game object.
    fn hash() -> u32 {
        0xADDDF1EC
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::PhysicsFighting"
    }

    /// Returns the size of a serialised `Game::PhysicsFighting` object.
    fn size() -> usize {
        0xD50
    }

    /// Return a new `PhysicsFighting` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<PhysicsFighting, Error> {
        Ok(PhysicsFighting {})
    }
}