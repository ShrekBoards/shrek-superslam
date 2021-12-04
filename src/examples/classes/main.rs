use std::env;
use std::path::Path;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::*;
use shrek_superslam::files::{Bin, BinObject};
use shrek_superslam::{MasterDat, MasterDir};

mod args;
use args::Config;

/// Function for printing additional info on classes of types we can decode.
fn print_class_additional_info(bin: &Bin, object: &BinObject) {
    match object.name {
        "gf::LocalizedString" => {
            let localized_string = bin
                .get_object_from_offset::<LocalizedString>(object.offset)
                .unwrap();
            print!(" ({})", localized_string.string);
        }

        "Game::EffectStringReference" => {
            let localized_string = bin
                .get_object_from_offset::<EffectStringReference>(object.offset)
                .unwrap();
            print!(" ({})", localized_string.string);
        }

        "Game::AttackMoveType" => {
            let attack = bin
                .get_object_from_offset::<AttackMoveType>(object.offset)
                .unwrap();
            print!(" ({})", attack.name);
        }

        _ => {}
    };
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the MASTER.DIR and MASTER.DAT files
    let master_dir = match MasterDir::from_file(&config.master_dir_path, config.console) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dir_path, e),
    };
    let master_dat = match MasterDat::from_file(&config.master_dat_path, master_dir) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dat_path, e),
    };

    // Iterate over all objects in all .bin files, and print a sequential list
    // of the objects within each file and the offset of each of those objects
    for filepath in master_dat.files() {
        if let Some(extension) = Path::new(&filepath).extension() {
            if extension == "bin" {
                let bin = Bin::new(
                    master_dat.decompressed_file(&filepath).unwrap(),
                    config.console,
                )
                .unwrap_or_else(|e| panic!("Error reading '{}': {:?}", &filepath, e));

                if !bin.objects().is_empty() {
                    println!("{} ({} objects)", filepath, bin.objects().len());
                    for object in bin.objects() {
                        print!("\t+{:04x}: {}", object.offset + 0x40, object.name);
                        print_class_additional_info(&bin, &object);
                        println!();
                    }
                }
            }
        }
    }
}
