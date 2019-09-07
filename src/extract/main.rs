use std::env;
use std::fs;
use std::process;

mod args;
mod dir;

use args::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        process::exit(1);
    });

    let master_dir = fs::read(&config.master_dir_path)
       .expect("unable to read master.dir");

    let dir = dir::MasterDir::new(master_dir, &config.console);
    for d in dir.entries {
        println!("{} (0x{:x}, 0x{:x}, 0x{:x})",
            d.name,
            d.offset,
            d.malloc_size,
            d.orig_size
        );
    }
}
