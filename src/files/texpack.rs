use std::fs;
use std::io;
use std::path::Path;

use crate::console::Console;
use crate::errors::Error;

/// The different types of entry within a texpack
pub enum TexpackEntryType {
    /// An actual texture file - DDS on PC, GCT on Gamecube
    Texture,

    /// Plain text file that contain lists of texture filenames, used to
    /// describe looping animations
    Tga,
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
}

impl TexpackEntry {
    /// Create a new TexpackEntry structure
    ///
    /// # Parameters
    ///
    /// - `raw`: The raw bytes of the entry descriptor
    /// - `console`: The console version the .texpack file is from
    ///
    /// # Returns
    ///
    /// The constructed texpack entry
    fn new(raw: &[u8], console: Console) -> Result<TexpackEntry, Error> {
        let hash = console.read_u32(&raw[0x00..0x04])?;
        let filename = String::from_utf8(raw[0x04..0x20].to_vec())
            .unwrap()
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
        })
    }

    /// # Returns
    ///
    /// The size of a single entry in the .texpack file, in bytes
    const fn size() -> usize {
        0x30
    }
}

/// Structure defining the header of a texpack
struct TexpackHeader {
    /// The number of entries within the texpack
    pub entries: u32,
}

impl TexpackHeader {
    /// Construct a new TexpackHeader structure
    ///
    /// # Parameters
    ///
    /// - `raw`: The raw bytes of the .texpack file header
    /// - `console`: The console version the .texpack came from
    fn new(raw: &[u8], console: Console) -> Result<TexpackHeader, Error> {
        Ok(TexpackHeader {
            entries: console.read_u32(&raw[0x08..0x0C])?,
        })
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
    /// Construct a TexpackFile structure
    ///
    /// # Parameters
    ///
    /// - `filename`: The filename of the file, as it comes from the texpack entry
    /// - `filetype`: The type of the file
    /// - `data`: The binary data of the file
    /// - `console`: The console version the texpack is from
    ///
    /// # Returns
    ///
    /// The constructed TexpackFile structure
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

    /// # Returns
    ///
    /// The full filename of the file, including extension
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
    /// Construct an empty Texpack structure
    ///
    /// # Parameters
    ///
    /// - `console`: The console version this texpack is for
    ///
    /// # Returns
    ///
    /// The constructed, empty texpack
    pub fn new(console: Console) -> Texpack {
        Texpack {
            files: vec![],
            console,
        }
    }

    /// Construct a new Texpack structure from the bytes of a .texpack file
    ///
    /// # Parameters
    ///
    /// - `raw`: The full bytes of the .texpack file
    /// - `console`: The console version the .texpack is from
    ///
    /// # Returns
    ///
    /// The constructed texpack from the passed bytes
    pub fn from_bytes(raw: &[u8], console: Console) -> Result<Texpack, Error> {
        // Read the header
        let header = TexpackHeader::new(&raw[0x00..0x10], console)?;

        // Parse each entry from the header
        let entries: Vec<TexpackEntry> = (0..header.entries as usize)
            .map(|i| {
                let begin = (i * TexpackEntry::size()) + 0x10;
                let end = begin + TexpackEntry::size();
                TexpackEntry::new(&raw[begin..end], console)
            })
            .collect();

        // Use the entries to pull out each file from the texpack
        let files = entries
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

    /// Creates a new Texpack structure from the passed file
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the .texpack file to read
    /// - `console`: The console version this file is from
    ///
    /// # Returns
    ///
    /// An `Ok(Texpack)` on success, or an `Err(std::io::Error)` if there is an
    /// error while reading the file
    pub fn from_file(path: &Path, console: Console) -> Result<Texpack, Error> {
        // Read all of the file to a byte array
        let file_contents = fs::read(&path)?;

        // Parse the bytes to a Texpack object
        Ok(Texpack::from_bytes(&file_contents, console)?)
    }

    /// Add a file to the Texpack
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the file (no extension)
    /// - `data`: The file data
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

    /// # Returns
    ///
    /// The list of files within the texpack
    pub fn files(&self) -> &Vec<TexpackFile> {
        &self.files
    }
}
