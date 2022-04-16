///! Module for parsing and managing .bin files.
///!
///! This module is split across multiple implementing files as the .bin file
///! type is quite complex. The [`bin::Bin`] object is the public interface,
///! which provides methods for reading the file, accessing the objects,
///! writing new objects over the existing ones, and reconstructing the file
///! to bytes so it can be written back to the MASTER.DAT.
///!
///! ## .bin File Type Documentation
///!
///! There a multiple sections to the .bin file, each with its own implementing
///! file in this module.
///!
///! The main top-level function for reading all this information out in the
///! game itself resides at `loc_410E60` in the PC version. Like with most
///! files in this game, it is important to remember that many pointer fields
///! are represented as offsets at rest, and at runtime written back to with
///! the loaded address to instead form a pointer type, that is then read.
///!
///! Similarly, all offsets in .bin files are calculated from the start of the
///! file plus the header, so an offset in the file 0x00 in reality is
///! referring to +0x40.
///!
///! The .bin file has a header, and is then split into four sections.
///!
///! * Top-level DB - a `gf::DB` object that is used as a name-pointer mapping
///!   to each other object within the .bin file. The game uses this to find
///!   types it needs. Each object pointer to is basically a serialised C++
///!   object that is loaded in at runtime and again has pointers patched. The
///!   'classes' module of this crate deals with loading these.
///! * 'Sections' (better name pending) - small entries that again point
///!   elsewhere in the file.
///! * Dependencies - a list of dependencies this .bin file has on others. The
///!   game recursively loads these at runtime with the function at
///!   `loc_4111D0`.
///! * 'Offset4Types' (better name pending) - unknown what this does.
///!
///! Then the remainder of the file is the objects pointed to by the DB.
///!
///! ### Header
///!
///! The header makes up the first 64 (0x40) bytes. It is used to locate each
///! of the four other sections within the file. It is implemented in with the
///! [`header::BinHeader`] type in this module.
///!
///! | Name             | Offset | Size in bytes | Notes                                                                                          |
///! |------------------|--------|---------------|------------------------------------------------------------------------------------------------|
///! | Blank            | +0x00  | 16            | 0x00 16 times                                                                                  |
///! | gf_db_size       | +0x10  | 4             | Size in bytes of the top-level DB (a gf::DB object), both the header and the database entries. |
///! | gf_db_ptr        | +0x14  | 4             | Pointer to top-level DB (a gf::DB object), calculated at runtime (always base + 0x40).         |
///! | sections         | +0x18  | 4             | Number of entries in second section.                                                           |
///! | unknown          | +0x1C  | 4             | Doesn't seem to be used?                                                                       |
///! | sections_ptr     | +0x20  | 4             | Pointer to second section, calculated at runtime (gf_db_ptr + gf_db_size).                     |
///! | dependencies     | +0x24  | 4             | Number of entries in the third section, the number of dependencies.                            |
///! | dependencies_ptr | +0x28  | 4             | Pointer to the dependencies, calculated at runtime (sections_ptr + (sections * 0x10))          |
///! | offset4_count    | +0x2C  | 4             | Number of entries in the fourth section.                                                       |
///! | offset4_ptr      | +0x30  | 4             | Pointer to the 'offset4' type objects (dependencies_ptr + (dependencies * 0x80)).              |
///! | processed        | +0x34  | 4             | Count of the number of times the .bin has been loaded? Zero at rest.                           |
///! | Blank            | +0x38  | 8             | 0x00 8 times                                                                                   |
///!
///! ### Top-level DB
///!
///! ### 'Sections'
///!
///! Each 'section' has a type, an offset to later in the file, and a number of
///! entries found at the offset. They are a simple type, and implemented in
///! module with the [`section::BinSection`] structure.
///!
///! | Name   | Offset | Size in bytes | Notes                                                         |
///! |--------|--------|---------------|---------------------------------------------------------------|
///! | number | +0x00  | 4             | Number identifying the type this entry points to.             |
///! | count  | +0x04  | 4             | The number of pointers in the section.                        |
///! | ptr    | +0x08  | 4             | The offset the collection of pointers begins within the file. |
///! | unused | +0x0C  | 4             |                                                               |
///!
///! ### Dependencies
///!
///! ### 'Offset4Type'
///!

mod bin;
mod dependency;
mod header;
mod offset4type;
mod section;

pub use bin::{Bin, BinObject};