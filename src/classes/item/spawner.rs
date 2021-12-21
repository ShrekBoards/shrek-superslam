use crate::classes::item::{PotionType, PowerupType, WeaponType};
use crate::classes::util;
use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::ItemSpawner` object type.
///
/// ItemSpawners are used to spawn various item types throughout the level.
pub struct ItemSpawner {
    /// The list of weapons spawned by the item spawner.
    pub weapons: Vec<WeaponType>,

    /// The list of potions spawned by the item spawner.
    pub potions: Vec<PotionType>,

    /// The list of powerups spawned by the item spawner.
    pub powerups: Vec<PowerupType>,
}

impl SerialisedShrekSuperSlamGameObject for ItemSpawner {
    /// Returns the hashcode for the `Game::ItemSpawner` in-game object.
    fn hash() -> u32 {
        0xCD47AA2B
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::ItemSpawner"
    }

    /// Returns the size of a serialised `Game::ItemSpawner` object.
    fn size() -> usize {
        0x688
    }

    /// Return a new `ItemSpawner` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<ItemSpawner, Error> {
        // The list of weapons created by the spawner is at +2C.
        // The size of this array is at +30.
        let weapons = util::construct_array(bin, offset, bin.console, 0x2C, 0x30)?;

        // The list of potions created by the spawner is at +34.
        // The size of this array is at +38.
        let potions = util::construct_array(bin, offset, bin.console, 0x34, 0x38)?;

        // The list of powerups created by the spawner is at +3C.
        // The size of this array is at +40.
        let powerups = util::construct_array(bin, offset, bin.console, 0x3C, 0x40)?;

        Ok(ItemSpawner {
            weapons,
            potions,
            powerups,
        })
    }
}
