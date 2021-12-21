use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::Entity` object type.
///
/// Used as the entry point to all the runtime information about a character.
pub struct Entity {
}

impl SerialisedShrekSuperSlamGameObject for Entity {
    /// Returns the hashcode for the `Game::Entity` in-game object.
    fn hash() -> u32 {
        0xDDEC024E
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::Entity"
    }

    /// Returns the size of a serialised `Game::Entity` object.
    fn size() -> usize {
        0x280
    }

    /// Return a new `Entity` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<Entity, Error> {
        Ok(Entity {})
    }
}