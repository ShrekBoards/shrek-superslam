use crate::classes::player::{BehaviorFightingControlShrek, PhysicsFighting, RenderSpawn};
use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::Entity` object type.
///
/// Used as the entry point to all the runtime information about a character.
pub struct Entity {
    /// The player behaviour object.
    pub behaviour: BehaviorFightingControlShrek,

    /// The name of the character.
    pub name: String,

    /// The player physics.
    pub physics: PhysicsFighting,

    /// The player render.
    pub render: RenderSpawn,

    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for Entity {
    /// Returns the hashcode for the `Game::Entity` in-game object.
    fn hash() -> u32 {
        0xDDEC024E
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::Entity"
    }

    /// Returns the size of a serialised `Game::Entity` object.
    fn size() -> usize {
        0x280
    }

    /// Return a new `Entity` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<Entity, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // The name of the character in plaintext is stored at +04.
        let name_offset = c.read_u32(&raw[offset + 0x04..offset + 0x08])?;

        // Offsets to more specific runtime types are at:
        // +08 - Game::BehaviorFightingControlShrek
        // +0C - Game::RenderSpawn
        // +10 - Game::PhysicsFighting
        let behavior_offset = c.read_u32(&raw[offset + 0x08..offset + 0x0C])?;
        let render_offset = c.read_u32(&raw[offset + 0x0C..offset + 0x10])?;
        let physics_offset = c.read_u32(&raw[offset + 0x10..offset + 0x14])?;

        Ok(Entity {
            behaviour: bin.get_object_from_offset(behavior_offset)?,
            name: bin.get_str_from_offset(name_offset)?,
            physics: bin.get_object_from_offset(physics_offset)?,
            render: bin.get_object_from_offset(render_offset)?,
            _bytes: bin.raw[offset..(offset + Self::size())].to_vec(),
        })
    }
}
