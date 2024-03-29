use std::env;
use std::fs;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::GfDb;
use shrek_superslam::files::Bin;

mod args;
use args::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the .bin file and extract the gf::DB object
    let bin = Bin::new(fs::read(&config.db_path).unwrap(), config.console).unwrap();
    let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

    // Dump the contents of the object list
    println!("offset,name,type");
    for entry in &db.entries {
        println!("{:08X},{},{}", entry.1.offset + 0x40, entry.0, entry.1.name);
    }
}
