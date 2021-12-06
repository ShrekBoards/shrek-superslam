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
        let raw = &bin.raw;
        let c = bin.console;

        // The list of keyframes is at offset +20.
        // The count of keyframes is at offset +24.
        let keyframe_list_offset = c.read_u32(&raw[offset + 0x20..offset + 0x24])? as usize;
        let keyframes_count = c.read_u32(&raw[offset + 0x24..offset + 0x28])? as usize;
        let keyframes: Result<Vec<SpitterKeyframe>, Error> =
            (0..keyframes_count)
                .map(|i| {
                    let keyframe_list_entry_offset = (Bin::header_length() + keyframe_list_offset + (i * 4)) as usize; 
                    c.read_u32(&raw[keyframe_list_entry_offset..keyframe_list_entry_offset + 4])
                })
                .map(|offset| { bin.get_object_from_offset::<SpitterKeyframe>(offset?) })
                .collect();

        Ok(Spitter { keyframes: keyframes?, })
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
        let raw = &bin.raw;
        let c = bin.console;

        // The offset to a EventSequence, if any, is at +BC
        let sequence_event_offset = c.read_u32(&raw[offset + 0xBC..offset + 0xC0])?;
        let event = if sequence_event_offset != 0 {
            Some(bin.get_object_from_offset::<EventSequence>(sequence_event_offset)?)
        } else {
            None
        };

        Ok(SpitterKeyframe { event, })
    }
}