use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use itertools::Itertools;

use crate::console::Console;
use crate::errors::Error;
use crate::hash::hash;

/// The different types of entry within a texpack
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum TexpackEntryType {
    /// An actual texture file - DDS on PC, GCT on Gamecube
    Texture,

    /// Plain text file that contain lists of texture filenames, used to
    /// describe looping animations
    Tga,
}

/// Structure defining the header of a texpack
struct TexpackHeader {
    /// The number of entries within the texpack
    pub entries: u32,

    /// The console version the data comes from
    console: Console,
}

impl TexpackHeader {
    /// Construct a new Texpack header for the `console` version of the game,
    /// with a manually-set number of `entries`.
    fn new(entries: u32, console: Console) -> TexpackHeader {
        TexpackHeader { entries, console }
    }

    /// Returns the size in bytes of a texpack header
    const fn size() -> usize {
        0x10
    }

    /// Construct a new TexpackHeader from the passed `raw` bytes of a file
    /// from the `console` version of the game.
    fn from_bytes(raw: &[u8], console: Console) -> Result<TexpackHeader, Error> {
        Ok(TexpackHeader {
            entries: console.read_u32(&raw[0x08..0x0C])?,
            console,
        })
    }

    // Construct the bytes for the texpack header
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // Start with the constant 'KPXT' magic bytes. This constant is backwards
        // for Gamecube texpacks.
        let mut header_bytes = match self.console {
            Console::Gamecube => vec![b'T', b'X', b'P', b'K'],
            _ => vec![b'K', b'P', b'X', b'T'],
        };

        // Add the fields
        header_bytes.extend(self.console.write_u32(1)?);
        header_bytes.extend(self.console.write_u32(self.entries)?);
        header_bytes.extend(self.console.write_u32(0)?);

        Ok(header_bytes)
    }
}

/// Structure representing an entry in a texpack file, which describes one of
/// the files contained within the texpack
struct TexpackEntry {
    /// Hash on the name
    pub hash: u32,

    /// The filename of the entry
    pub filename: String,

    /// The offset of the file this entry refers to within the same texpack
    pub offset: u32,

    /// The size of the file in bytes
    pub size: u32,

    /// The type of file this entry refers to
    pub filetype: TexpackEntryType,

    /// The console version this data comes from
    console: Console,
}

impl TexpackEntry {
    /// Create a new TexpackEntry structure with the fields manually set.
    fn new(
        filename: String,
        offset: u32,
        size: u32,
        filetype: TexpackEntryType,
        console: Console,
    ) -> TexpackEntry {
        TexpackEntry {
            // Due to long filenames being truncated, we cannot accurately recreate
            // the hashes of names that have been truncated.. We therefore have to
            // special case these names, thankfully there are not too many.
            hash: match filename.as_str() {
                "levelrender_fairytalevillag" => hash(&"levelrender_fairytalevillage"),
                "levelrender_gepettosworksho" => hash(&"levelrender_gepettosworkshop"),
                "levelrender_gingerbreadhous" => hash(&"levelrender_gingerbreadhouse"),
                "loadingscreen_gingerbreadho" => hash(&"loadingscreen_gingerbreadhouse"),
                _ => hash(&filename),
            },
            filename,
            offset,
            size,
            filetype,
            console,
        }
    }

    /// Construct a new TexpackEntry structure from the given `bytes` from a
    /// texpack file from the given `console`.
    fn from_bytes(raw: &[u8], console: Console) -> Result<TexpackEntry, Error> {
        let hash = console.read_u32(&raw[0x00..0x04])?;
        let filename = ISO_8859_1
            .decode(&raw[0x04..0x20].to_vec(), DecoderTrap::Strict)?
            .trim_end_matches(char::from(0))
            .to_owned();
        let offset = console.read_u32(&raw[0x20..0x24])?;
        let size = console.read_u32(&raw[0x24..0x28])?;
        let filetype = match console.read_u32(&raw[0x28..0x2C])? {
            0x00 => TexpackEntryType::Texture,
            0x02 => TexpackEntryType::Tga,
            _ => panic!("uh oh!"),
        };

        Ok(TexpackEntry {
            hash,
            filename,
            offset,
            size,
            filetype,
            console,
        })
    }

    /// Returns the size of a single entry in the .texpack file, in bytes
    const fn size() -> usize {
        0x30
    }

    /// Return the Texpack entry as raw bytes
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut entry_bytes = vec![];

        // Write in the hash of the name
        entry_bytes.extend(self.console.write_u32(self.hash)?);

        // Write in the name. It is always exactly 28 bytes - shorter
        // names are padded with zeroes, longer names are truncated.
        let mut name_bytes = self.filename.as_bytes().to_owned();
        match self.filename.len().cmp(&0x1C) {
            Ordering::Less => name_bytes.extend(vec![0x00; 0x1B - self.filename.len()]),
            Ordering::Greater => name_bytes.truncate(0x1B),
            _ => {}
        };
        entry_bytes.extend(&name_bytes);
        entry_bytes.push(0x00);

        // Write the remaining fields
        entry_bytes.extend(self.console.write_u32(self.offset)?);
        entry_bytes.extend(self.console.write_u32(self.size)?);
        match self.filetype {
            TexpackEntryType::Texture => entry_bytes.extend(self.console.write_u32(0x00)?),
            TexpackEntryType::Tga => entry_bytes.extend(self.console.write_u32(0x02)?),
        };
        entry_bytes.extend(&[0x00, 0x00, 0x00, 0x00]);

        Ok(entry_bytes)
    }
}

/// Structure for the files within a texpack
pub struct TexpackFile {
    /// The filename of the file, without the extension (which differs by platform)
    filename: String,

    /// The console the file comes from
    console: Console,

    /// The type of the file
    filetype: TexpackEntryType,

    /// The data of the file
    pub data: Vec<u8>,
}

impl TexpackFile {
    /// Construct a new TexpackFile structure.
    fn new(
        filename: String,
        filetype: TexpackEntryType,
        data: &[u8],
        console: Console,
    ) -> TexpackFile {
        TexpackFile {
            filename,
            console,
            filetype,
            data: data.to_vec(),
        }
    }

    /// Returns the full filename of the file, including extension.
    pub fn filename(&self) -> String {
        match self.filetype {
            TexpackEntryType::Texture => match self.console {
                Console::Gamecube => format!("{}.gct", self.filename),
                Console::PC => format!("{}.dds", self.filename),
                Console::PS2 => format!("{}.tm2", self.filename),
                Console::Xbox => format!("{}.dds", self.filename),
            },
            TexpackEntryType::Tga => format!("{}.tga", self.filename),
        }
    }

    /// Return the padded form of the file, for writing back to a texpack file.
    fn padded(&self) -> Vec<u8> {
        let mut padded = self.data.clone();
        padded.extend(&vec![0xEE; self.padded_size() - self.data.len()]);
        padded
    }

    /// Returns the size of the padded form of the file
    fn padded_size(&self) -> usize {
        self.data.len() + (0x1000 - (self.data.len() % 0x1000))
    }
}

/// Structure for reading and managing a .texpack file from the extracted Shrek
/// SuperSlam game files
pub struct Texpack {
    /// Mapping of the filenames within the texpack to the file data
    files: Vec<TexpackFile>,

    /// The console version of the game this file comes from
    console: Console,
}

impl Texpack {
    /// Construct an empty Texpack structure for the given `console` version.
    ///
    /// # Example
    ///
    /// ```
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Texpack;
    ///
    /// let new_texpack = Texpack::new(Console::PC);
    /// ```
    pub fn new(console: Console) -> Texpack {
        Texpack {
            files: vec![],
            console,
        }
    }

    /// Construct a new Texpack structure from the `raw` bytes of a .texpack file
    /// from the given `console` version.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::{Console, MasterDat, MasterDir};
    /// use shrek_superslam::files::Texpack;
    ///
    /// // Read the MASTER.DAT and MASTER.DIR files
    /// let master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
    /// let master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();
    ///
    /// // Read a texpack from the MASTER.DAT
    /// let texpack = Texpack::from_bytes(
    ///     &master_dat.decompressed_file("data\\spawns\\players\\shrek\\object.texpack").unwrap(),
    ///     Console::PC
    /// );
    /// ```
    pub fn from_bytes(raw: &[u8], console: Console) -> Result<Texpack, Error> {
        // Read the header
        let header = TexpackHeader::from_bytes(&raw[0x00..0x10], console)?;

        // Parse each entry from the header
        let entries: Result<Vec<TexpackEntry>, Error> = (0..header.entries as usize)
            .map(|i| {
                let begin = (i * TexpackEntry::size()) + 0x10;
                let end = begin + TexpackEntry::size();
                TexpackEntry::from_bytes(&raw[begin..end], console)
            })
            .collect();

        // Use the entries to pull out each file from the texpack
        let files = entries?
            .into_iter()
            .map(|e| {
                TexpackFile::new(
                    e.filename,
                    e.filetype,
                    &raw[e.offset as usize..(e.offset + e.size) as usize],
                    console,
                )
            })
            .collect();

        Ok(Texpack { files, console })
    }

    /// Creates a new Texpack structure from the passed file.
    ///
    /// Returns an `Ok(Texpack)` on success, or an `Err(std::io::Error)`
    /// if there is an error while reading the file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Texpack;
    ///
    /// let texpack = Texpack::from_file(Path::new("data\\spawns\\players\\shrek\\object.texpack"), Console::PC);
    /// ```
    pub fn from_file(path: &Path, console: Console) -> Result<Texpack, Error> {
        // Read all of the file to a byte array
        let file_contents = fs::read(&path)?;

        // Parse the bytes to a Texpack object
        Texpack::from_bytes(&file_contents, console)
    }

    /// Add a new file with the given `name` and `data`, of the given `kind` to
    /// the texpack.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Texpack;
    ///
    /// let mut texpack = Texpack::new(Console::PC);
    /// texpack.add_file("data\\test.dds".to_string(), &Vec::new());
    /// ```
    pub fn add_file(&mut self, name: String, data: &[u8]) {
        // Determine the filetype based on the console version and the header
        let header = &data[0x00..0x04];
        let filetype = if (self.console == Console::Gamecube && header == [0x47, 0x43, 0x4E, 0x54])
            || ((self.console == Console::PC || self.console == Console::Xbox)
                && header == [0x44, 0x44, 0x53, 0x20])
            || (self.console == Console::PS2 && header == [0x54, 0x49, 0x4D, 0x32])
        {
            TexpackEntryType::Texture
        } else {
            TexpackEntryType::Tga
        };

        // Add the new file to the list of files
        self.files
            .push(TexpackFile::new(name, filetype, data, self.console));
    }

    /// Returns the list of files within the texpack
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Texpack;
    ///
    /// let texpack = Texpack::from_file(Path::new("data\\spawns\\players\\shrek\\object.texpack"), Console::PC).unwrap();
    /// for file in texpack.files() {
    ///     println!("{}: {} bytes", file.filename(), file.data.len());
    /// }
    /// ```
    pub fn files(&self) -> &Vec<TexpackFile> {
        &self.files
    }

    /// Convert the Texpack object to its on-disk byte representation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shrek_superslam::Console;
    /// use shrek_superslam::files::Texpack;
    ///
    /// let mut texpack = Texpack::new(Console::PC);
    /// texpack.add_file("data\\test.dds".to_string(), &Vec::new());
    /// let texpack_bytes = texpack.to_bytes();
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut texpack_bytes = vec![];

        // Write the header
        let header = TexpackHeader::new(self.files.len() as u32, self.console);
        texpack_bytes.extend(header.to_bytes()?);

        // Calculations to determine where the offset of each texture file
        // will begin within the texpack, needed for each metadata entry.
        let entries_start = TexpackHeader::size();
        let entries_end = entries_start + (self.files.len() * TexpackEntry::size());
        let header_padding_size = 0x1000 - (entries_end % 0x1000);
        let files_start = entries_end + header_padding_size;
        let mut cumulative_offset = files_start;

        // Create each entry to point to the actual files.
        //
        // The text file entries need to be after the files they reference,
        // so we sort the files by their type so that the text files all come
        // last.
        for file in self.files.iter().sorted_by(|a, b| {
            a.filetype
                .partial_cmp(&b.filetype)
                .unwrap_or(Ordering::Equal)
        }) {
            let entry = TexpackEntry::new(
                file.filename.clone(),
                cumulative_offset as u32,
                file.data.len() as u32,
                file.filetype,
                self.console,
            );
            texpack_bytes.extend(&entry.to_bytes()?);
            cumulative_offset += file.padded_size();
        }

        // Add the padding between the header and the files
        texpack_bytes.extend(&vec![0xEE; header_padding_size]);

        // Add the contents of each file
        for file in self.files.iter().sorted_by(|a, b| {
            a.filetype
                .partial_cmp(&b.filetype)
                .unwrap_or(Ordering::Equal)
        }) {
            texpack_bytes.extend(&file.padded());
        }

        Ok(texpack_bytes)
    }
}
