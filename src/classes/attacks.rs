use serde::{Deserialize, Serialize};

use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::files::Bin;
use crate::Console;

/// Structure representing the in-game `Game::AttackMoveType` object type,
/// which represents an attack (from a character or an item).
#[derive(Deserialize, Serialize)]
pub struct AttackMoveType {
    /// The first damage field, used for most damage calculations
    pub damage1: f32,

    /// The second damage field, unknown purpose
    pub damage2: f32,

    /// The third damage field, unknown purpose
    pub damage3: f32,

    /// If true, the attack cannot be used
    pub disabled: bool,

    /// The amount of time (in seconds) the character is inactive for after
    /// using the attack
    pub endlag: f32,

    /// Vertical movement vector - positive goes up, negative goes down
    pub fall_speed: f32,

    /// The attack's hitboxes, if any
    pub hitboxes: Vec<AttackMoveRegion>,

    /// If true, the attack passes through and does no damage or knockback
    pub intangible: bool,

    /// True if the attack knocks the opponent down, false if not
    pub knocks_down: bool,

    /// The attack's name
    pub name: String,

    /// The offsets within the player.db.bin file of each hitbox, in the same
    /// order they exist within the hitboxes property
    #[serde(skip)]
    hitbox_offsets: Vec<u32>,
}

/// Structure representing the in-game `Game::AttackMoveRegion` object type,
/// which represents an attack's hitbox.
///
/// This struct owns its own data from the .bin file that contains the data.
#[derive(Deserialize, Serialize)]
pub struct AttackMoveRegion {
    /// The delay (in seconds?) from the attack starting to the hitbox coming out
    pub delay: f32,

    /// The angle of the hitbox - smaller wraps more around the character
    pub width: f32,

    /// The height of the hitbox - larger extends out wider
    pub radius: f32,
}

impl SerialisedShrekSuperSlamGameObject for AttackMoveType {
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

    /// # Returns
    ///
    /// The size of a serialised Game::AttackMoveType object
    fn size() -> usize {
        0x260
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

        // Read numeric fields
        let endlag = c.read_f32(&raw[offset + 0x04..offset + 0x08]);
        let fall_speed = c.read_f32(&raw[offset + 0x14..offset + 0x18]);
        let damage1 = c.read_f32(&raw[offset + 0x84..offset + 0x88]);
        let damage2 = c.read_f32(&raw[offset + 0x88..offset + 0x8C]);
        let damage3 = c.read_f32(&raw[offset + 0x8C..offset + 0x90]);
        let name_offset = c.read_u32(&raw[offset + 0x28..offset + 0x2C]);

        // Read boolean flag fields
        let knocks_down = raw[offset + 0x34] != 0;
        let disabled = raw[offset + 0x35] != 0;
        let intangible = raw[offset + 0x3A] != 0;

        // Read the list of hitbox offsets, and use those to read each hitbox
        let hitbox_offsets = AttackMoveType::hitbox_offsets(&raw, offset, c);
        let hitboxes = hitbox_offsets
            .iter()
            .map(|o| bin.get_object_from_offset::<AttackMoveRegion>(*o).unwrap())
            .collect();

        AttackMoveType {
            endlag,
            fall_speed,
            damage1,
            damage2,
            damage3,
            disabled,
            hitboxes,
            intangible,
            knocks_down,
            name: bin.get_str_from_offset(name_offset).unwrap(),
            hitbox_offsets,
        }
    }
}

impl WriteableShrekSuperSlamGameObject for AttackMoveType {
    /// Writes the Game::AttackMoveType object back to its .bin file
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin file to write this object to
    /// - `offset`: The offset to write this object back to
    fn write(&self, bin: &mut Bin, offset: usize) {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x04..offset + 0x08, c.write_f32(self.endlag));
        bin.raw
            .splice(offset + 0x14..offset + 0x18, c.write_f32(self.fall_speed));
        bin.raw[offset + 0x34] = self.knocks_down as u8;
        bin.raw[offset + 0x35] = self.disabled as u8;
        bin.raw[offset + 0x3A] = self.intangible as u8;
        bin.raw
            .splice(offset + 0x84..offset + 0x88, c.write_f32(self.damage1));
        bin.raw
            .splice(offset + 0x88..offset + 0x8C, c.write_f32(self.damage2));
        bin.raw
            .splice(offset + 0x8C..offset + 0x90, c.write_f32(self.damage3));

        // Write the attack's hitboxes back to the .bin file too
        //
        // If this AttackMoveType was deserialised (e.g. from a JSON version),
        // we will not know where the hitboxes are supposed to go in the .bin
        // file, so read out the offsets from the object that we are about to
        // replace
        let hitbox_offsets = if AttackMoveType::number_of_hitboxes(&bin.raw, offset, c)
            > self.hitbox_offsets.len() as u32
        {
            AttackMoveType::hitbox_offsets(&bin.raw, offset, c)
                .iter()
                .map(|o| o + 0x40)
                .collect()
        } else {
            self.hitbox_offsets.clone()
        };

        for (offset, hitbox) in hitbox_offsets.iter().zip(self.hitboxes.iter()) {
            hitbox.write(bin, *offset as usize);
        }
    }
}

impl AttackMoveType {
    /// Retrieve a list of offsets for an attack's hitboxes within the .bin file
    ///
    /// # Parameters
    ///
    /// - `raw`: The full bytes of the .bin file
    /// - `offset`: The offset the attack starts at within the file
    /// - `console`: The console version the file comes from
    ///
    /// # Returns
    ///
    /// A list of offsets within the .bin file where each hitbox for the attack
    /// at the offset is located. Empty if the attack has no hitboxes.
    fn hitbox_offsets(raw: &[u8], offset: usize, console: Console) -> Vec<u32> {
        // Offset 0x20 of the AttackMoveType contains an offset within the .bin
        // file to a list of further offsets, each of which points to an
        // AttackMoveRegion object. These are the hitboxes for the attack.
        //
        // The number of items in the list pointed by the offset is located at
        // offset 0x24 within the AttackMoveType object.
        //
        // We later use this information to construct a list of AttackMoveRegion
        // objects for the attack.
        let num_hitboxes = AttackMoveType::number_of_hitboxes(&raw, offset, console);
        let regions_offset = console.read_u32(&raw[offset + 0x20..offset + 0x24]);
        (0..num_hitboxes)
            .map(|i| {
                let region_offset_offset = (regions_offset + 0x40 + (i * 4)) as usize;
                console.read_u32(&raw[region_offset_offset..region_offset_offset + 4])
            })
            .collect()
    }

    /// Retrieve the number of hitboxes an attack has
    ///
    /// # Parameters
    ///
    /// - `raw`: The full bytes of the .bin file
    /// - `offset`: The offset the attack starts at within the file
    /// - `console`: The console version the file comes from
    ///
    /// # Returns
    ///
    /// The number of hitboxes for the attack starting at the given offset
    fn number_of_hitboxes(raw: &[u8], offset: usize, console: Console) -> u32 {
        console.read_u32(&raw[offset + 0x24..offset + 0x28])
    }
}

impl SerialisedShrekSuperSlamGameObject for AttackMoveRegion {
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

    /// # Returns
    ///
    /// The size of a serialised Game::AttackMoveRegion object
    fn size() -> usize {
        0x40
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

impl WriteableShrekSuperSlamGameObject for AttackMoveRegion {
    /// Writes the Game::AttackMoveRegion object back to its .bin file
    ///
    /// # Parameters
    ///
    /// - `bin`: The .bin file to write this object to
    /// - `offset`: The offset to write this object back to
    fn write(&self, bin: &mut Bin, offset: usize) {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x04..offset + 0x08, c.write_f32(self.delay));
        bin.raw
            .splice(offset + 0x30..offset + 0x34, c.write_f32(self.width));
        bin.raw
            .splice(offset + 0x38..offset + 0x3C, c.write_f32(self.radius));
    }
}
