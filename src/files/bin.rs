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
            offset1: console.read32(&raw[0x10..0x14]),
            sections: console.read32(&raw[0x18..0x1C]),
            offset2: console.read32(&raw[0x1C..0x20]),
            dependencies: console.read32(&raw[0x24..0x28]),
            offset4: console.read32(&raw[0x2C..0x30]),
        }
    }
}

/// Poorly-named struct that represents a 'section' within a bin, that serves
/// as a kind of map to other parts of the file?
struct BinSection {
    pub number: u32,
    pub size: u32,
}

impl BinSection {
    /// Create a new BinSection struct from the given section bytes
    ///
    /// # Parameters
    ///
    /// - `raw`: The 16 bytes corresponding to the 'section' in the .bin file
    /// - `console`: The console the .bin comes from
    fn new(raw: &[u8], console: Console) -> BinSection {
        BinSection {
            number: console.read32(&raw[0x00..0x04]),
            size: console.read32(&raw[0x04..0x08]),
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
        let hash = console.read32(&raw[(0x40 + offset) as usize..(0x40 + offset + 0x04) as usize]);
        match hash_lookup(hash) {
            Some(name) => Some(BinObject { hash, name, offset }),
            _ => None,
        }
    }
}

/// Structure for reading and managing a .bin file from the extracted Shrek
/// SuperSlam game files
pub struct Bin {
    _raw: Vec<u8>,
    _header: BinHeader,
    objects: Vec<BinObject>,
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
        let mut object_ptrs_begin_offset = ptr4_begin_offset + (header.offset4 * 0x40);

        // Create an object for each serialised game object in the .bin
        let mut objects: Vec<BinObject> = vec![];
        for i in 0..header.sections {
            // The 'section' with a value of 1 in its first field details the
            // number of objects within the .bin file
            let section_offset = (section_begin_offset + (i * 0x10)) as usize;
            let next_section_offset = section_offset + 0x10;
            let section = BinSection::new(&raw[section_offset..next_section_offset], console);
            if section.number == 1 {
                // This region contains a list of offsets within the file to
                // each object contained within it
                for j in 0..section.size {
                    let object_ptr_offset = (object_ptrs_begin_offset + (j * 0x04)) as usize;
                    let object_offset =
                        console.read32(&raw[object_ptr_offset..(object_ptr_offset + 0x04)]);
                    objects.push(BinObject::new(&raw, object_offset, console).unwrap());
                }
            }
            object_ptrs_begin_offset += section.size * 4;
        }

        Bin {
            _raw: raw,
            _header: header,
            objects,
        }
    }

    pub fn get_object_from_offset<T: ShrekSuperSlamGameObject>(_offset: u32) -> Result<T, ()> {
        Err(())
    }

    /// # Returns
    ///
    /// A list of objects within the .bin file
    pub fn objects(&self) -> &Vec<BinObject> {
        &self.objects
    }
}
