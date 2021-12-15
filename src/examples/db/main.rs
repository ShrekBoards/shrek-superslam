use std::env;
use std::fs;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::{GfDb, ScriptDb};
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

    let parsed = bin.parse().unwrap();
    for (name, _) in parsed {
        println!("{}", name);
    }

    /*
    let db = bin.get_object_from_offset::<GfDb>(0x00).unwrap();

    // Dump the contents of the object list
    println!("name,type");
    for entry in &db.entries {
        println!("{},{}", entry.0, entry.1.name);
        if entry.1.name == "GF_TEMP::ScriptDB" {
            let script_db = bin.get_object_from_offset::<ScriptDb>(entry.1.offset).unwrap();
            for s_entry in &script_db.entries {
                println!("\t{},{}", s_entry.0, s_entry.1.name);
            }
        }
    }
    */
}
