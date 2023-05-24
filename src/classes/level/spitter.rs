use serde::{Deserialize, Serialize};

use crate::Console;
use crate::classes::{EventSequence, SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::Spitter` object type.
///
/// A 'spitter' is what players get slammed into, and is responsible for
/// spawning the player back onto the battlefield, and the slam event
/// animations.
#[derive(Deserialize, Serialize)]
pub struct Spitter {
    /// The keyframes of the spitter.
    pub keyframes: Vec<SpitterKeyframe>,

    /// Unknown property at offset +038.
    pub unknown_004: u32,

    /// Unknown property at offset +038.
    pub unknown_038: u32,

    /// Unknown property at offset +044.
    pub unknown_044: u8,

    /// Unknown property at offset +045.
    pub unknown_045: u8,

    /// Unknown property at offset +046.
    pub unknown_046: u8,

    /// Unknown property at offset +047.
    pub unknown_047: u8,

    /// The offsets within the level.db.bin file of each keyframe, in the same
    /// order they exist within the keyframes property.
    #[serde(skip)]
    keyframe_offsets: Vec<u32>,
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

        // Read the list of keyframe offsets, and use those to read each keyframe
        let keyframe_offsets = Spitter::keyframe_offsets(&raw, offset, c)?;
        let keyframes = keyframe_offsets
            .iter()
            .map(|o| bin.get_object_from_offset::<SpitterKeyframe>(*o).unwrap())
            .collect();

        // Unknown fields
        let unknown_004 = c.read_u32(&raw[offset + 0x04..offset + 0x08])?;
        let unknown_038 = c.read_u32(&raw[offset + 0x38..offset + 0x3C])?;

        let unknown_044 = raw[offset + 0x44];
        let unknown_045 = raw[offset + 0x45];
        let unknown_046 = raw[offset + 0x46];
        let unknown_047 = raw[offset + 0x47];

        Ok(Spitter {
            keyframes,
            keyframe_offsets,
            unknown_004,
            unknown_038,
            unknown_044,
            unknown_045,
            unknown_046,
            unknown_047,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for Spitter {
    /// Writes the object back to its `bin` file at the given `offset`.
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;

        // Unknown fields
        bin.raw
            .splice(offset + 0x04..offset + 0x08, c.write_u32(self.unknown_004)?);
        bin.raw
            .splice(offset + 0x38..offset + 0x3C, c.write_u32(self.unknown_038)?);

        bin.raw[offset + 0x44] = self.unknown_044;
        bin.raw[offset + 0x45] = self.unknown_045;
        bin.raw[offset + 0x46] = self.unknown_046;
        bin.raw[offset + 0x47] = self.unknown_047;

        // Write the spitter's keyframes back to the .bin file
        //
        // If this Spitter was deserialised (e.g. from a JSON version),
        // we will not know where the keyframes are supposed to go in the .bin
        // file, so read out the offsets from the object that we are about to
        // replace
        let keyframes_count = c.read_u32(&bin.raw[offset + 0x24..offset + 0x28])? as usize;
        let keyframe_offsets = if keyframes_count > self.keyframe_offsets.len()
        {
            Spitter::keyframe_offsets(&bin.raw, offset, c)?
                .iter()
                .map(|o| o + Bin::header_length() as u32)
                .collect()
        } else {
            self.keyframe_offsets.clone()
        };

        for (offset, keyframe) in keyframe_offsets.iter().zip(self.keyframes.iter()) {
            keyframe.write(bin, *offset as usize)?;
        }

        Ok(())
    }
}

impl Spitter {
    /// Retrieve a list of offsets for a spitter's keyframes within the .bin file
    fn keyframe_offsets(raw: &[u8], offset: usize, console: Console) -> Result<Vec<u32>, Error> {
        // Offset 0x20 of the Spitter contains an offset within the .bin
        // file to a list of further offsets, each of which points to an
        // SpitterKeyframe object. These are the keyframes for the Spitter.
        //
        // The number of items in the list pointed by the offset is located at
        // offset 0x24 within the Spitter object.
        //
        // We later use this information to construct a list of AttackMoveRegion
        // objects for the attack.
        let keyframes_count = console.read_u32(&raw[offset + 0x24..offset + 0x28])? as usize;
        let keyframes_offset = console.read_u32(&raw[offset + 0x20..offset + 0x24])?;
        (0..keyframes_count)
            .map(|i| {
                let region_offset_offset = keyframes_offset as usize + Bin::header_length() + (i * 4);
                console.read_u32(&raw[region_offset_offset..region_offset_offset + 4])
            })
            .collect()
    }
}

/// Structure representing the in-game `Game::SpitterKeyframe` object type.
///
/// This represents an individual keyframe within a spitter animation.
#[derive(Deserialize, Serialize)]
pub struct SpitterKeyframe {
    /// Unknown property at offset +00c.
    pub unknown_00c: u32,

    #[serde(skip)]
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

        // Unknown fields
        let unknown_00c = c.read_u32(&raw[offset + 0x0C..offset + 0x10])?;

        // The offset to a EventSequence, if any, is at +BC
        let sequence_event_offset = c.read_u32(&raw[offset + 0xBC..offset + 0xC0])?;
        let event = if sequence_event_offset != 0 {
            Some(bin.get_object_from_offset::<EventSequence>(sequence_event_offset)?)
        } else {
            None
        };

        Ok(SpitterKeyframe { unknown_00c, event, })
    }
}

impl WriteableShrekSuperSlamGameObject for SpitterKeyframe {
    /// Writes the object back to its `bin` file at the given `offset`.
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;

        // Unknown fields
        bin.raw
            .splice(offset + 0x0C..offset + 0x10, c.write_u32(self.unknown_00c)?);

        Ok(())
    }
}