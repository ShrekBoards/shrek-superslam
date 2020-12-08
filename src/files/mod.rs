//! Module containing parsers for the various file types within the decompressed
//! Shrek SuperSlam game files.
mod bin;
mod texpack;
pub use bin::Bin;
pub use texpack::{Texpack, TexpackEntryType};
