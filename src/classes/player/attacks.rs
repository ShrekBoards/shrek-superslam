use serde::{Deserialize, Serialize};

use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;
use crate::Console;

/// Structure representing the in-game `Game::AttackMoveType` object type.
///
/// This type represents a single attack, from a player character or from an
/// item.
#[derive(Deserialize, Serialize)]
pub struct AttackMoveType {
    /// The distance at which the attack homes in on the opponent.
    pub aim_range: f32,

    /// The time (in seconds) for a strong move or throw to fully charge.
    pub charge: f32,

    /// The time (in seconds) for the flash effect to appear when charging a
    /// strong move or throw.
    pub charge_effect: f32,

    /// The first damage field, used for most damage calculations.
    pub damage1: f32,

    /// The second damage field, unknown purpose.
    pub damage2: f32,

    /// The third damage field, unknown purpose.
    pub damage3: f32,

    /// If true, the attack cannot be used.
    pub disabled: bool,

    /// The amount of time (in seconds) the character is inactive for after
    /// using the attack.
    pub endlag: f32,

    /// Vertical movement vector - positive goes up, negative goes down.
    pub fall_speed: f32,

    /// The attack's hitboxes, if any.
    pub hitboxes: Vec<AttackMoveRegion>,

    /// If true, the attack hits characters that are knocked down.
    pub hits_otg: bool,

    /// If true, the attack passes through and does no damage or knockback.
    pub intangible: bool,

    /// Period of seconds to grant invincibility for. Expires if another attack is performed.
    pub invincibility: f32,

    /// If true, the attack slams the opponent on contact, when the user's SLAM
    /// meter is full.
    pub is_slam: bool,

    /// The amount the attack knocks the opponent back. Positive pushes away, negative pulls toward.
    pub knockback: f32,

    /// True if the attack knocks the opponent down, false if not.
    pub knocks_down: bool,

    /// True to lock the position of the user while the move is being used.
    pub lock_position: bool,

    /// Speed (in seconds) that multi-hit moves hit. Set to zero to make the
    /// move single-hit.
    pub multi_hit_speed: f32,

    /// The attack's name.
    pub name: String,

    /// If true, the move will not make contact with the opponent. The move
    /// will still make contact with walls and throwables.
    pub no_opponent_contact: bool,

    /// The projectile type the attack spawns, if any.
    pub projectile: Option<ProjectileType>,

    /// True if the attack breaks shield (block) on contact, false if not.
    pub shield_breaks: bool,

    /// The amount of time the attack stuns for, in seconds.
    pub stun: f32,

    /// Unknown property at offset +008
    pub unknown_008: f32,

    /// Unknown property at offset +010
    pub unknown_010: f32,

    /// Unknown property at offset +018
    pub unknown_018: f32,

    /// Unknown property at offset +02D
    pub unknown_02d: bool,

    /// Unknown property at offset +02F
    pub unknown_02f: bool,

    /// Unknown property at offset +030
    pub unknown_030: bool,

    /// Unknown property at offset +036
    pub unknown_036: bool,

    /// Unknown property at offset +037
    pub unknown_037: bool,

    /// Unknown property at offset +038
    pub unknown_038: bool,

    /// Unknown property at offset +039
    pub unknown_039: bool,

    /// Unknown property at offset +03C
    pub unknown_03c: u32,

    /// Unknown property at offset +040
    pub unknown_040: bool,

    /// Unknown property at offset +041
    pub unknown_041: bool,

    /// Unknown property at offset +042
    pub unknown_042: bool,

    /// Unknown property at offset +043
    pub unknown_043: bool,

    /// Unknown property at offset +044
    pub unknown_044: bool,

    /// Unknown property at offset +045
    pub unknown_045_max_255: u8,

    /// Unknown property at offset +046
    pub unknown_046_max_128: i8,

    /// Unknown property at offset +047
    pub unknown_047: bool,

    /// Unknown property at offset +049
    pub unknown_049_does_something_if_5_max_value_255: u8,

    /// Unknown property at offset +04A
    pub unknown_04a: bool,

    /// Unknown property at offset +04B
    pub unknown_04b: bool,

    /// Unknown property at offset +070
    pub unknown_070: f32,

    /// Unknown property at offset +078
    pub unknown_078: f32,

    /// Unknown property at offset +07C
    pub unknown_07c: f32,

    /// Unknown property at offset +090
    pub unknown_090: f32,

    /// Unknown property at offset +0A8
    pub unknown_0a8: f32,

    /// Unknown property at offset +0B0
    pub unknown_0b0: f32,

    /// Unknown property at offset +0B4
    pub unknown_0b4: f32,

    /// Unknown property at offset +0B8
    pub unknown_0b8: f32,

    /// Unknown property at offset +0BC
    pub unknown_0bc: f32,

    /// Unknown property at offset +0C0, relates to opponents horizontal vector
    /// when colliding with a shielded opponent?
    pub unknown_0c0_shielded_opponent_horizontal_vector: f32,

    /// Unknown property at offset +0C4
    pub unknown_0c4: f32,

    /// Unknown property at offset +0D8
    pub unknown_0d8: f32,

    /// Unknown property at offset +0DC
    pub unknown_0dc: f32,

    /// Unknown property at offset +0E0
    pub unknown_0e0: f32,

    /// Unknown property at offset +0E4
    pub unknown_0e4: f32,

    /// Unknown property at offset +0E8
    pub unknown_0e8: f32,

    /// Unknown property at offset +0EC
    pub unknown_0ec: f32,

    /// Unknown property at offset +120
    pub unknown_120: f32,

    /// Unknown property at offset +124
    pub unknown_124: f32,

    /// Unknown property at offset +128
    pub unknown_128: f32,

    /// Unknown property at offset +12C
    pub unknown_12c: f32,

    /// Unknown property at offset +130
    pub unknown_130: f32,

    /// Unknown property at offset +134
    pub unknown_134: f32,

    /// Unknown property at offset +138
    pub unknown_138: f32,

    /// Unknown property at offset +13C
    pub unknown_13c: f32,

    /// The offsets within the player.db.bin file of each hitbox, in the same
    /// order they exist within the hitboxes property.
    #[serde(skip)]
    hitbox_offsets: Vec<u32>,

    /// The offset within the player.db.bin file the projectile exists at,
    /// if there is any projectile.
    #[serde(skip)]
    projectile_offset: Option<u32>,
}

impl SerialisedShrekSuperSlamGameObject for AttackMoveType {
    /// Returns the hashcode for the `Game::AttackMoveType` in-game object.
    fn hash() -> u32 {
        0xEBF07BB5
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::AttackMoveType"
    }

    /// Returns the size of a serialised `Game::AttackMoveType` object.
    fn size() -> usize {
        0x260
    }

    /// Return a new `AttackMoveType` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<AttackMoveType, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // Read numeric fields
        let endlag = c.read_f32(&raw[offset + 0x04..offset + 0x08])?;
        let invincibility = c.read_f32(&raw[offset + 0x0C..offset + 0x10])?;
        let fall_speed = c.read_f32(&raw[offset + 0x14..offset + 0x18])?;
        let name_offset = c.read_u32(&raw[offset + 0x28..offset + 0x2C])?;
        let aim_range = c.read_f32(&raw[offset + 0x74..offset + 0x78])?;
        let damage1 = c.read_f32(&raw[offset + 0x84..offset + 0x88])?;
        let damage2 = c.read_f32(&raw[offset + 0x88..offset + 0x8C])?;
        let damage3 = c.read_f32(&raw[offset + 0x8C..offset + 0x90])?;
        let charge_effect = c.read_f32(&raw[offset + 0x94..offset + 0x98])?;
        let charge = c.read_f32(&raw[offset + 0x98..offset + 0x9C])?;
        let multi_hit_speed = c.read_f32(&raw[offset + 0xA0..offset + 0xA4])?;
        let stun = c.read_f32(&raw[offset + 0xA4..offset + 0xA8])?;
        let knockback = c.read_f32(&raw[offset + 0xAC..offset + 0xB0])?;

        let unknown_008 = c.read_f32(&raw[offset + 0x08..offset + 0x0C])?;
        let unknown_010 = c.read_f32(&raw[offset + 0x10..offset + 0x14])?;
        let unknown_018 = c.read_f32(&raw[offset + 0x18..offset + 0x1C])?;
        let unknown_03c = c.read_u32(&raw[offset + 0x3C..offset + 0x40])?;
        let unknown_070 = c.read_f32(&raw[offset + 0x70..offset + 0x74])?;
        let unknown_078 = c.read_f32(&raw[offset + 0x78..offset + 0x7C])?;
        let unknown_07c = c.read_f32(&raw[offset + 0x7C..offset + 0x80])?;
        let unknown_090 = c.read_f32(&raw[offset + 0x90..offset + 0x94])?;
        let unknown_0a8 = c.read_f32(&raw[offset + 0xA8..offset + 0xAC])?;
        let unknown_0b0 = c.read_f32(&raw[offset + 0xB0..offset + 0xB4])?;
        let unknown_0b4 = c.read_f32(&raw[offset + 0xB4..offset + 0xB8])?;
        let unknown_0b8 = c.read_f32(&raw[offset + 0xB8..offset + 0xBC])?;
        let unknown_0bc = c.read_f32(&raw[offset + 0xBC..offset + 0xC0])?;
        let unknown_0c0 = c.read_f32(&raw[offset + 0xC0..offset + 0xC4])?;
        let unknown_0c4 = c.read_f32(&raw[offset + 0xC4..offset + 0xC8])?;
        let unknown_0d8 = c.read_f32(&raw[offset + 0xD8..offset + 0xDC])?;
        let unknown_0dc = c.read_f32(&raw[offset + 0xDC..offset + 0xE0])?;
        let unknown_0e0 = c.read_f32(&raw[offset + 0xE0..offset + 0xE4])?;
        let unknown_0e4 = c.read_f32(&raw[offset + 0xE4..offset + 0xE8])?;
        let unknown_0e8 = c.read_f32(&raw[offset + 0xE8..offset + 0xEC])?;
        let unknown_0ec = c.read_f32(&raw[offset + 0xEC..offset + 0xF0])?;
        let unknown_120 = c.read_f32(&raw[offset + 0x120..offset + 0x124])?;
        let unknown_124 = c.read_f32(&raw[offset + 0x124..offset + 0x128])?;
        let unknown_128 = c.read_f32(&raw[offset + 0x128..offset + 0x12C])?;
        let unknown_12c = c.read_f32(&raw[offset + 0x12C..offset + 0x130])?;
        let unknown_130 = c.read_f32(&raw[offset + 0x130..offset + 0x134])?;
        let unknown_134 = c.read_f32(&raw[offset + 0x134..offset + 0x138])?;
        let unknown_138 = c.read_f32(&raw[offset + 0x138..offset + 0x13C])?;
        let unknown_13c = c.read_f32(&raw[offset + 0x13C..offset + 0x140])?;

        // Read boolean flag fields
        let is_slam = raw[offset + 0x2C] != 0;
        let shield_breaks = raw[offset + 0x2E] != 0;
        let lock_position = raw[offset + 0x31] != 0;
        let no_opponent_contact = raw[offset + 0x32] != 0;
        let hits_otg = raw[offset + 0x33] != 0;
        let knocks_down = raw[offset + 0x34] != 0;
        let disabled = raw[offset + 0x35] != 0;
        let intangible = raw[offset + 0x3A] != 0;

        let unknown_02d = raw[offset + 0x2D] != 0;
        let unknown_02f = raw[offset + 0x2F] != 0;
        let unknown_030 = raw[offset + 0x30] != 0;
        let unknown_036 = raw[offset + 0x36] != 0;
        let unknown_037 = raw[offset + 0x37] != 0;
        let unknown_038 = raw[offset + 0x38] != 0;
        let unknown_039 = raw[offset + 0x39] != 0;
        let unknown_040 = raw[offset + 0x40] != 0;
        let unknown_041 = raw[offset + 0x41] != 0;
        let unknown_042 = raw[offset + 0x42] != 0;
        let unknown_043 = raw[offset + 0x43] != 0;
        let unknown_044 = raw[offset + 0x44] != 0;
        let unknown_045 = raw[offset + 0x45];
        let unknown_046 = raw[offset + 0x46] as i8;
        let unknown_047 = raw[offset + 0x47] != 0;
        let unknown_049 = raw[offset + 0x49];
        let unknown_04a = raw[offset + 0x4A] != 0;
        let unknown_04b = raw[offset + 0x4B] != 0;

        // Read the projectile type the attack spawns, if any
        let projectile_offset_num = c.read_u32(&raw[offset + 0x9C..offset + 0xA0])?;
        let projectile_offset = match projectile_offset_num {
            0 => None,
            _ => Some(projectile_offset_num),
        };
        let projectile = projectile_offset.map(|offset| {
            bin.get_object_from_offset::<ProjectileType>(offset)
                .unwrap()
        });

        // Read the list of hitbox offsets, and use those to read each hitbox
        let hitbox_offsets = AttackMoveType::hitbox_offsets(&raw, offset, c)?;
        let hitboxes = hitbox_offsets
            .iter()
            .map(|o| bin.get_object_from_offset::<AttackMoveRegion>(*o).unwrap())
            .collect();

        Ok(AttackMoveType {
            aim_range,
            charge,
            charge_effect,
            endlag,
            fall_speed,
            damage1,
            damage2,
            damage3,
            disabled,
            hitboxes,
            hits_otg,
            intangible,
            invincibility,
            is_slam,
            knocks_down,
            knockback,
            lock_position,
            multi_hit_speed,
            name: bin.get_str_from_offset(name_offset)?,
            no_opponent_contact,
            projectile,
            shield_breaks,
            stun,
            hitbox_offsets,
            projectile_offset,
            unknown_008,
            unknown_010,
            unknown_018,
            unknown_02d,
            unknown_02f,
            unknown_030,
            unknown_036,
            unknown_037,
            unknown_038,
            unknown_039,
            unknown_03c,
            unknown_040,
            unknown_041,
            unknown_042,
            unknown_043,
            unknown_044,
            unknown_045_max_255: unknown_045,
            unknown_046_max_128: unknown_046,
            unknown_047,
            unknown_049_does_something_if_5_max_value_255: unknown_049,
            unknown_04a,
            unknown_04b,
            unknown_070,
            unknown_078,
            unknown_07c,
            unknown_090,
            unknown_0a8,
            unknown_0b0,
            unknown_0b4,
            unknown_0b8,
            unknown_0bc,
            unknown_0c0_shielded_opponent_horizontal_vector: unknown_0c0,
            unknown_0c4,
            unknown_0d8,
            unknown_0dc,
            unknown_0e0,
            unknown_0e4,
            unknown_0e8,
            unknown_0ec,
            unknown_120,
            unknown_124,
            unknown_128,
            unknown_12c,
            unknown_130,
            unknown_134,
            unknown_138,
            unknown_13c,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for AttackMoveType {
    /// Writes the object back to its `bin` file at the given `offset`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::WriteableShrekSuperSlamGameObject;
    /// use shrek_superslam::classes::AttackMoveType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load an attack from the .bin file, modify the damage, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC).unwrap();
    /// let mut attack = bin.get_object_from_offset::<AttackMoveType>(0x1000).unwrap();
    /// attack.damage1 = 100.0;
    /// attack.write(&mut bin, 0x1000);
    /// ```
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x04..offset + 0x08, c.write_f32(self.endlag)?);
        bin.raw
            .splice(offset + 0x0C..offset + 0x10, c.write_f32(self.invincibility)?);
        bin.raw
            .splice(offset + 0x14..offset + 0x18, c.write_f32(self.fall_speed)?);
        bin.raw[offset + 0x2C] = self.is_slam as u8;
        bin.raw[offset + 0x2E] = self.shield_breaks as u8;
        bin.raw[offset + 0x31] = self.lock_position as u8;
        bin.raw[offset + 0x32] = self.no_opponent_contact as u8;
        bin.raw[offset + 0x33] = self.hits_otg as u8;
        bin.raw[offset + 0x34] = self.knocks_down as u8;
        bin.raw[offset + 0x35] = self.disabled as u8;
        bin.raw[offset + 0x3A] = self.intangible as u8;
        bin.raw
            .splice(offset + 0x74..offset + 0x78, c.write_f32(self.aim_range)?);
        bin.raw
            .splice(offset + 0x84..offset + 0x88, c.write_f32(self.damage1)?);
        bin.raw
            .splice(offset + 0x88..offset + 0x8C, c.write_f32(self.damage2)?);
        bin.raw
            .splice(offset + 0x8C..offset + 0x90, c.write_f32(self.damage3)?);
        bin.raw.splice(
            offset + 0x94..offset + 0x98,
            c.write_f32(self.charge_effect)?,
        );
        bin.raw
            .splice(offset + 0x98..offset + 0x9C, c.write_f32(self.charge)?);
        bin.raw
            .splice(offset + 0xA0..offset + 0xA4, c.write_f32(self.multi_hit_speed)?);
        bin.raw
            .splice(offset + 0xA4..offset + 0xA8, c.write_f32(self.stun)?);
        bin.raw
            .splice(offset + 0xAC..offset + 0xB0, c.write_f32(self.knockback)?);

        // Unknown fields
        bin.raw
            .splice(offset + 0x08..offset + 0x0C, c.write_f32(self.unknown_008)?);
        bin.raw
            .splice(offset + 0x10..offset + 0x14, c.write_f32(self.unknown_010)?);
        bin.raw
            .splice(offset + 0x18..offset + 0x1C, c.write_f32(self.unknown_018)?);
        bin.raw
            .splice(offset + 0x3C..offset + 0x40, c.write_u32(self.unknown_03c)?);
        bin.raw
            .splice(offset + 0x70..offset + 0x74, c.write_f32(self.unknown_070)?);
        bin.raw
            .splice(offset + 0x78..offset + 0x7C, c.write_f32(self.unknown_078)?);
        bin.raw
            .splice(offset + 0x7C..offset + 0x80, c.write_f32(self.unknown_07c)?);
        bin.raw
            .splice(offset + 0x90..offset + 0x94, c.write_f32(self.unknown_090)?);
        bin.raw
            .splice(offset + 0xA8..offset + 0xAC, c.write_f32(self.unknown_0a8)?);
        bin.raw
            .splice(offset + 0xB0..offset + 0xB4, c.write_f32(self.unknown_0b0)?);
        bin.raw
            .splice(offset + 0xB4..offset + 0xB8, c.write_f32(self.unknown_0b4)?);
        bin.raw
            .splice(offset + 0xB8..offset + 0xBC, c.write_f32(self.unknown_0b8)?);
        bin.raw
            .splice(offset + 0xBC..offset + 0xC0, c.write_f32(self.unknown_0bc)?);
        bin.raw
            .splice(offset + 0xC0..offset + 0xC4, c.write_f32(self.unknown_0c0_shielded_opponent_horizontal_vector)?);
        bin.raw
            .splice(offset + 0xC4..offset + 0xC8, c.write_f32(self.unknown_0c4)?);
        bin.raw
            .splice(offset + 0xD8..offset + 0xDC, c.write_f32(self.unknown_0d8)?);
        bin.raw
            .splice(offset + 0xDC..offset + 0xE0, c.write_f32(self.unknown_0dc)?);
        bin.raw
            .splice(offset + 0xE0..offset + 0xE4, c.write_f32(self.unknown_0e0)?);
        bin.raw
            .splice(offset + 0xE4..offset + 0xE8, c.write_f32(self.unknown_0e4)?);
        bin.raw
            .splice(offset + 0xE8..offset + 0xEC, c.write_f32(self.unknown_0e8)?);
        bin.raw
            .splice(offset + 0xEC..offset + 0xF0, c.write_f32(self.unknown_0ec)?);
        bin.raw
            .splice(offset + 0x120..offset + 0x124, c.write_f32(self.unknown_120)?);
        bin.raw
            .splice(offset + 0x124..offset + 0x128, c.write_f32(self.unknown_124)?);
        bin.raw
            .splice(offset + 0x128..offset + 0x12C, c.write_f32(self.unknown_128)?);
        bin.raw
            .splice(offset + 0x12C..offset + 0x130, c.write_f32(self.unknown_12c)?);
        bin.raw
            .splice(offset + 0x130..offset + 0x134, c.write_f32(self.unknown_130)?);
        bin.raw
            .splice(offset + 0x134..offset + 0x138, c.write_f32(self.unknown_134)?);
        bin.raw
            .splice(offset + 0x138..offset + 0x13C, c.write_f32(self.unknown_138)?);
        bin.raw
            .splice(offset + 0x13C..offset + 0x140, c.write_f32(self.unknown_13c)?);

        bin.raw[offset + 0x2D] = self.unknown_02d as u8;
        bin.raw[offset + 0x2F] = self.unknown_02f as u8;
        bin.raw[offset + 0x30] = self.unknown_030 as u8;
        bin.raw[offset + 0x36] = self.unknown_036 as u8;
        bin.raw[offset + 0x37] = self.unknown_037 as u8;
        bin.raw[offset + 0x38] = self.unknown_038 as u8;
        bin.raw[offset + 0x39] = self.unknown_039 as u8;
        bin.raw[offset + 0x40] = self.unknown_040 as u8;
        bin.raw[offset + 0x41] = self.unknown_041 as u8;
        bin.raw[offset + 0x42] = self.unknown_042 as u8;
        bin.raw[offset + 0x43] = self.unknown_043 as u8;
        bin.raw[offset + 0x44] = self.unknown_044 as u8;
        bin.raw[offset + 0x45] = self.unknown_045_max_255;
        bin.raw[offset + 0x46] = self.unknown_046_max_128 as u8;
        bin.raw[offset + 0x47] = self.unknown_047 as u8;
        bin.raw[offset + 0x49] = self.unknown_049_does_something_if_5_max_value_255;
        bin.raw[offset + 0x4A] = self.unknown_04a as u8;
        bin.raw[offset + 0x4B] = self.unknown_04b as u8;

        // Write the attack's hitboxes back to the .bin file too
        //
        // If this AttackMoveType was deserialised (e.g. from a JSON version),
        // we will not know where the hitboxes are supposed to go in the .bin
        // file, so read out the offsets from the object that we are about to
        // replace
        let hitbox_offsets = if AttackMoveType::number_of_hitboxes(&bin.raw, offset, c)?
            > self.hitbox_offsets.len() as u32
        {
            AttackMoveType::hitbox_offsets(&bin.raw, offset, c)?
                .iter()
                .map(|o| o + Bin::header_length() as u32)
                .collect()
        } else {
            self.hitbox_offsets.clone()
        };

        for (offset, hitbox) in hitbox_offsets.iter().zip(self.hitboxes.iter()) {
            hitbox.write(bin, *offset as usize)?;
        }

        // Write the attack's projectile, if any, back to the .bin file too
        if self.projectile.is_some() && self.projectile_offset.is_some() {
            self.projectile
                .as_ref()
                .unwrap()
                .write(bin, self.projectile_offset.unwrap() as usize)?;
        }

        Ok(())
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
    fn hitbox_offsets(raw: &[u8], offset: usize, console: Console) -> Result<Vec<u32>, Error> {
        // Offset 0x20 of the AttackMoveType contains an offset within the .bin
        // file to a list of further offsets, each of which points to an
        // AttackMoveRegion object. These are the hitboxes for the attack.
        //
        // The number of items in the list pointed by the offset is located at
        // offset 0x24 within the AttackMoveType object.
        //
        // We later use this information to construct a list of AttackMoveRegion
        // objects for the attack.
        let num_hitboxes = AttackMoveType::number_of_hitboxes(&raw, offset, console)? as usize;
        let regions_offset = console.read_u32(&raw[offset + 0x20..offset + 0x24])?;
        (0..num_hitboxes)
            .map(|i| {
                let region_offset_offset = regions_offset as usize + Bin::header_length() + (i * 4);
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
    fn number_of_hitboxes(raw: &[u8], offset: usize, console: Console) -> Result<u32, Error> {
        console.read_u32(&raw[offset + 0x24..offset + 0x28])
    }
}

/// Structure representing the in-game `Game::ProjectileType` object type.
///
/// This type represents a projectile generated by an attack.
#[derive(Deserialize, Serialize)]
pub struct ProjectileType {
    /// Speed the projectile moves in the X-axis
    pub x_vector: f32,

    /// Some kind of angle the projectile is fired at
    pub angle: f32,

    /// The arc of the projectile, some other kind of angle
    pub arc: f32,

    /// Related to homing in on an opponent
    pub homing1: f32,

    /// Related to homing in on an opponent
    pub homing2: f32,

    /// Related to homing in on an opponent
    pub homing3: f32,
}

impl SerialisedShrekSuperSlamGameObject for ProjectileType {
    /// Returns the hashcode for the `Game::ProjectileType` in-game object.
    fn hash() -> u32 {
        0x8811292E
    }

    /// Returns name of the in-game class.
    fn name() -> &'static str {
        "Game::ProjectileType"
    }

    /// Returns the size of a serialised `Game::ProjectileType` object.
    fn size() -> usize {
        0x74
    }

    /// Return a new `ProjectileType` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<ProjectileType, Error> {
        let c = bin.console;

        Ok(ProjectileType {
            x_vector: c.read_f32(&bin.raw[offset + 0x08..offset + 0x0C])?,
            angle: c.read_f32(&bin.raw[offset + 0x14..offset + 0x18])?,
            arc: c.read_f32(&bin.raw[offset + 0x18..offset + 0x1C])?,
            homing1: c.read_f32(&bin.raw[offset + 0x44..offset + 0x48])?,
            homing2: c.read_f32(&bin.raw[offset + 0x48..offset + 0x4C])?,
            homing3: c.read_f32(&bin.raw[offset + 0x4C..offset + 0x50])?,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for ProjectileType {
    /// Writes the object back to its `bin` file at the given `offset`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::WriteableShrekSuperSlamGameObject;
    /// use shrek_superslam::classes::ProjectileType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load a projectile from the .bin file, modify the x vector, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC).unwrap();
    /// let mut projectile = bin.get_object_from_offset::<ProjectileType>(0x2000).unwrap();
    /// projectile.x_vector = -1.0;
    /// projectile.write(&mut bin, 0x2000);
    /// ```
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x08..offset + 0x0C, c.write_f32(self.x_vector)?);
        bin.raw
            .splice(offset + 0x14..offset + 0x18, c.write_f32(self.angle)?);
        bin.raw
            .splice(offset + 0x18..offset + 0x1C, c.write_f32(self.arc)?);
        bin.raw
            .splice(offset + 0x44..offset + 0x48, c.write_f32(self.homing1)?);
        bin.raw
            .splice(offset + 0x48..offset + 0x4C, c.write_f32(self.homing2)?);
        bin.raw
            .splice(offset + 0x4C..offset + 0x50, c.write_f32(self.homing3)?);

        Ok(())
    }
}

/// Structure representing the in-game `Game::AttackMoveRegion` object type.
///
/// This type represents a single hitbox generated by an attack or projectile.
#[derive(Deserialize, Serialize)]
pub struct AttackMoveRegion {
    /// The delay (in seconds?) from the attack starting to the hitbox coming out.
    pub delay: f32,

    /// The angle of the hitbox - smaller wraps more around the character.
    pub width: f32,

    /// The height of the hitbox - larger extends out wider.
    pub radius: f32,

    /// Unknown property at offset +10
    pub unknown_010: f32,

    /// Unknown property at offset +24
    pub unknown_024: f32,
}

impl SerialisedShrekSuperSlamGameObject for AttackMoveRegion {
    /// Returns the hashcode for the `Game::AttackMoveRegion` in-game object.
    fn hash() -> u32 {
        0xF2CFE08D
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::AttackMoveRegion"
    }

    /// Returns the size of a serialised Game::AttackMoveRegion object.
    fn size() -> usize {
        0x40
    }

    /// Return a new `AttackMoveRegion` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<AttackMoveRegion, Error> {
        let c = bin.console;

        Ok(AttackMoveRegion {
            delay: c.read_f32(&bin.raw[offset + 0x04..offset + 0x08])?,
            width: c.read_f32(&bin.raw[offset + 0x30..offset + 0x34])?,
            radius: c.read_f32(&bin.raw[offset + 0x38..offset + 0x3C])?,
            unknown_010: c.read_f32(&bin.raw[offset + 0x10..offset + 0x14])?,
            unknown_024: c.read_f32(&bin.raw[offset + 0x24..offset + 0x28])?,
        })
    }
}

impl WriteableShrekSuperSlamGameObject for AttackMoveRegion {
    /// Writes the object back to its `bin` file at the given `offset`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::WriteableShrekSuperSlamGameObject;
    /// use shrek_superslam::classes::AttackMoveRegion;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load a hitbox from the .bin file, modify the width, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC)
    ///                    .unwrap_or_else(|e| panic!("Failed to read bin bytes: {:?}", e));
    /// let mut hitbox = bin.get_object_from_offset::<AttackMoveRegion>(0x1500).unwrap();
    /// hitbox.width = 5.0;
    /// hitbox.write(&mut bin, 0x1500).unwrap();
    /// ```
    fn write(&self, bin: &mut Bin, offset: usize) -> Result<(), Error> {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x04..offset + 0x08, c.write_f32(self.delay)?);
        bin.raw
            .splice(offset + 0x30..offset + 0x34, c.write_f32(self.width)?);
        bin.raw
            .splice(offset + 0x38..offset + 0x3C, c.write_f32(self.radius)?);

        bin.raw
            .splice(offset + 0x10..offset + 0x14, c.write_f32(self.unknown_010)?);
        bin.raw
            .splice(offset + 0x24..offset + 0x28, c.write_f32(self.unknown_024)?);

        Ok(())
    }
}
