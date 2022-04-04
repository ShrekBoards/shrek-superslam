use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::DynamicThrowable` object type.
///
/// This class represents a single throwable item in a level.
pub struct DynamicThrowable {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for DynamicThrowable {
    /// Returns the hashcode for the `Game::DynamicThrowable` in-game object.
    fn hash() -> u32 {
        0xC8E0C03F
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::DynamicThrowable"
    }

    /// Returns the size of a serialised `Game::DynamicThrowable` object.
    fn size() -> usize {
        0x150
    }

    /// Return a new `DynamicThrowable` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<DynamicThrowable, Error> {
        Ok(DynamicThrowable {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
