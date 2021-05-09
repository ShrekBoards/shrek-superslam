use std::io;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::errors::Error;

/// The different console versions of the game, used to determine which
/// endianness to use when reading numbers from files
#[derive(Copy, Clone, PartialEq)]
pub enum Console {
    Gamecube,
    PC,
    PS2,
    Xbox,
}

impl Console {
    /// Read a 32-bit unsigned integer from the given bytes from the given console
    ///
    /// # Parameters
    ///
    /// - `bytes`: The bytes to read as an integer
    ///
    /// # Returns
    ///
    /// The 32-bit unsigned integer from the bytes
    pub fn read_u32(&self, bytes: &[u8]) -> Result<u32, Error> {
        if bytes.len() < 4 {
            return Err(Error::ConsoleNumberError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Too few bytes to read",
            )));
        }

        match self {
            Console::Gamecube => e((&bytes[0..4]).read_u32::<BigEndian>()),
            _ => e((&bytes[0..4]).read_u32::<LittleEndian>()),
        }
    }

    /// Given a 32-bit unsigned integer, returns the console's representation
    /// as an array of bytes
    ///
    /// # Parameters
    ///
    /// - `n`: The number to convert
    ///
    /// # Returns
    ///
    /// The 32-bit integer as an array of 4 bytes, as the console represents
    /// the value
    pub fn write_u32(&self, n: u32) -> Result<Vec<u8>, Error> {
        let mut wtr = Vec::new();
        match self {
            Console::Gamecube => e(wtr.write_u32::<BigEndian>(n))?,
            _ => e(wtr.write_u32::<LittleEndian>(n))?,
        };

        Ok(wtr)
    }

    /// Read a 32-bit floating-point from the given bytes from the given console
    ///
    /// # Parameters
    ///
    /// - `bytes`: The bytes to read as a floating point
    ///
    /// # Returns
    ///
    /// The 32-bit floating point from the bytes
    pub fn read_f32(&self, bytes: &[u8]) -> Result<f32, Error> {
        if bytes.len() < 4 {
            return Err(Error::ConsoleNumberError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Too few bytes to read",
            )));
        }

        match self {
            Console::Gamecube => e((&bytes[0..4]).read_f32::<BigEndian>()),
            _ => e((&bytes[0..4]).read_f32::<LittleEndian>()),
        }
    }

    /// Given a 32-bit floating-point, returns the console's representation
    /// as an array of bytes
    ///
    /// # Parameters
    ///
    /// - `n`: The number to convert
    ///
    /// # Returns
    ///
    /// The 32-bit floating-point as an array of 4 bytes, as the console represents
    /// the value
    pub fn write_f32(&self, n: f32) -> Result<Vec<u8>, Error> {
        let mut wtr = Vec::new();
        match self {
            Console::Gamecube => e(wtr.write_f32::<BigEndian>(n))?,
            _ => e(wtr.write_f32::<LittleEndian>(n))?,
        };

        Ok(wtr)
    }
}

/// Converts an error from the [`byteorder`] crate to an error from the library.
fn e<T>(result: Result<T, io::Error>) -> Result<T, Error> {
    match result {
        Ok(x) => Ok(x),
        Err(x) => Err(Error::ConsoleNumberError(x)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_u32_pc() {
        let data1 = vec![0x00, 0x00, 0x00, 0x00];
        let data2 = vec![0xFF, 0xFF, 0xFF, 0xFF];
        let data3 = vec![0x01, 0x02, 0x03, 0x04];
        let too_short = vec![0x00];
        let too_long = vec![0x01, 0x02, 0x03, 0x04, 0x05];

        assert_eq!(Console::PC.read_u32(&data1[0..4]).unwrap(), 0);
        assert_eq!(Console::PC.read_u32(&data2[0..4]).unwrap(), u32::MAX);
        assert_eq!(Console::PC.read_u32(&data3[0..4]).unwrap(), 0x04030201);
        assert!(Console::PC.read_u32(&too_short).is_err());
        assert_eq!(Console::PC.read_u32(&too_long).unwrap(), 0x04030201);
    }

    #[test]
    fn read_u32_gcn() {
        let data1 = vec![0x00, 0x00, 0x00, 0x00];
        let data2 = vec![0xFF, 0xFF, 0xFF, 0xFF];
        let data3 = vec![0x01, 0x02, 0x03, 0x04];

        assert_eq!(Console::Gamecube.read_u32(&data1[0..4]).unwrap(), 0);
        assert_eq!(Console::Gamecube.read_u32(&data2[0..4]).unwrap(), u32::MAX);
        assert_eq!(
            Console::Gamecube.read_u32(&data3[0..4]).unwrap(),
            0x01020304
        );
    }

    #[test]
    fn write_u32_pc() {
        assert_eq!(
            Console::PC.write_u32(0).unwrap(),
            vec![0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Console::PC.write_u32(u32::MAX).unwrap(),
            vec![0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(
            Console::PC.write_u32(0x04030201).unwrap(),
            vec![0x01, 0x02, 0x03, 0x04]
        );
    }

    #[test]
    fn write_u32_gcn() {
        assert_eq!(
            Console::Gamecube.write_u32(0).unwrap(),
            vec![0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Console::Gamecube.write_u32(u32::MAX).unwrap(),
            vec![0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(
            Console::Gamecube.write_u32(0x04030201).unwrap(),
            vec![0x04, 0x03, 0x02, 0x01]
        );
    }

    #[test]
    fn read_f32_pc() {
        let data1 = vec![0x00, 0x00, 0x00, 0x00];
        let data2 = vec![0x00, 0x00, 0x80, 0x3F];
        let data3 = vec![0x00, 0x00, 0x80, 0xBF];

        assert_eq!(Console::PC.read_f32(&data1[0..4]).unwrap(), 0.0);
        assert_eq!(Console::PC.read_f32(&data2[0..4]).unwrap(), 1.0);
        assert_eq!(Console::PC.read_f32(&data3[0..4]).unwrap(), -1.0);
    }

    #[test]
    fn read_f32_gcn() {
        let data1 = vec![0x00, 0x00, 0x00, 0x00];
        let data2 = vec![0x3F, 0x80, 0x00, 0x00];
        let data3 = vec![0xBF, 0x80, 0x00, 0x00];

        assert_eq!(Console::Gamecube.read_f32(&data1[0..4]).unwrap(), 0.0);
        assert_eq!(Console::Gamecube.read_f32(&data2[0..4]).unwrap(), 1.0);
        assert_eq!(Console::Gamecube.read_f32(&data3[0..4]).unwrap(), -1.0);
    }

    #[test]
    fn write_f32_pc() {
        assert_eq!(
            Console::PC.write_f32(0.0).unwrap(),
            vec![0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Console::PC.write_f32(1.0).unwrap(),
            vec![0x00, 0x00, 0x80, 0x3F]
        );
        assert_eq!(
            Console::PC.write_f32(-1.0).unwrap(),
            vec![0x00, 0x00, 0x80, 0xBF]
        );
    }

    #[test]
    fn write_f32_gcn() {
        assert_eq!(
            Console::Gamecube.write_f32(0.0).unwrap(),
            vec![0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Console::Gamecube.write_f32(1.0).unwrap(),
            vec![0x3F, 0x80, 0x00, 0x00]
        );
        assert_eq!(
            Console::Gamecube.write_f32(-1.0).unwrap(),
            vec![0xBF, 0x80, 0x00, 0x00]
        );
    }
}
