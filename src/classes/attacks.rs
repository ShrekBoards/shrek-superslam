use crate::classes::ShrekSuperSlamGameObject;
use crate::files::Bin;

/// Structure representing the in-game `Game::AttackMoveType` object type.
/// This struct owns its own data from the .bin file that contains the data.
pub struct AttackMoveType {
    pub damage1: f32,
    pub damage2: f32,
    pub damage3: f32,
    pub name: String,
}

impl ShrekSuperSlamGameObject for AttackMoveType {
    /// # Returns
    ///
    /// The hashcode for the Game::AttackMoveType in-game object
    fn hash() -> u32 {
        0xEBF07BB5
    }

    /// # Returns
    ///
    /// The name of the in-game class - "Game::AttackMoveType"
    fn name() -> &'static str {
        "Game::AttackMoveType"
    }

    /// Constructor
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin containing the object
    /// - `offset`: The offset the object begins at within the .bin file
    fn new(bin: &Bin, offset: usize) -> AttackMoveType {
        let raw = &bin.raw;
        let c = bin.console;

        let damage1 = c.read_f32(&raw[offset + 0x84..offset + 0x88]);
        let damage2 = c.read_f32(&raw[offset + 0x88..offset + 0x8C]);
        let damage3 = c.read_f32(&raw[offset + 0x8C..offset + 0x90]);
        let name_offset = c.read_u32(&raw[offset + 0x28..offset + 0x2C]);

        AttackMoveType {
            damage1,
            damage2,
            damage3,
            name: bin.get_str_from_offset(name_offset).unwrap().to_owned(),
        }
    }
}
