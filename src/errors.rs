use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;

use crate::classes;

/// Top-level error type for all errors generated by the library.
#[derive(Debug)]
pub enum Error {
    /// An error generated when trying to deserialise a class from a .bin file.
    ClassDeserialiseError(classes::Error),

    /// An error generated when trying to read or write a number value for the
    /// given console. Contains the error generated by the `byteorder` crate.
    ConsoleNumberError(io::Error),

    /// An error generated if there is an error reading or writing to a file.
    FileError(io::Error),

    /// An error generated when a string cannot be decoded from an in-game file.
    /// Contains the error generated by the `ISO_8859_1` crate.
    StringDeserialiseError(Cow<'static, str>),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yo")
    }
}