use crate::classes::util;
use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::AttackMoveType` object type.
///
/// This type represents a single attack, from a player character or from an
/// item.
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

    /// The raw bytes of the object.
    _bytes: Vec<u8>,
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
        let c = bin.console;
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();

        // Read numeric fields
        let endlag = c.read_f32(&bytes[0x04..0x08])?;
        let fall_speed = c.read_f32(&bytes[0x14..0x18])?;
        let name_offset = c.read_u32(&bytes[0x28..0x2C])?;
        let aim_range = c.read_f32(&bytes[0x74..0x78])?;
        let damage1 = c.read_f32(&bytes[0x84..0x88])?;
        let damage2 = c.read_f32(&bytes[0x88..0x8C])?;
        let damage3 = c.read_f32(&bytes[0x8C..0x90])?;
        let charge_effect = c.read_f32(&bytes[0x94..0x98])?;
        let charge = c.read_f32(&bytes[0x98..0x9C])?;
        let stun = c.read_f32(&bytes[0xA4..0xA8])?;
        let knockback = c.read_f32(&bytes[0xAC..0xB0])?;

        let unknown_008 = c.read_f32(&bytes[0x08..0x0C])?;
        let unknown_00c = c.read_f32(&bytes[0x0C..0x10])?;
        let unknown_010 = c.read_f32(&bytes[0x10..0x14])?;
        let unknown_018 = c.read_f32(&bytes[0x18..0x1C])?;
        let unknown_0b0 = c.read_f32(&bytes[0xB0..0xB4])?;
        let unknown_0b4 = c.read_f32(&bytes[0xB4..0xB8])?;
        let unknown_0d8 = c.read_f32(&bytes[0xD8..0xDC])?;
        let unknown_0dc = c.read_f32(&bytes[0xDC..0xE0])?;
        let unknown_0e0 = c.read_f32(&bytes[0xE0..0xE4])?;
        let unknown_0e4 = c.read_f32(&bytes[0xE4..0xE8])?;
        let unknown_0e8 = c.read_f32(&bytes[0xE8..0xEC])?;
        let unknown_0ec = c.read_f32(&bytes[0xEC..0xF0])?;

        // Read boolean flag fields
        let hits_otg = bytes[0x33] != 0;
        let knocks_down = bytes[0x34] != 0;
        let disabled = bytes[0x35] != 0;
        let intangible = bytes[0x3A] != 0;

        // The list of hitboxes is at +20.
        // The count of hitboxes is at +24.
        let hitboxes = util::construct_array(bin, offset, c, 0x20, 0x24)?;

        // The offset to the projectile, if any, is +9C.
        let projectile = util::construct_optional_type(bin, offset, c, 0x9C)?;

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
            stun,
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
            _bytes: bytes,
        })
    }
}

/// Structure representing the in-game `Game::ProjectileType` object type.
///
/// This type represents a projectile generated by an attack.
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

    /// The raw bytes of the object.
    _bytes: Vec<u8>,
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
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();

        Ok(ProjectileType {
            x_vector: c.read_f32(&bytes[0x08..0x0C])?,
            angle: c.read_f32(&bytes[0x14..0x18])?,
            arc: c.read_f32(&bytes[0x18..0x1C])?,
            homing1: c.read_f32(&bytes[0x44..0x48])?,
            homing2: c.read_f32(&bytes[0x48..0x4C])?,
            homing3: c.read_f32(&bytes[0x4C..0x50])?,
            _bytes: bytes,
        })
    }
}

/// Structure representing the in-game `Game::AttackMoveRegion` object type.
///
/// This type represents a single hitbox generated by an attack or projectile.
pub struct AttackMoveRegion {
    /// The delay (in seconds?) from the attack starting to the hitbox coming out.
    pub delay: f32,

    /// The angle of the hitbox - smaller wraps more around the character.
    pub width: f32,

    /// The height of the hitbox - larger extends out wider.
    pub radius: f32,

    /// The raw bytes of the object.
    pub bytes: Vec<u8>,
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
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();

        Ok(AttackMoveRegion {
            delay: c.read_f32(&bytes[0x04..0x08])?,
            width: c.read_f32(&bytes[0x30..0x34])?,
            radius: c.read_f32(&bytes[0x38..0x3C])?,
            bytes,
        })
    }
}
