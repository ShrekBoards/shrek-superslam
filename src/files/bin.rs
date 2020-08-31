use std::str;

use crate::classes::{hash_lookup, ShrekSuperSlamGameObject};
use crate::console::Console;

/// Structure representing the header (the first 40 bytes) of a .bin file
struct BinHeader {
    pub offset1: u32,
    pub sections: u32,
    pub offset2: u32,
    pub dependencies: u32,
    pub offset4: u32,
}

impl BinHeader {
    /// Create a new BinHeader struct from the given header bytes
    ///
    /// # Parameters
    ///
    /// - `raw`: The first 64 bytes of a .bin file
    /// - `console`: The console the .bin file comes from
    fn new(raw: &[u8], console: Console) -> BinHeader {
        BinHeader {
            offset1: console.read_u32(&raw[0x10..0x14]),
            sections: console.read_u32(&raw[0x18..0x1C]),
            offset2: console.read_u32(&raw[0x1C..0x20]),
            dependencies: console.read_u32(&raw[0x24..0x28]),
            offset4: console.read_u32(&raw[0x2C..0x30]),
        }
    }
}

/// Poorly-named struct that represents the description a 'section' within a
/// .bin - a small 16-byte area that describes and points to a big list of
/// offsets to entries of a certain type within the file
struct BinSection {
    /// Determines the type of each thing being pointed to
    pub number: u32,

    /// The number of pointers in the section
    pub size: u32,

    /// What offset the section begins within the file
    pub offset: u32,
}

impl BinSection {
    /// Create a new BinSection struct from the given section bytes
    ///
    /// # Parameters
    ///
    /// - `raw`: The 16 bytes corresponding to the 'section' in the .bin file
    /// - `console`: The console the .bin comes from
    fn new(raw: &[u8], offset: u32, console: Console) -> BinSection {
        BinSection {
            number: console.read_u32(&raw[0x00..0x04]),
            size: console.read_u32(&raw[0x04..0x08]),
            offset,
        }
    }
}

/// Thin struct that represents the beginning of a serialised Shrek SuperSlam
/// class within a .bin file.
///
/// # Notes
///
/// Use `bin::get_object_from_offset()` using the `offset` field to retrieve
/// the full type from the .bin
pub struct BinObject {
    /// The hash of the object
    pub hash: u32,

    /// The name of the object
    pub name: &'static str,

    /// The offset of the object within the file
    pub offset: u32,
}

impl BinObject {
    /// Create a new BinObject structure
    ///
    /// # Parameters
    ///
    /// - `raw`: The raw bytes of the .bin file
    /// - `offset`: The offset the beginning of the object is at
    /// - `console`: The console the .bin file is from
    ///
    /// # Returns
    ///
    /// Some(BinObject) detailing the object that begins at the offset, or None
    /// if there is no object starting at the given offset
    fn new(raw: &[u8], offset: u32, console: Console) -> Option<BinObject> {
        let hash =
            console.read_u32(&raw[(0x40 + offset) as usize..(0x40 + offset + 0x04) as usize]);
        match hash_lookup(hash) {
            Some(name) => Some(BinObject { hash, name, offset }),
            _ => None,
        }
    }
}

/// Structure for reading and managing a .bin file from the extracted Shrek
/// SuperSlam game files
pub struct Bin {
    _header: BinHeader,
    objects: Vec<BinObject>,
    pub(crate) console: Console,
    pub(crate) raw: Vec<u8>,
}

impl Bin {
    /// Read a .bin file from the decompressed Shrek SuperSlam files
    ///
    /// # Parameters
    ///
    /// - `raw`: The raw bytes of the file
    /// - `console`: The console version the file is from
    pub fn new(raw: Vec<u8>, console: Console) -> Bin {
        // Read the header
        let header = BinHeader::new(&raw[0x00..0x40], console);

        // The offsets and counts within the header are used to calculate
        // various offsets to the different sections within the .bin file
        let file_begin_offset = 0x40;
        let section_begin_offset = file_begin_offset + header.offset1;
        let dependencies_begin_offset = section_begin_offset + (header.sections * 0x10);
        let ptr4_begin_offset = dependencies_begin_offset + (header.dependencies * 0x80);

        // Create an entry for each 'section', which is later used to access
        // different parts of the file
        let mut section_dst_offset = ptr4_begin_offset + (header.offset4 * 0x40);
        let mut sections: Vec<BinSection> = vec![];
        for i in 0..header.sections {
            let section_offset = (section_begin_offset + (i * 0x10)) as usize;
            let next_section_offset = section_offset + 0x10;
            let section = BinSection::new(
                &raw[section_offset..next_section_offset],
                section_dst_offset,
                console,
            );
            let section_size = section.size;
            sections.push(section);

            section_dst_offset += section_size * 4;
        }

        // Create an object for each serialised game object in the .bin
        let mut objects: Vec<BinObject> = vec![];
        for section in sections {
            // The 'section' with a value of 1 in its first field details the
            // number of objects within the .bin file
            if section.number == 1 {
                // This region contains a list of offsets within the file to
                // each object contained within it
                for j in 0..section.size {
                    let object_ptr_offset = (section.offset + (j * 0x04)) as usize;
                    let object_offset =
                        console.read_u32(&raw[object_ptr_offset..(object_ptr_offset + 0x04)]);
                    objects.push(BinObject::new(&raw, object_offset, console).unwrap());
                }
            }
        }

        Bin {
            _header: header,
            console,
            objects,
            raw,
        }
    }

    /// Get an object at an offset within the .bin file as a full deserialised
    /// game object type
    ///
    /// # Parameters
    ///
    /// - `offset`: The offset within the .bin that the object starts at
    ///
    /// # Type parameters
    ///
    /// - `T`: The game object type to deserialise to
    ///
    /// # Returns
    ///
    /// `Ok(T)` if the object exists at the given object and can be deserialised,
    /// otherwise `Err()`.
    pub fn get_object_from_offset<T: ShrekSuperSlamGameObject>(
        &self,
        offset: u32,
    ) -> Result<T, ()> {
        // Ensure the requested type exists at the given offset by checking the
        // hash at the offset matches the expected hash of the type
        let object_begin = (offset + 0x40) as usize;
        let hash = self
            .console
            .read_u32(&self.raw[object_begin..object_begin + 4]);
        if hash != T::hash() {
            return Err(());
        }

        // Pass the offset to the game object's own constructor
        Ok(T::new(&self, object_begin))
    }

    /// Get a string slice from an offset within the .bin file
    ///
    /// # Parameters
    ///
    /// - `offset`: The offset within the .bin file that the string starts at
    ///
    /// # Returns
    ///
    /// `Ok(&str)` if a string exists at the offset, or an `Err(Utf8Error)` if
    /// the string fails to decode
    pub fn get_str_from_offset(&self, offset: u32) -> Result<&str, str::Utf8Error> {
        let str_begin = (offset + 0x40) as usize;

        // Find the first NULL byte, which ends the string. If not found,
        // default to the end of the slice, which will more than likely give us
        // a Utf8Error later
        let slice = &self.raw[str_begin..];
        let size = slice.iter().position(|&b| b == 0x00).unwrap_or(0);

        // Try to decode from the offset to the NULL byte as a UTF-8 string
        str::from_utf8(&self.raw[str_begin..str_begin + size])
    }

    /// # Returns
    ///
    /// A list of objects within the .bin file
    pub fn objects(&self) -> &Vec<BinObject> {
        &self.objects
    }
}
