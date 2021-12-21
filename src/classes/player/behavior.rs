use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::BehaviorFightingControlShrek` object type.
///
/// This object controls some kind of fighting behaviour, likely at runtime.
pub struct BehaviorFightingControlShrek {}

impl SerialisedShrekSuperSlamGameObject for BehaviorFightingControlShrek {
    /// Returns the hashcode for the `Game::BehaviorFightingControlShrek` in-game object.
    fn hash() -> u32 {
        0xD306D805
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::BehaviorFightingControlShrek"
    }

    /// Returns the size of a serialised `Game::BehaviorFightingControlShrek` object.
    fn size() -> usize {
        0xF0
    }

    /// Return a new `BehaviorFightingControlShrek` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<BehaviorFightingControlShrek, Error> {
        Ok(BehaviorFightingControlShrek {})
    }
}

/// Structure representing the in-game `Game::BehaviorAIFighting` object type.
///
/// This object controls some kind of fighting behaviour, likely at runtime,
/// and likely for AI rather than player characters.
pub struct BehaviorAiFighting {}

impl SerialisedShrekSuperSlamGameObject for BehaviorAiFighting {
    /// Returns the hashcode for the `Game::BehaviorAIFighting` in-game object.
    fn hash() -> u32 {
        0xE2AD9980
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::BehaviorAIFighting"
    }

    /// Returns the size of a serialised `Game::BehaviorAIFighting` object.
    fn size() -> usize {
        0x350
    }

    /// Return a new `BehaviorAIFighting` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(_bin: &Bin, _offset: usize) -> Result<BehaviorAiFighting, Error> {
        Ok(BehaviorAiFighting {})
    }
}
