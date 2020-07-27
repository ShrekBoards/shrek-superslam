use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

use crate::console::Console;

/// Read a 32-bit unsigned integer from the given bytes from the given console
///
/// \param bytes   The bytes to read as an integer
/// \param console The console platform the bytes have come from
///
/// \returns The 32-bit unsigned integer from the bytes
pub fn read32(bytes : &[u8], console : Console) -> u32 {
    return match console {
        Console::Gamecube => (&bytes[0..4]).read_u32::<BigEndian>().unwrap(),
        _ => (&bytes[0..4]).read_u32::<LittleEndian>().unwrap()
    }
}