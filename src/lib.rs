//! Library for parsing and interacting with the game files from the 2005
//! videogame [Shrek SuperSlam](https://en.wikipedia.org/wiki/Shrek_SuperSlam).
//!
//! This code supports the Gamecube, PC, PS2 and Xbox releases of the game.
//! Each of these versions contains MASTER.DAT file, which contains all of the
//! game's assets in a compressed blob, as well as an associated MASTER.DIR
//! file. With this library, this blob can be extracted and rebuilt, and the
//! contents can be examined, modified and edited. These modifications are
//! accepted by the game - which contains no integrity checking - and so this
//! library can be used for modifications to the base game through the editing
//! of its files.
//!
//! ## Example
//!
//! Here is an example of reading the MASTER.DAT file for the PC version, and
//! listing its contents. Note that the console version the files come from
//! must be manually specified.
//!
//! ```no_run
//! use std::path::Path;
//! use shrek_superslam::{Console, MasterDat, MasterDir};
//!
//! let master_dir_path = Path::new("C:\\Program Files (x86)\\Activision\\Shrek SuperSlam\\MASTER.DIR");
//! let master_dat_path = Path::new("C:\\Program Files (x86)\\Activision\\Shrek SuperSlam\\MASTER.DAT");
//! let master_dir = MasterDir::from_file(master_dir_path, Console::PC).unwrap();
//! let master_dat = MasterDat::from_file(master_dat_path, master_dir).unwrap();
//! for filename in &master_dat.files() {
//!     println!("{}", filename);
//! }
//! ```
//!
//! ## General game anatomy
//!
//! Generally, the game contains the following file types:
//!
//! * **.db.bin files**. These have the majority of the 'game data' within.
//!   They are collections of serialised C++ classes that make up details such
//!   as the character data (e.g. hitboxes, attacks, etc.), stages, modes, and
//!   so on. Within this library, the [`Bin`](crate::files::Bin) structure can
//!   be used to parse these and extract these serialised objects to types
//!   found in the [`classes`](crate::classes) crate. Currently, however, most
//!   class types are unsupported.
//! * **havok.xml files**. These define the level layouts for the Havok
//!   collision engine the game uses. They are not supported by this library,
//!   however they are in plaintext XML and can be modified externally.
//! * **Mesh files**. These contain the 3D model data for the game. They differ
//!   by platform, but are currently unsupported by the library.
//! * **song.bin files**. It is unknown what these do. They are unsupported by
//!   the library.
//! * **Texture files**. These contain the various textures used throughout the
//!   game. The exact type of the texture file depends on the platform being
//!   used. The PC release contains [DirectDraw Surface](https://en.wikipedia.org/wiki/DirectDraw_Surface)
//!   (.dds) files. The Gamecube release used .gct files, use a custom container
//!   format to house the CMPR format common to many Gamecube and Wii games.
//!   This library does not currently support any of these texture types,
//!   though external tooling exists online.
//! * **Texpack files**. These use the extension .texpack. They are a container
//!   format to house multiple texture files. These can be opened and
//!   manipulated with the [`Texpack`](crate::files::Texpack) structure.
//!
//! ## Example programs
//!
//! This crate also contains multiple example programs to assist in working
//! with the games files.
//!
//! * **shreksuperslam-classes**: Extracts the name and offset of every
//!   class from every .db.bin file.
//! * **shreksuperslam-db**: Extracts every entry in a `gf::DB` object that
//!   prefaces the given .db.bin file.
//! * **shreksuperslam-extract**: Extracts the given MASTER.DAT / MASTER.DIR
//!   pair to the filesystem.
//! * **shreksuperslam-repackage**: Repackages an extracted `data\` folder
//!   into a MASTER.DAT and MASTER.DIR pair. This can be used in combination
//!   with the extract program to extract the files, modify them externally,
//!   and repackage them so that the game runs with the changes.

// Public interface
mod console;
pub use console::Console;

mod errors;
pub use errors::Error;

mod master_dat;
pub use master_dat::MasterDat;

mod master_dir;
pub use master_dir::MasterDir;

pub mod classes;
pub mod files;

// Internal interface
mod compression;
mod hash;
