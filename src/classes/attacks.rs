use crate::classes::ShrekSuperSlamGameObject;
use crate::files::Bin;

/// Structure representing the in-game `Game::AttackMoveType` object type,
/// which represents an attack (from a character or an item).
///
/// This struct owns its own data from the .bin file that contains the data.
pub struct AttackMoveType {
    /// The first damage field, used for most damage calculations
    pub damage1: f32,

    /// The second damage field, unknown purpose
    pub damage2: f32,

    /// The third damage field, unknown purpose
    pub damage3: f32,

    /// The attack's hitboxes, if any
    pub hitboxes: Vec<AttackMoveRegion>,

    /// The attack's name
    pub name: String,
}

/// Structure representing the in-game `Game::AttackMoveRegion` object type,
/// which represents an attack's hitbox.
///
/// This struct owns its own data from the .bin file that contains the data.
pub struct AttackMoveRegion {
    /// The delay (in seconds?) from the attack starting to the hitbox coming out
    pub delay: f32,

    /// The angle of the hitbox - smaller wraps more around the character
    pub width: f32,

    /// The height of the hitbox - larger extends out wider
    pub radius: f32,
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

        // Offset 0x20 of the AttackMoveType contains an offset within the .bin
        // file to a list of further offsets, each of which points to an
        // AttackMoveRegion object. These are the hitboxes for the attack.
        //
        // The number of items in the list pointed by the offset is located at
        // offset 0x24 within the AttackMoveType object.
        //
        // We use this information to construct a list of AttackMoveRegion
        // objects for the attack.
        let num_hitboxes = c.read_u32(&raw[offset + 0x24..offset + 0x28]);
        let regions_offset = c.read_u32(&raw[offset + 0x20..offset + 0x24]);
        let hitboxes = (0..num_hitboxes)
            .map(|i| {
                let region_offset_offset = (regions_offset + 0x40 + (i * 4)) as usize;
                let obj_offset = c.read_u32(&raw[region_offset_offset..region_offset_offset + 4]);
                bin.get_object_from_offset::<AttackMoveRegion>(obj_offset)
                    .unwrap()
            })
            .collect();

        AttackMoveType {
            damage1,
            damage2,
            damage3,
            hitboxes,
            name: bin.get_str_from_offset(name_offset).unwrap().to_owned(),
        }
    }
}

impl ShrekSuperSlamGameObject for AttackMoveRegion {
    /// # Returns
    ///
    /// The hashcode for the Game::AttackMoveRegion in-game object
    fn hash() -> u32 {
        0xF2CFE08D
    }

    /// # Returns
    ///
    /// The name of the in-game class - "Game::AttackMoveRegion"
    fn name() -> &'static str {
        "Game::AttackMoveRegion"
    }

    /// Constructor
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin containing the object
    /// - `offset`: The offset the object begins at within the .bin file
    fn new(bin: &Bin, offset: usize) -> AttackMoveRegion {
        let c = bin.console;

        AttackMoveRegion {
            delay: c.read_f32(&bin.raw[offset + 0x04..offset + 0x08]),
            width: c.read_f32(&bin.raw[offset + 0x30..offset + 0x34]),
            radius: c.read_f32(&bin.raw[offset + 0x38..offset + 0x3C]),
        }
    }
}
