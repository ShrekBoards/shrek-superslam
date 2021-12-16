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

    /// The amount the attack knocks the opponent back. Positive pushes away, negative pulls toward.
    pub knockback: f32,

    /// True if the attack knocks the opponent down, false if not.
    pub knocks_down: bool,

    /// The attack's name.
    pub name: String,

    /// The projectile type the attack spawns, if any.
    pub projectile: Option<ProjectileType>,

    /// True if the attack breaks shield (block) on contact, false if not.
    pub shield_breaks: bool,

    /// The amount of time the attack stuns for, in seconds.
    pub stun: f32,

    /// Unknown property at offset +008
    pub unknown_008: f32,

    /// Unknown property at offset +00C
    pub unknown_00c: f32,

    /// Unknown property at offset +010
    pub unknown_010: f32,

    /// Unknown property at offset +018
    pub unknown_018: f32,

    /// Unknown property at offset +0B0
    pub unknown_0b0: f32,

    /// Unknown property at offset +0B4
    pub unknown_0b4: f32,

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
        let fall_speed = c.read_f32(&raw[offset + 0x14..offset + 0x18])?;
        let name_offset = c.read_u32(&raw[offset + 0x28..offset + 0x2C])?;
        let aim_range = c.read_f32(&raw[offset + 0x74..offset + 0x78])?;
        let damage1 = c.read_f32(&raw[offset + 0x84..offset + 0x88])?;
        let damage2 = c.read_f32(&raw[offset + 0x88..offset + 0x8C])?;
        let damage3 = c.read_f32(&raw[offset + 0x8C..offset + 0x90])?;
        let charge_effect = c.read_f32(&raw[offset + 0x94..offset + 0x98])?;
        let charge = c.read_f32(&raw[offset + 0x98..offset + 0x9C])?;
        let stun = c.read_f32(&raw[offset + 0xA4..offset + 0xA8])?;
        let knockback = c.read_f32(&raw[offset + 0xAC..offset + 0xB0])?;

        let unknown_008 = c.read_f32(&raw[offset + 0x08..offset + 0x0C])?;
        let unknown_00c = c.read_f32(&raw[offset + 0x0C..offset + 0x10])?;
        let unknown_010 = c.read_f32(&raw[offset + 0x10..offset + 0x14])?;
        let unknown_018 = c.read_f32(&raw[offset + 0x18..offset + 0x1C])?;
        let unknown_0b0 = c.read_f32(&raw[offset + 0xB0..offset + 0xB4])?;
        let unknown_0b4 = c.read_f32(&raw[offset + 0xB4..offset + 0xB8])?;
        let unknown_0d8 = c.read_f32(&raw[offset + 0xD8..offset + 0xDC])?;
        let unknown_0dc = c.read_f32(&raw[offset + 0xDC..offset + 0xE0])?;
        let unknown_0e0 = c.read_f32(&raw[offset + 0xE0..offset + 0xE4])?;
        let unknown_0e4 = c.read_f32(&raw[offset + 0xE4..offset + 0xE8])?;
        let unknown_0e8 = c.read_f32(&raw[offset + 0xE8..offset + 0xEC])?;
        let unknown_0ec = c.read_f32(&raw[offset + 0xEC..offset + 0xF0])?;

        // Read boolean flag fields
        let shield_breaks = raw[offset + 0x2E] != 0;
        let hits_otg = raw[offset + 0x33] != 0;
        let knocks_down = raw[offset + 0x34] != 0;
        let disabled = raw[offset + 0x35] != 0;
        let intangible = raw[offset + 0x3A] != 0;

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
            knocks_down,
            knockback,
            name: bin.get_str_from_offset(name_offset)?,
            projectile,
            shield_breaks,
            stun,
            hitbox_offsets,
            projectile_offset,
            unknown_008,
            unknown_00c,
            unknown_010,
            unknown_018,
            unknown_0b0,
            unknown_0b4,
            unknown_0d8,
            unknown_0dc,
            unknown_0e0,
            unknown_0e4,
            unknown_0e8,
            unknown_0ec,
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
            .splice(offset + 0x14..offset + 0x18, c.write_f32(self.fall_speed)?);
        bin.raw[offset + 0x2E] = self.shield_breaks as u8;
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
            .splice(offset + 0xA4..offset + 0xA8, c.write_f32(self.stun)?);
        bin.raw
            .splice(offset + 0xAC..offset + 0xB0, c.write_f32(self.knockback)?);

        // Unknown fields
        bin.raw
            .splice(offset + 0x08..offset + 0x0C, c.write_f32(self.unknown_008)?);
        bin.raw
            .splice(offset + 0x0C..offset + 0x10, c.write_f32(self.unknown_00c)?);
        bin.raw
            .splice(offset + 0x10..offset + 0x14, c.write_f32(self.unknown_010)?);
        bin.raw
            .splice(offset + 0x18..offset + 0x1C, c.write_f32(self.unknown_018)?);
        bin.raw
            .splice(offset + 0xB0..offset + 0xB4, c.write_f32(self.unknown_0b0)?);
        bin.raw
            .splice(offset + 0xB4..offset + 0xB8, c.write_f32(self.unknown_0b4)?);
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

        Ok(())
    }
}
