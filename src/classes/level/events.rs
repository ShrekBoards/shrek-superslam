use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::Bin;

/// Structure representing the in-game `Game::EventSequence` object type.
///
/// This contains a series of events to be executed in a sequence.
pub struct EventSequence {
    /// The offsets to the events in the sequence.
    pub event_offsets: Vec<u32>,

    /// The raw bytes of the object.
    _bytes: Vec<u8>,
}

impl SerialisedShrekSuperSlamGameObject for EventSequence {
    /// Returns the hashcode for the `Game::EventSequence` in-game object.
    fn hash() -> u32 {
        0xD24634FE
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "Game::EventSequence"
    }

    /// Returns the size of a serialised `Game::EventSequence` object.
    fn size() -> usize {
        0x30
    }

    /// Return a new `EventSequence` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<EventSequence, Error> {
        let c = bin.console;
        let bytes = bin.raw[offset..(offset + Self::size())].to_vec();

        // Read numeric fields
        let event_count = c.read_u32(&bytes[0x08..0x0C])? as usize;

        // Read the offset to the events array, then read each offset in the
        // events array.
        // TODO: We currently only read the offsets rather than deserialising
        //       them to Game::[x]Event types because the game uses polymorphism
        //       for these, and so it could be one of many different event types.
        //       Does the current type system for extracting objects from .bin
        //       files handle this? If not, how can it?
        let events_array_offset = c.read_u32(&bytes[0x04..0x08])? as usize;
        let event_offsets: Result<Vec<u32>, Error> = (0..event_count)
            .map(|i| {
                let event_offset_offset = events_array_offset + Bin::header_length() + (i * 4);
                c.read_u32(&bin.raw[event_offset_offset..event_offset_offset + 4])
            })
            .collect();
        let event_offsets = event_offsets?;

        Ok(EventSequence {
            event_offsets,
            _bytes: bytes
        })
    }
}
