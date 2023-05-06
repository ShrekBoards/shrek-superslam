use serde::{Deserialize, Serialize};

use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::GameWorld` object type.
///
/// This contains information about a level.
#[derive(Deserialize, Serialize)]
pub struct GameWorld {
    // Setting this field to anything but 1 locks the players in place. It is
    // always set to 1, at least in the files. Possibly modified at runtime?
    pub playable: u32,

    // These unknown floats seem to be related to out-of-bounds somehow? Making
    // any of them really small seems to shift the OOB boundary. In almost all
    // cases, the first set are all 100, and the second set are -100, -10 and
    // -100 each.
    pub unknown_float_1_x: f32,
    pub unknown_float_1_y: f32,
    pub unknown_float_1_z: f32,
    pub unknown_float_2_x: f32,
    pub unknown_float_2_y: f32,
    pub unknown_float_2_z: f32,
}

impl SerialisedShrekSuperSlamGameObject for GameWorld {
    /// Returns the hashcode for the `Game::GameWorld` in-game object.
    fn hash() -> u32 {
        0xB974E53B
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::GameWorld"
    }

    /// Returns the size of a serialised `Game::GameWorld` object.
    fn size() -> usize {
        0xB430
    }

    /// Return a new `GameWorld` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<GameWorld, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // Playable flag is at +14
        let playable = c.read_u32(&raw[offset + 0x14..offset + 0x18])?;

        // Weird floats set one begin at +30
        let unknown_float_1_x = c.read_f32(&raw[offset + 0x30..offset + 0x34])?;
        let unknown_float_1_y = c.read_f32(&raw[offset + 0x34..offset + 0x38])?;
        let unknown_float_1_z = c.read_f32(&raw[offset + 0x38..offset + 0x3C])?;

        let unknown_float_2_x = c.read_f32(&raw[offset + 0x40..offset + 0x44])?;
        let unknown_float_2_y = c.read_f32(&raw[offset + 0x44..offset + 0x48])?;
        let unknown_float_2_z = c.read_f32(&raw[offset + 0x48..offset + 0x4C])?;

        Ok(GameWorld {
            playable,
            unknown_float_1_x,
            unknown_float_1_y,
            unknown_float_1_z,
            unknown_float_2_x,
            unknown_float_2_y,
            unknown_float_2_z,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for GameWorld {
    /// Writes the object back to its `bin` file at the given `offset`.
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x14..offset + 0x18, c.write_u32(self.playable)?);

        bin.raw
            .splice(offset + 0x30..offset + 0x34, c.write_f32(self.unknown_float_1_x)?);
        bin.raw
            .splice(offset + 0x34..offset + 0x38, c.write_f32(self.unknown_float_1_y)?);
        bin.raw
            .splice(offset + 0x38..offset + 0x3C, c.write_f32(self.unknown_float_1_z)?);

        bin.raw
            .splice(offset + 0x40..offset + 0x44, c.write_f32(self.unknown_float_2_x)?);
        bin.raw
            .splice(offset + 0x44..offset + 0x48, c.write_f32(self.unknown_float_2_y)?);
        bin.raw
            .splice(offset + 0x48..offset + 0x4C, c.write_f32(self.unknown_float_2_z)?);

        Ok(())
    }
}