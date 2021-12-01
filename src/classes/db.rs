use crate::classes::SerialisedShrekSuperSlamGameObject;
use crate::errors::Error;
use crate::files::{Bin, BinObject};
use crate::Console;

/// Structure representing the in-game `gf::DB` object type, which is the
/// first object in every .db.bin file, and maps out the rest of the file.
pub struct GfDb {
    /// The filename containing this object
    pub filename: String,

    /// Objects within the DB
    pub objects: Vec<GfDbEntry>,
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
    /// Prefer calling
    /// [Bin::get_object_from_offset<T>()](../../files/struct.Bin.html#method.get_object_from_offset)
    /// rather than calling this method.
    fn new(bin: &Bin, offset: usize) -> Result<GfDb, Error> {
        let raw = &bin.raw;
        let c = bin.console;

        // +08 contains the offset to in the .bin to the string of this DB's filename
        let filename_offset = c.read_u32(&bin.raw[offset + 0x08..offset + 0x0C])?;
        let filename = bin.get_str_from_offset(filename_offset)?;

        // +14 contains the offset to the start of the array of objects in the DB.
        // +18 contains a count of the number of objects in this array.
        let objects_initial_offset =
            0x40 + c.read_u32(&raw[offset + 0x14..offset + 0x18])? as usize;
        let objects_count = c.read_u32(&raw[offset + 0x18..offset + 0x1C])? as usize;
        let objects_end_offset = objects_initial_offset + (objects_count * GfDbEntry::size());

        let objects: Result<Vec<GfDbEntry>, Error> = raw
            [objects_initial_offset..objects_end_offset]
            .chunks(GfDbEntry::size())
            .map(|entry_bytes| GfDbEntry::new(entry_bytes, bin, c))
            .collect();

        Ok(GfDb {
            filename,
            objects: objects?,
        })
    }
}

/// Represents a single entry in the list of objects owned by a gf::DB.
pub struct GfDbEntry {
    /// The name of the object.
    pub name: String,

    /// The object being pointed to by the entry.
    pub object: BinObject,

    // These may never be used
    _unknown1: u32,
    _unknown2: u32,
}

impl GfDbEntry {
    /// Returns the size of a single `gf::DB` entry.
    const fn size() -> usize {
        0x10
    }

    /// Constructs a new object array entry from the given `raw` bytes.
    fn new(raw: &[u8], bin: &Bin, c: Console) -> Result<GfDbEntry, Error> {
        // +00 is the offset in the .bin file to the name of the entry
        let name_offset = c.read_u32(&raw[0x00..0x04])?;
        let name = bin.get_str_from_offset(name_offset)?;

        // +04 is the offset to the actual entry in the .bin file
        let object_offset = c.read_u32(&raw[0x04..0x08])?;
        let object = BinObject::new(&bin.raw, object_offset, c)?;

        Ok(GfDbEntry {
            name,
            object,
            _unknown1: c.read_u32(&raw[0x08..0x0C])?,
            _unknown2: c.read_u32(&raw[0x0C..0x10])?,
        })
    }
}
