use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `anim::LookAtData` object type.
///
/// Represents some kind of animation detail.
pub struct LookAtData {
    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for LookAtData {
    /// Returns the hashcode for the `anim::LookAtData` in-game object.
    fn hash() -> u32 {
        0xD9BB3F0F
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "anim::LookAtData"
    }

    /// Returns the size of a serialised `anim::LookAtData` object.
    fn size() -> usize {
        0x24
    }

    /// Return a new `LookAtData` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<LookAtData, Error> {
        Ok(LookAtData {
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
