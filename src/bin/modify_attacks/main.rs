use std::env;
use std::path::Path;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::attacks::AttackMoveType;
use shrek_superslam::files::Bin;
use shrek_superslam::{MasterDat, MasterDir};

mod args;
use args::Config;

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
    let mut master_dat = match MasterDat::from_file(&config.master_dat_path, master_dir) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dat_path, e),
    };

    for filepath in master_dat.files() {
        let trimmed = filepath.trim_end_matches(char::from(0));
        if trimmed == "data\\players\\shrek\\player.db.bin" {
            // Open the Shrek character .bin file
            let mut bin = Bin::new(
                master_dat.decompressed_file(&filepath).unwrap(),
                config.console,
            );

            // Get the Game::AttackMoveType objects from the .bin file
            let attacks: Vec<(u32, AttackMoveType)> = bin
                .objects()
                .iter()
                .filter(|o| o.name == "Game::AttackMoveType")
                .map(|o| {
                    (
                        o.offset,
                        bin.get_object_from_offset::<AttackMoveType>(o.offset)
                            .unwrap(),
                    )
                })
                .collect();

            for (offset, mut attack) in attacks {
                // Find the 'wall_atk' attack
                if attack.name == "wall_atk" {
                    // Modify the stats of the attack
                    attack.damage1 = 100.0;
                    attack.damage2 = 100.0;
                    attack.damage3 = 100.0;

                    // Write back the new attack to the .bin file
                    bin.overwrite_object(offset, &attack)
                        .unwrap_or_else(|_| panic!("failed to write object to offset {}", offset));
                }
            }

            // Write the updated .bin file to the MASTER.DAT
            if let Err(i) = master_dat.update_file(&filepath, bin.raw()) {
                panic!("updated file had wrong size: {} instead of {}", bin.raw().len(), i);
            }
        }
    }

    // Write the updated MASTER.DAT and MASTER.DIR to a new file
    master_dat.write(Path::new("MASTER.DAT"), Path::new("MASTER.DIR"))
        .expect("could not write out new files");
}
