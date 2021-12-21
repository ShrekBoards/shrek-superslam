use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};

use crate::classes;
use crate::classes::*;
use crate::console::Console;
use crate::errors::Error;

use std::collections::HashMap;

/// Structure representing the header (the first 40 bytes) of a .bin file
struct BinHeader {
    pub offset1: u32,
    pub sections: u32,
    pub offset2: u32,
    pub dependencies: u32,
    pub offset4: u32,
}

impl BinHeader {
    /// Create a new BinHeader struct from the given `raw` header bytes from
    /// the given `console` platform.
    fn new(raw: &[u8], console: Console) -> Result<BinHeader, Error> {
        Ok(BinHeader {
            offset1: console.read_u32(&raw[0x10..0x14])?,
            sections: console.read_u32(&raw[0x18..0x1C])?,
            offset2: console.read_u32(&raw[0x1C..0x20])?,
            dependencies: console.read_u32(&raw[0x24..0x28])?,
            offset4: console.read_u32(&raw[0x2C..0x30])?,
        })
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
    /// Create a new BinSection struct from the given `raw` section bytes for
    /// the given `console` platform.
    fn new(raw: &[u8], offset: u32, console: Console) -> Result<BinSection, Error> {
        Ok(BinSection {
            number: console.read_u32(&raw[0x00..0x04])?,
            size: console.read_u32(&raw[0x04..0x08])?,
            offset,
        })
    }
}

/// Thin structure that represents the beginning of a serialised Shrek SuperSlam
/// class within a .bin file.
///
/// # Notes
///
/// Use `offset` field as a parameter to the [`Bin::get_object_from_offset`]
/// function to get the full deserialised object from the .bin file.
pub struct BinObject {
    /// The hash of the object
    pub hash: u32,

    /// The name of the object
    pub name: &'static str,

    /// The offset of the object within the file
    pub offset: u32,
}

impl BinObject {
    /// Create a new BinObject structure from the given `offset` in the `raw`
    /// bytes of the entire .bin file from the given `console` version.
    pub fn new(raw: &[u8], offset: u32, console: Console) -> Result<BinObject, Error> {
        let hash = console.read_u32(
            &raw[Bin::header_length() + offset as usize
                ..Bin::header_length() + offset as usize + 0x04],
        )?;

        if let Some(name) = hash_lookup(hash) {
            Ok(BinObject { hash, name, offset })
        } else {
            Err(Error::ClassDeserialiseError(
                classes::Error::IncorrectType { hash },
            ))
        }
    }
}

/// Structure for reading and modifying a .bin file from the extracted Shrek
/// SuperSlam game files.
///
/// These files within the game are primarily a collection of serialised objects
/// of in-game class types relating to a particular subject, such as a playable
/// character or an item. This structure provides an abstraction around these files,
/// and provides methods for deserialising and extracting the objects contained
/// within them, as well as limited support for rewriting modified copies of these
/// objects back to the file.
///
/// For more information about the available classes, see the [classes](../classes/index.html)
/// module, which contains structures representing the classes found within these
/// .bin files.
pub struct Bin {
    pub(crate) objects: Vec<BinObject>,
    pub(crate) console: Console,
    pub(crate) raw: Vec<u8>,
}

impl Bin {
    /// Returns the length of the .bin file header.
    pub(crate) const fn header_length() -> usize {
        0x40
    }

    /// Construct a new `Bin` object from the given `raw` bytes of a
    /// decompressed .bin file, from the given `console` version.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    /// use shrek_superslam::files::Bin;
    ///
    /// let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// let my_file_bytes = master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap();
    /// let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// ```
    pub fn new(raw: Vec<u8>, console: Console) -> Result<Bin, Error> {
        // Read the header
        let header = BinHeader::new(&raw[0x00..Bin::header_length()], console)?;

        // The offsets and counts within the header are used to calculate
        // various offsets to the different sections within the .bin file
        let file_begin_offset = Bin::header_length() as u32;
        let section_begin_offset = file_begin_offset + header.offset1;
        let dependencies_begin_offset = section_begin_offset + (header.sections * 0x10);
        let ptr4_begin_offset = dependencies_begin_offset + (header.dependencies * 0x80);

        // Create an entry for each 'section', which is later used to access
        // different parts of the file
        let mut section_dst_offset =
            ptr4_begin_offset + (header.offset4 * Bin::header_length() as u32);
        let mut sections: Vec<BinSection> = vec![];
        for i in 0..header.sections {
            let section_offset = (section_begin_offset + (i * 0x10)) as usize;
            let next_section_offset = section_offset + 0x10;
            let section = BinSection::new(
                &raw[section_offset..next_section_offset],
                section_dst_offset,
                console,
            )?;
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
                        console.read_u32(&raw[object_ptr_offset..(object_ptr_offset + 0x04)])?;
                    let obj = BinObject::new(&raw, object_offset, console)?;
                    objects.push(obj);
                }
            }
        }

        Ok(Bin {
            objects,
            console,
            raw,
        })
    }

    /// Get all objects of a requested type `T` contained within the .bin file.
    ///
    /// Returns a list of tuples containing the offset of the object within the
    /// file, and the deserialised object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::AttackMoveType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Get all Game::AttackMoveType objects contained within the .bin file
    /// # let my_file_bytes: Vec<u8> = vec![];
    /// let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// let attacks = bin.get_all_objects_of_type::<AttackMoveType>();
    /// for (offset, attack) in attacks {
    ///     println!("Attack at offset {} is {}, which deals {} damage",
    ///         offset,
    ///         attack.name,
    ///         attack.damage1
    ///     );
    /// }
    /// ```
    pub fn get_all_objects_of_type<T>(&self) -> Vec<(u32, T)>
    where
        T: SerialisedShrekSuperSlamGameObject,
    {
        self.objects()
            .iter()
            .filter(|o| o.hash == T::hash())
            .map(|o| {
                (
                    o.offset,
                    self.get_object_from_offset::<T>(o.offset).unwrap(),
                )
            })
            .collect()
    }

    /// Returns a deserialised object of type `T` contained at given `offset`
    /// within the .bin file.
    ///
    /// # Errors
    ///
    /// If the given `offset` does not contain the start of the requested type,
    /// or the file does not have enough space to contain the object from the
    /// given offset, then an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::classes::AttackMoveType;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Get a specific Game::AttackMoveType object located in the .bin file
    /// # let my_file_bytes: Vec<u8> = vec![];
    /// let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// let attack = bin.get_object_from_offset::<AttackMoveType>(0x1000).unwrap();
    /// println!("Attack at offset {} is {}, which deals {} damage",
    ///     0x1000,
    ///     attack.name,
    ///     attack.damage1
    /// );
    /// ```
    pub fn get_object_from_offset<T>(&self, offset: u32) -> Result<T, Error>
    where
        T: SerialisedShrekSuperSlamGameObject,
    {
        // Ensure there are enough bytes for the requested type to fit before
        // we try and make a slice for it
        if offset as usize + T::size() > self.raw.len() {
            return Err(classes::Error::NotEnoughBytes {
                requested: T::size(),
                file_size: self.raw.len(),
                offset: offset as usize,
            }
            .into());
        }

        // Ensure the requested type exists at the given offset by checking the
        // hash at the offset matches the expected hash of the type
        let object_begin = offset as usize + Bin::header_length();
        let hash = self
            .console
            .read_u32(&self.raw[object_begin..object_begin + 4])?;
        if hash != T::hash() {
            return Err(classes::Error::IncorrectType { hash }.into());
        }

        // Pass the offset to the game object's own constructor
        T::new(&self, object_begin)
    }

    /// Returns a string from the given `offset` within the .bin file.
    ///
    /// # Errors
    ///
    /// If the bytes at the given `offset` fail to decode as an ISO 8859-1
    /// string, then an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Bin;
    ///
    /// // Get a specific string located in the .bin file
    /// # let my_file_bytes: Vec<u8> = vec![];
    /// let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// let my_string = bin.get_str_from_offset(0x500).unwrap();
    /// println!("At offset {}, there is the string '{}'", 0x500, my_string);
    /// ```
    pub fn get_str_from_offset(&self, offset: u32) -> Result<String, Error> {
        let str_begin = offset as usize + Bin::header_length();

        // Find the first NULL byte, which ends the string. If not found,
        // default to the end of the slice, which will more than likely give us
        // an Error later
        let slice = &self.raw[str_begin..];
        let size = slice.iter().position(|&b| b == 0x00).unwrap_or(0);

        // Text within the game is stored using the single-byte ISO 8859-1
        // encoding. Specifically, $AE = ®. We therefore need to decode it
        Ok(ISO_8859_1.decode(&self.raw[str_begin..str_begin + size], DecoderTrap::Strict)?)
    }

    /// Overwrite an existing object at the given `offset` with the new object
    /// given in the `object` parameter.
    ///
    /// # Errors
    ///
    /// If the given `offset` does not contain the beginning of an object of
    /// the given type, then an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Bin;
    /// use shrek_superslam::classes::AttackMoveType;
    ///
    /// // Overwrite the damage of a specific Game::AttackMoveType object
    /// # let my_file_bytes: Vec<u8> = vec![];
    /// let mut bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// let mut attack = bin.get_object_from_offset::<AttackMoveType>(0x1000).unwrap();
    /// attack.damage1 = 100.0;
    /// bin.overwrite_object(0x1000, &attack);
    /// ```
    pub fn overwrite_object<T>(&mut self, offset: u32, object: &T) -> Result<(), Error>
    where
        T: SerialisedShrekSuperSlamGameObject + WriteableShrekSuperSlamGameObject,
    {
        // Check that the given offset actually contains an object of the type
        // given as a parameter before we overwrite it
        let object_begin = offset as usize + Bin::header_length();
        let hash = self
            .console
            .read_u32(&self.raw[object_begin..object_begin + 4])?;
        if hash != T::hash() {
            return Err(classes::Error::IncorrectType { hash }.into());
        }

        object.write(self, object_begin)?;

        Ok(())
    }

    /// Parse a .db.bin file to its named entries.
    ///
    /// Use this function to parse a .db.bin file as the game would - by
    /// reading the `gf::DB` object at the top of the file, parsing out each
    /// object in it along with its name, and resolving each reference to an
    /// actual game type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use shrek_superslam::{Console, MasterDat, MasterDir};
    /// # use shrek_superslam::classes::ShrekSuperSlamObject;
    /// # use shrek_superslam::files::Bin;
    /// #
    /// # let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// # let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    /// # let my_file_bytes = master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap();
    /// # let bin = Bin::new(my_file_bytes, Console::PC).unwrap();
    /// // Get the 'Fast1Atk' object from a player .db.bin, and print the attack damage.
    /// let parsed = bin.parse().unwrap();
    /// if let Some(obj) = parsed.get("Fast1Atk") {
    ///     if let ShrekSuperSlamObject::AttackMoveType(fast_1_atk) = obj {
    ///         println!("Fast1Atk damage: {}", fast_1_atk.damage1);
    ///     }
    /// }
    /// ```
    pub fn parse(&self) -> Result<HashMap<String, ShrekSuperSlamObject>, Error> {
        // Get the gf::DB object, which resides at offset 0 always.
        let db = self.get_object_from_offset::<GfDb>(0x00)?;

        // Resolve each item in the DB to an actual type, and add it to the
        // hashmap along with its name.
        let mut mapping: HashMap<String, ShrekSuperSlamObject> = HashMap::new();
        for (name, object) in db.entries {
            mapping.insert(name, self.resolve_object(&object)?);
        }

        Ok(mapping)
    }

    /// Resolve a stub .bin `object` in the file to an actual object type.
    ///
    /// This function is used to take the stub object that identifies an object
    /// in the .bin file, and fleshes it out to an actual type so that you can
    /// use the data members.
    pub fn resolve_object(&self, object: &BinObject) -> Result<ShrekSuperSlamObject, Error> {
        match object.hash {
            0xF2CFE08D => Ok(ShrekSuperSlamObject::AttackMoveRegion(
                self.get_object_from_offset::<AttackMoveRegion>(object.offset)?,
            )),
            0xEBF07BB5 => Ok(ShrekSuperSlamObject::AttackMoveType(
                self.get_object_from_offset::<AttackMoveType>(object.offset)?,
            )),
            0xD306D805 => Ok(ShrekSuperSlamObject::BehaviorFightingControlShrek(
                self.get_object_from_offset::<BehaviorFightingControlShrek>(object.offset)?,
            )),
            0xC43D420D => Ok(ShrekSuperSlamObject::EffectStringReference(
                self.get_object_from_offset::<EffectStringReference>(object.offset)?,
            )),
            0xD24634FE => Ok(ShrekSuperSlamObject::EventSequence(
                self.get_object_from_offset::<EventSequence>(object.offset)?,
            )),
            0xB974E53B => Ok(ShrekSuperSlamObject::GameWorld(
                self.get_object_from_offset::<GameWorld>(object.offset)?,
            )),
            0x9B3DDBED => Ok(ShrekSuperSlamObject::GfDb(
                self.get_object_from_offset::<GfDb>(object.offset)?,
            )),
            0xCD47AA2B => Ok(ShrekSuperSlamObject::ItemSpawner(
                self.get_object_from_offset::<ItemSpawner>(object.offset)?,
            )),
            0xBFC7788D => Ok(ShrekSuperSlamObject::LocalizedString(
                self.get_object_from_offset::<LocalizedString>(object.offset)?,
            )),
            0xDBFB4A35 => Ok(ShrekSuperSlamObject::ObjectInitializer(
                self.get_object_from_offset::<ObjectInitializer>(object.offset)?,
            )),
            0xADDDF1EC => Ok(ShrekSuperSlamObject::PhysicsFighting(
                self.get_object_from_offset::<PhysicsFighting>(object.offset)?,
            )),
            0xF05C7BD3 => Ok(ShrekSuperSlamObject::PotionType(
                self.get_object_from_offset::<PotionType>(object.offset)?,
            )),
            0xBE7B44BA => Ok(ShrekSuperSlamObject::PowerupType(
                self.get_object_from_offset::<PowerupType>(object.offset)?,
            )),
            0x8811292E => Ok(ShrekSuperSlamObject::ProjectileType(
                self.get_object_from_offset::<ProjectileType>(object.offset)?,
            )),
            0xA6FC81A0 => Ok(ShrekSuperSlamObject::RenderSpawn(
                self.get_object_from_offset::<RenderSpawn>(object.offset)?,
            )),
            0xA128E61A => Ok(ShrekSuperSlamObject::ScriptDb(
                self.get_object_from_offset::<ScriptDb>(object.offset)?,
            )),
            0x90D8FCD6 => Ok(ShrekSuperSlamObject::Spitter(
                self.get_object_from_offset::<Spitter>(object.offset)?,
            )),
            0x84AD7E70 => Ok(ShrekSuperSlamObject::SpitterKeyframe(
                self.get_object_from_offset::<SpitterKeyframe>(object.offset)?,
            )),
            0xFE392AB6 => Ok(ShrekSuperSlamObject::WeaponType(
                self.get_object_from_offset::<WeaponType>(object.offset)?,
            )),
            _ => Err(Error::NotImplementedError(object.name.to_string())),
        }
    }

    /// Returns the raw bytes of the .bin file.
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    /// Returns a list of objects within the .bin file.
    pub fn objects(&self) -> &Vec<BinObject> {
        &self.objects
    }
}
