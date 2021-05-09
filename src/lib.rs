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
