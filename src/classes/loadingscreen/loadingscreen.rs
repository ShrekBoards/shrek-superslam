use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::LoadingScreen` object type.
///
/// This class a loading screen for a stage or character.
pub struct LoadingScreen {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for LoadingScreen {
    /// Returns the hashcode for the `Game::LoadingScreen` in-game object.
    fn hash() -> u32 {
        0xF32EBBA9
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::LoadingScreen"
    }

    /// Returns the size of a serialised `Game::LoadingScreen` object.
    fn size() -> usize {
        0x8C
    }

    /// Return a new `LoadingScreen` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<LoadingScreen, Error> {
        Ok(LoadingScreen {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
