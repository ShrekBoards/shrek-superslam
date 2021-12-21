use crate::classes::util;
use crate::classes::{EventSequence, SerialisedShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::Spitter` object type.
///
/// A 'spitter' is what players get slammed into, and is responsible for
/// spawning the player back onto the battlefield, and the slam event
/// animations.
pub struct Spitter {
    /// The keyframes of the spitter.
    pub keyframes: Vec<SpitterKeyframe>,
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
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<Spitter, Error> {
        // The list of keyframes is at offset +20.
        // The count of keyframes is at offset +24.
        let keyframes = util::construct_array(bin, offset, bin.console, 0x20, 0x24)?;

        Ok(Spitter { keyframes })
    }
}

/// Structure representing the in-game `Game::SpitterKeyframe` object type.
///
/// This represents an individual keyframe within a spitter animation.
pub struct SpitterKeyframe {
    /// The event to run on the keyframe, if any.
    pub event: Option<EventSequence>,
}

impl SerialisedShrekSuperSlamGameObject for SpitterKeyframe {
    /// Returns the hashcode for the `Game::SpitterKeyframe` in-game object.
    fn hash() -> u32 {
        0x84AD7E70
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::SpitterKeyframe"
    }

    /// Returns the size of a serialised `Game::Spitter` object.
    fn size() -> usize {
        0x100
    }

    /// Return a new `SpitterKeyframe` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<SpitterKeyframe, Error> {
        // The offset to a EventSequence, if any, is at +BC
        let event = util::construct_optional_type(bin, offset, bin.console, 0xBC)?;

        Ok(SpitterKeyframe { event })
    }
}
