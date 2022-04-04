use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::GameWorld` object type.
///
/// This contains information about a level.
pub struct GameWorld {
    // Setting this field to anything but 1 locks the players in place. It is
    // always set to 1, at least in the files. Possibly modified at runtime?
    _playable: u32,

    // These unknown floats seem to be related to out-of-bounds somehow? Making
    // any of them really small seems to shift the OOB boundary. In almost all
    // cases, the first set are all 100, and the second set are -100, -10 and
    // -100 each.
    _unknown_float_1_x: f32,
    _unknown_float_1_y: f32,
    _unknown_float_1_z: f32,
    _unknown_float_2_x: f32,
    _unknown_float_2_y: f32,
    _unknown_float_2_z: f32,

    /// The raw bytes of the object.
    _bytes: Vec<u8>,
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
        let c = bin.console;
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();

        // Playable flag is at +14
        let playable = c.read_u32(&bytes[0x14..0x18])?;

        // Weird floats set one begin at +30
        let unknown_float_1_x = c.read_f32(&bytes[0x30..0x34])?;
        let unknown_float_1_y = c.read_f32(&bytes[0x34..0x38])?;
        let unknown_float_1_z = c.read_f32(&bytes[0x38..0x3C])?;

        let unknown_float_2_x = c.read_f32(&bytes[0x40..0x44])?;
        let unknown_float_2_y = c.read_f32(&bytes[0x44..0x48])?;
        let unknown_float_2_z = c.read_f32(&bytes[0x48..0x4C])?;

        Ok(GameWorld {
            _playable: playable,
            _unknown_float_1_x: unknown_float_1_x,
            _unknown_float_1_y: unknown_float_1_y,
            _unknown_float_1_z: unknown_float_1_z,
            _unknown_float_2_x: unknown_float_2_x,
            _unknown_float_2_y: unknown_float_2_y,
            _unknown_float_2_z: unknown_float_2_z,
            _bytes: bytes,
        })
    }
}
