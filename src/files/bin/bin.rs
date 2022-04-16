use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};

use crate::classes;
use crate::classes::*;
use crate::console::Console;
use crate::errors::Error;
use crate::files::bin::{
    dependency::BinDependency,
    header::BinHeader,
    offset4type::BinOffset4Struct,
    section::BinSection,
};

use std::collections::HashMap;

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
            &raw[BinHeader::size() + offset as usize
                ..BinHeader::size() + offset as usize + 0x04],
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
    sections: Vec<BinSection>,
    dependencies: Vec<BinDependency>,
    offset4objs: Vec<BinOffset4Struct>,
    pub(crate) console: Console,
    pub(crate) raw: Vec<u8>,
}

impl Bin {
    /// Returns the length of the .bin file header.
    pub(crate) const fn header_length() -> usize {
        BinHeader::size()
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
        let header = BinHeader::new(&raw[0x00..BinHeader::size()], console)?;

        // The offsets and counts within the header are used to calculate
        // various offsets to the different sections within the .bin file.
        let file_begin_offset = BinHeader::size();
        let section_begin_offset = file_begin_offset + header.gf_db_size as usize;
        let dependencies_begin_offset = section_begin_offset + (header.sections as usize * BinSection::size());
        let ptr4_begin_offset = dependencies_begin_offset + (header.dependencies as usize * BinDependency::size());
        let mut sections_data_begin_offset = ptr4_begin_offset + (header.offset4_count as usize * BinOffset4Struct::size());

        // Read in each section, then the offset in each needs to be set.
        let mut sections = BinSection::new(&raw[section_begin_offset..dependencies_begin_offset], console)?;
        for section in &mut sections {
            section.offset = sections_data_begin_offset as u32;
            sections_data_begin_offset += section.count as usize * 4;
        }

        // Create an object for each serialised game object in the .bin
        let mut objects: Vec<BinObject> = vec![];
        for section in &sections {
            // The 'section' with a value of 1 in its first field details the
            // number of objects within the .bin file
            if section.number == 1 {
                // This region contains a list of offsets within the file to
                // each object contained within it
                for j in 0..section.count {
                    let object_ptr_offset = (section.offset + (j * 0x04)) as usize;
                    let object_offset =
                        console.read_u32(&raw[object_ptr_offset..(object_ptr_offset + 0x04)])?;
                    let obj = BinObject::new(&raw, object_offset, console)?;
                    objects.push(obj);
                }
            }
        }

        Ok(Bin {
            sections,
            dependencies: Vec::new(),
            offset4objs: Vec::new(),
            console,
            raw,
        })
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
        Ok(db.objects)
    }

    /// Get the bytes representation for this .bin file.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // The first section, immediately following the header, is the initial
        // gf::DB object. This contains both the header information for the
        // gf::DB object itself, and the name-offset directory that make up the
        // entries within the DB.
        //
        // We need to get this prior to making the header since we need to know
        // how big it is.
        let my_fake_db_bin_as_bytes: Vec<u8> = vec![0xab; 0x10];

        /*
         * offset1 = total size of gf::DB field, when 0x40 is added becomes offset to sections
         * sections = number of sections
         * offset2 = ????
         * dependencies = number of dependencies
         * offset4 = number of entries for offset4 struct
         */
        // With the gf::DB converted to bytes, we can construct the header of
        // the file and convert it to bytes, which is the start of the file.
        // The header contains counts of structure in each section.
        let mut bytes = BinHeader {
            gf_db_size: my_fake_db_bin_as_bytes.len() as u32,
            sections: self.sections.len() as u32,
            unknown: 0x00,
            dependencies: self.dependencies.len() as u32,
            offset4_count: self.offset4objs.len() as u32,
        }.to_bytes(self.console)?;

        // Put the bytes of the gf::DB object immediately after the header.
        bytes.extend(my_fake_db_bin_as_bytes);

        // Write the second section, which contains entries that point to other
        // places within the file.
        for section in &self.sections {
            bytes.extend(section.to_bytes(self.console)?);
        }

        // Write the third section, which contains an entry for each dependency
        // this .bin file has on another .bin file.

        // Write the fourth section, which contains objects that do ????

        // After the fourth section, there is the lists of pointers referenced
        // by the thin objects in the second section.

        // Finally, write each object within the .bin.

        Ok(bytes)
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
    pub(crate) fn get_object_from_offset<T>(&self, offset: u32) -> Result<T, Error>
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
        let object_begin = offset as usize + BinHeader::size();
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
    pub(crate) fn get_str_from_offset(&self, offset: u32) -> Result<String, Error> {
        let str_begin = offset as usize + BinHeader::size();

        // Find the first NULL byte, which ends the string. If not found,
        // default to the end of the slice, which will more than likely give us
        // an Error later
        let slice = &self.raw[str_begin..];
        let size = slice.iter().position(|&b| b == 0x00).unwrap_or(0);

        // Text within the game is stored using the single-byte ISO 8859-1
        // encoding. Specifically, $AE = ®. We therefore need to decode it
        Ok(ISO_8859_1.decode(&self.raw[str_begin..str_begin + size], DecoderTrap::Strict)?)
    }

    /// Resolve a stub .bin `object` in the file to an actual object type.
    ///
    /// This function is used to take the stub object that identifies an object
    /// in the .bin file, and fleshes it out to an actual type so that you can
    /// use the data members.
    pub(crate) fn resolve_object(&self, object: &BinObject) -> Result<ShrekSuperSlamObject, Error> {
        match object.hash {
            0xF2CFE08D => Ok(ShrekSuperSlamObject::AttackMoveRegion(
                self.get_object_from_offset::<AttackMoveRegion>(object.offset)?,
            )),
            0xEBF07BB5 => Ok(ShrekSuperSlamObject::AttackMoveType(
                self.get_object_from_offset::<AttackMoveType>(object.offset)?,
            )),
            0xE2AD9980 => Ok(ShrekSuperSlamObject::BehaviorAiFighting(
                self.get_object_from_offset::<BehaviorAiFighting>(object.offset)?,
            )),
            0xD306D805 => Ok(ShrekSuperSlamObject::BehaviorFightingControlShrek(
                self.get_object_from_offset::<BehaviorFightingControlShrek>(object.offset)?,
            )),
            0xC8E0C03F => Ok(ShrekSuperSlamObject::DynamicThrowable(
                self.get_object_from_offset::<DynamicThrowable>(object.offset)?,
            )),
            0xC43D420D => Ok(ShrekSuperSlamObject::EffectStringReference(
                self.get_object_from_offset::<EffectStringReference>(object.offset)?,
            )),
            0xDDEC024E => Ok(ShrekSuperSlamObject::Entity(
                self.get_object_from_offset::<Entity>(object.offset)?,
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
            0xF32EBBA9 => Ok(ShrekSuperSlamObject::LoadingScreen(
                self.get_object_from_offset::<LoadingScreen>(object.offset)?,
            )),
            0xBFC7788D => Ok(ShrekSuperSlamObject::LocalizedString(
                self.get_object_from_offset::<LocalizedString>(object.offset)?,
            )),
            0xD9BB3F0F => Ok(ShrekSuperSlamObject::LookAtData(
                self.get_object_from_offset::<LookAtData>(object.offset)?,
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
}
