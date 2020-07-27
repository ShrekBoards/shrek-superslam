use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

/// The different console versions of the game, used to determine which
/// endianness to use when reading numbers from files
#[derive(Copy, Clone)]
pub enum Console {
    Gamecube,
    PC,
    PS2,
    Xbox,
}

impl Console {
    /// Read a 32-bit unsigned integer from the given bytes from the given console
    ///
    /// \param bytes   The bytes to read as an integer
    ///
    /// \returns The 32-bit unsigned integer from the bytes
    pub fn read32(&self, bytes : &[u8]) -> u32 {
        return match self {
            Console::Gamecube => (&bytes[0..4]).read_u32::<BigEndian>().unwrap(),
            _ => (&bytes[0..4]).read_u32::<LittleEndian>().unwrap()
        }
    }
}