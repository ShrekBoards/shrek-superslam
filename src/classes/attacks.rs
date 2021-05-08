use serde::{Deserialize, Serialize};

use crate::classes::{SerialisedShrekSuperSlamGameObject, WriteableShrekSuperSlamGameObject};
use crate::errors::Error;
use crate::files::Bin;
use crate::Console;

/// Structure representing the in-game `Game::AttackMoveType` object type,
/// which represents an attack (from a character or an item).
#[derive(Deserialize, Serialize)]
pub struct AttackMoveType {
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

    /// True if the attack knocks the opponent down, false if not.
    pub knocks_down: bool,

    /// The attack's name.
    pub name: String,

    /// The projectile type the attack spawns, if any.
    pub projectile: Option<ProjectileType>,

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
    /// Prefer calling
    /// [Bin::get_object_from_offset<T>()](../../files/struct.Bin.html#method.get_object_from_offset)
    /// rather than calling this method.
    fn new(bin: &Bin, offset: usize) -> Result<AttackMoveType, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // Read numeric fields
        let endlag = c.read_f32(&raw[offset + 0x04..offset + 0x08])?;
        let fall_speed = c.read_f32(&raw[offset + 0x14..offset + 0x18])?;
        let damage1 = c.read_f32(&raw[offset + 0x84..offset + 0x88])?;
        let damage2 = c.read_f32(&raw[offset + 0x88..offset + 0x8C])?;
        let damage3 = c.read_f32(&raw[offset + 0x8C..offset + 0x90])?;
        let name_offset = c.read_u32(&raw[offset + 0x28..offset + 0x2C])?;

        // Read boolean flag fields
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
        let projectile = match projectile_offset {
            Some(x) => Some(bin.get_object_from_offset::<ProjectileType>(x)?),
            _ => None,
        };

        // Read the list of hitbox offsets, and use those to read each hitbox
        let hitbox_offsets = AttackMoveType::hitbox_offsets(&raw, offset, c)?;
        let hitboxes = hitbox_offsets
            .iter()
            .map(|o| bin.get_object_from_offset::<AttackMoveRegion>(*o).unwrap())
            .collect();

        Ok(AttackMoveType {
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
            name: bin.get_str_from_offset(name_offset)?,
            projectile,
            hitbox_offsets,
            projectile_offset,
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
    /// use shrek_superslam::classes::attacks::AttackMoveType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load an attack from the .bin file, modify the damage, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC);
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
        bin.raw[offset + 0x33] = self.hits_otg as u8;
        bin.raw[offset + 0x34] = self.knocks_down as u8;
        bin.raw[offset + 0x35] = self.disabled as u8;
        bin.raw[offset + 0x3A] = self.intangible as u8;
        bin.raw
            .splice(offset + 0x84..offset + 0x88, c.write_f32(self.damage1)?);
        bin.raw
            .splice(offset + 0x88..offset + 0x8C, c.write_f32(self.damage2)?);
        bin.raw
            .splice(offset + 0x8C..offset + 0x90, c.write_f32(self.damage3)?);

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
                .map(|o| o + 0x40)
                .collect()
        } else {
            self.hitbox_offsets.clone()
        };

        for (offset, hitbox) in hitbox_offsets.iter().zip(self.hitboxes.iter()) {
            hitbox.write(bin, *offset as usize);
        }

        // Write the attack's projectile, if any, back to the .bin file too
        if self.projectile.is_some() && self.projectile_offset.is_some() {
            self.projectile
                .as_ref()
                .unwrap()
                .write(bin, self.projectile_offset.unwrap() as usize);
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
        let num_hitboxes = AttackMoveType::number_of_hitboxes(&raw, offset, console);
        let regions_offset = console.read_u32(&raw[offset + 0x20..offset + 0x24])?;
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
    fn number_of_hitboxes(raw: &[u8], offset: usize, console: Console) -> Result<u32, Error> {
        Ok(console.read_u32(&raw[offset + 0x24..offset + 0x28])?)
    }
}

/// Structure representing the in-game `Game::ProjectileType` object type,
/// which represents a projectile generated by an attack.
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
    /// Prefer calling
    /// [Bin::get_object_from_offset<T>()](../../files/struct.Bin.html#method.get_object_from_offset)
    /// rather than calling this method.
    fn new(bin: &Bin, offset: usize) -> ProjectileType {
        let c = bin.console;

        ProjectileType {
            x_vector: c.read_f32(&bin.raw[offset + 0x08..offset + 0x0C]),
            angle: c.read_f32(&bin.raw[offset + 0x14..offset + 0x18]),
            arc: c.read_f32(&bin.raw[offset + 0x18..offset + 0x1C]),
            homing1: c.read_f32(&bin.raw[offset + 0x44..offset + 0x48]),
            homing2: c.read_f32(&bin.raw[offset + 0x48..offset + 0x4C]),
            homing3: c.read_f32(&bin.raw[offset + 0x4C..offset + 0x50]),
        }
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
    /// use shrek_superslam::classes::attacks::ProjectileType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load a projectile from the .bin file, modify the x vector, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC);
    /// let mut projectile = bin.get_object_from_offset::<ProjectileType>(0x2000).unwrap();
    /// projectile.x_vector = -1.0;
    /// projectile.write(&mut bin, 0x2000);
    /// ```
    fn write(&self, bin: &mut Bin, offset: usize) {
        // Write back only fixed-length numeric fields to the new object - other
        // fields such as strings would modify the size of the file and
        // invalidate all offsets
        let c = bin.console;
        bin.raw
            .splice(offset + 0x08..offset + 0x0C, c.write_f32(self.x_vector));
        bin.raw
            .splice(offset + 0x14..offset + 0x18, c.write_f32(self.angle));
        bin.raw
            .splice(offset + 0x18..offset + 0x1C, c.write_f32(self.arc));
        bin.raw
            .splice(offset + 0x44..offset + 0x48, c.write_f32(self.homing1));
        bin.raw
            .splice(offset + 0x48..offset + 0x4C, c.write_f32(self.homing2));
        bin.raw
            .splice(offset + 0x4C..offset + 0x50, c.write_f32(self.homing3));
    }
}

/// Structure representing the in-game `Game::AttackMoveRegion` object type,
/// which represents an attack's hitbox.
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
    /// Prefer calling
    /// [Bin::get_object_from_offset<T>()](../../files/struct.Bin.html#method.get_object_from_offset)
    /// rather than calling this method.
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
    /// Writes the object back to its `bin` file at the given `offset`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::WriteableShrekSuperSlamGameObject;
    /// use shrek_superslam::classes::attacks::AttackMoveRegion;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Load a hitbox from the .bin file, modify the width, and write it back
    /// # let my_bin_bytes = vec![0x00, 0x01, 0x02];
    /// let mut bin = Bin::new(my_bin_bytes, Console::PC);
    /// let mut hitbox = bin.get_object_from_offset::<AttackMoveRegion>(0x1500).unwrap();
    /// hitbox.width = 5.0;
    /// hitbox.write(&mut bin, 0x1500);
    /// ```
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
