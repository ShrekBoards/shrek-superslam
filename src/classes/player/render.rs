use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::RenderSpawn` object type.
///
/// Holds some details about the player character during runtime, maybe
/// animation-related?
pub struct RenderSpawn {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for RenderSpawn {
    /// Returns the hashcode for the `Game::RenderSpawn` in-game object.
    fn hash() -> u32 {
        0xA6FC81A0
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::RenderSpawn"
    }

    /// Returns the size of a serialised `Game::RenderSpawn` object.
    fn size() -> usize {
        0x290
    }

    /// Return a new `RenderSpawn` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<RenderSpawn, Error> {
        Ok(RenderSpawn {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
