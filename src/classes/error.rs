use std::error;
use std::fmt;

/// Error type for errors caused by trying to read or write serialised game
/// classes from the Shrek SuperSlam .bin files
#[derive(Debug)]
pub enum Error {
    /// Caused when requesting an object that is too large for the space in the file
    NotEnoughBytes {
        requested: usize,
        file_size: usize,
        offset: usize,
    },

    /// Caused by requesting an object that does not match the type of object
    /// at the given offset
    IncorrectType { hash: u32 },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotEnoughBytes {
                requested,
                file_size,
                offset,
            } => write!(
                f,
                "File size of {} is not large enough for object of length {} at offset {}",
                file_size, requested, offset
            ),
            Error::IncorrectType { hash } => write!(
                f,
                "Incorrect hash at offset - hash was instead 0x{:04X}",
                hash
            ),
        }
    }
}
