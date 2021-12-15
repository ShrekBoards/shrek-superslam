use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::{Bin, BinObject};
use crate::Console;

/// Structure representing the in-game `gf::DB` object type.
///
/// This type is the first object in every .db.bin file, and maps out the rest
/// of the file.
pub struct GfDb {
    /// Objects within the DB
    pub entries: Vec<(String, BinObject)>,
}

impl SerialisedShrekSuperSlamGameObject for GfDb {
    /// Returns the hashcode for the `gf::DB` in-game object.
    fn hash() -> u32 {
        0x9B3DDBED
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "gf::DB"
    }

    /// Returns the size of a serialised `gf::DB` object.
    fn size() -> usize {
        0x24
    }

    /// Return a new `GfDb` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<GfDb, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // +14 contains the offset to the start of the array of objects in the DB.
        // +18 contains a count of the number of objects in this array.
        const OBJECT_ENTRY_SIZE: usize = 0x10;
        let objects_initial_offset =
            Bin::header_length() + c.read_u32(&raw[offset + 0x14..offset + 0x18])? as usize;
        let objects_count = c.read_u32(&raw[offset + 0x18..offset + 0x1C])? as usize;
        let objects_end_offset = objects_initial_offset + (objects_count * OBJECT_ENTRY_SIZE);

        let objects: Result<Vec<(String, BinObject)>, Error> = raw
            [objects_initial_offset..objects_end_offset]
            .chunks(OBJECT_ENTRY_SIZE)
            .map(|entry_bytes| create_object_entry(entry_bytes, bin, c))
            .collect();

        Ok(GfDb { entries: objects? })
    }
}

/// Structure representing the in-game `GF_TEMP::ScriptDB` object type.
///
/// This type behaves very similarly to the `gf::DB`, perhaps it is used for
/// scripted events?
pub struct ScriptDb {
    /// Objects within the DB
    pub entries: Vec<(String, BinObject)>,
}

impl SerialisedShrekSuperSlamGameObject for ScriptDb {
    /// Returns the hashcode for the `GF_TEMP::ScriptDb` in-game object.
    fn hash() -> u32 {
        0xA128E61A
    }

    /// Returns the name of the in-game class.
    fn name() -> &'static str {
        "GF_TEMP::ScriptDB"
    }

    /// Returns the size of a serialised `GF_TEMP::ScriptDb` object.
    fn size() -> usize {
        0x24
    }

    /// Return a new `ScriptDb` using data located at the given
    /// `offset` in the given `bin` file structure.
    ///
    /// # Remarks
    ///
    /// Prefer calling [`Bin::get_object_from_offset`] rather than calling
    /// this method.
    fn new(bin: &Bin, offset: usize) -> Result<ScriptDb, Error> {
        // This object works the same way as the gf::DB type, so we'll just
        // re-use the code for that.
        let db = GfDb::new(bin, offset)?;

        Ok(ScriptDb { entries: db.entries })
    }
}

/// Extract the `raw` bytes of a single gf::DB array entry into its actual
/// parts, and resolve them using the rest of the `bin`.
fn create_object_entry(raw: &[u8], bin: &Bin, c: Console) -> Result<(String, BinObject), Error> {
    // +00 is the offset in the .bin file to the name of the entry
    let name_offset = c.read_u32(&raw[0x00..0x04])?;
    let name = bin.get_str_from_offset(name_offset)?;

    // +04 is the offset to the actual entry in the .bin file
    let object_offset = c.read_u32(&raw[0x04..0x08])?;
    let object = BinObject::new(&bin.raw, object_offset, c)?;

    Ok((name, object))
}
