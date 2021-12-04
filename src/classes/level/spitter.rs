use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::Spitter` object type.
///
/// A 'spitter' is what players get slammed into, and are responsible for
/// spawning the player back onto the battlefield, and the slam event
/// animations.
pub struct Spitter {
    /// The keyframes of the spitter.
    pub keyframes: Vec<SpitterKeyFrame>,
}

impl SerialisedShrekSuperSlamGameObject for Spitter {
    /// Returns the hashcode for the `Game::Spitter` in-game object.
    fn hash() -> u32 {
        0x90D8FCD6
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::Spitter"
    }

    /// Returns the size of a serialised `Game::Spitter` object.
    fn size() -> usize {
        0xE0
    }

    /// Return a new `Spitter` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling
    /// [`Bin::get_object_from_offset`]
    /// rather than calling this method.
    fn new(bin: &Bin, offset: usize) -> Result<Spitter, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // The list of keyframes is at offset +20.
        // The count of keyframes is at offset +24.
        let keyframe_list_offset = c.read_u32(&raw[offset + 0x20..offset + 0x24])? as usize;
        let keyframes_count = c.read_u32(&raw[offset + 0x24..offset + 0x28])? as usize;
        let keyframes: Result<Vec<SpitterKeyFrame>, Error> =
            (0..keyframes_count)
                .map(|i| {
                    let keyframe_list_entry_offset = (Bin::header_length() + keyframe_list_offset + (i * 4)) as usize; 
                    c.read_u32(&raw[keyframe_list_entry_offset..keyframe_list_entry_offset + 4])
                })
                .map(|offset| { bin.get_object_from_offset::<SpitterKeyFrame>(offset?) })
                .collect();

        Ok(Spitter { keyframes: keyframes?, })
    }
}

/// Structure representing the in-game `Game::SpitterKeyFrame` object type.
///
/// This represents an individual keyframe within a spitter animation.
pub struct SpitterKeyFrame {
}

impl SerialisedShrekSuperSlamGameObject for SpitterKeyFrame {
    /// Returns the hashcode for the `Game::SpitterKeyFrame` in-game object.
    fn hash() -> u32 {
        0x84AD7E70
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::SpitterKeyFrame"
    }

    /// Returns the size of a serialised `Game::Spitter` object.
    fn size() -> usize {
        0x100
    }

    /// Return a new `SpitterKeyFrame` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<SpitterKeyFrame, Error> {
        Ok(SpitterKeyFrame {})
    }
}