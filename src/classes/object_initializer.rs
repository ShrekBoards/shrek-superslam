use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::ObjectInitializer` object type.
///
/// This class initialises objects?
pub struct ObjectInitializer {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for ObjectInitializer {
    /// Returns the hashcode for the `Game::ObjectInitializer` in-game object.
    fn hash() -> u32 {
        0xDBFB4A35
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::ObjectInitializer"
    }

    /// Returns the size of a serialised `Game::ObjectInitializer` object.
    fn size() -> usize {
        0xA8
    }

    /// Return a new `ObjectInitializer` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<ObjectInitializer, Error> {
        Ok(ObjectInitializer {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
