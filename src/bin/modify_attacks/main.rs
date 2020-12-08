use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::attacks::AttackMoveType;
use shrek_superslam::files::Bin;
use shrek_superslam::Console;
use shrek_superslam::{MasterDat, MasterDir};

mod args;
use args::{Config, Mode};

/// Get all character Game::AttackMoveType objects and put them in a JSON
/// structure, then write that JSON to a new file
///
/// The resultant JSON looks like:
///
/// ```
/// {
///     "shrek": [
///         {
///             "damage1": 4.0,
///             "damage2": 4.0,
///             "damage3": 4.0,
///             "disabled": false,
///             "fall_speed": 0.0,
///             "hitboxes": [
///                 {
///                     "delay": 0.16666667,
///                     "width": -1.0,
///                     "radius": 2.0,
///                     "offset": 7712
///                 }
///             ],
///             "intangible": false,
///             "knocks_down": false,
///             "name": "fast3_atk",
///             "pushes_back": false
///         },
///     ]
/// }
/// ```
///
/// # Parameters
///
/// - `master_dat`: The game's MASTER.DAT file
/// - `console`: The console the MASTER.DAT comes from
/// - `json_path`: The path to write the JSON representation to
fn attacks_to_json(master_dat: &MasterDat, console: Console, json_path: &Path) {
    // A BTreeMap is used so that the output values are sorted by key
    let mut attacks = BTreeMap::<String, Vec<AttackMoveType>>::new();

    // Enumerate all files to find the player.db.bin files
    for filepath in master_dat.files() {
        // Get the filename of
        let mut iter = filepath.rsplit('\\').take(2);
        let filename = iter.next().unwrap();

        if filename == "player.db.bin" {
            // Get the character name from the directory containing the file
            let character = iter.next().unwrap();

            // Read the player.db.bin file, grab all the Game::AttackMoveType
            // objects and convert them to JSON objects
            let bin = Bin::new(master_dat.decompressed_file(&filepath).unwrap(), console);
            let objects = bin.get_all_objects_of_type::<AttackMoveType>().into_iter().map(|(_, a)| a).collect();

            attacks.insert(character.to_owned(), objects);
        }
    }

    // Write the object to a JSON file
    let file = File::create(&json_path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &attacks).unwrap();
}

/// Writes the character attack values from the given JSON file to the
/// MASTER.DAT, then writes out a new MASTER.DAT and MASTER.DIR pair containing
/// the changes
///
/// # Parameters
///
/// - `master_dat`: The game's MASTER.DAT file
/// - `console`: The console the MASTER.DAT comes from
/// - `json_path`: The path to read the updated attack values from
fn write_new_attack_data(master_dat: &mut MasterDat, console: Console, json_path: &Path) {
    // Load and deserialise the JSON file
    let file = File::open(&json_path).unwrap();
    let reader = BufReader::new(file);

    let attacks: HashMap<String, Vec<AttackMoveType>> = serde_json::from_reader(reader).unwrap();
    for (character, attacks) in &attacks {
        // Read the player.db.bin file for this character
        let filename = format!("data\\players\\{}\\player.db.bin", character);
        let mut bin = Bin::new(master_dat.decompressed_file(&filename).unwrap(), console);

        // Collect every Game::AttackMoveType object in the player.db.bin file,
        // along with the attack's offset within the file
        let original_attacks = bin.get_all_objects_of_type::<AttackMoveType>();

        // Iterate over each of these attacks, find the matching attack in the
        // updated JSON file, then overwrite the attack in the player.db.bin
        // file with the updated one from the JSON
        for (offset, attack) in original_attacks {
            let matching_json_attack = match attacks.iter().find(|a| a.name == attack.name) {
                Some(a) => a,
                _ => panic!("Could not find attack '{}' in '{}'", attack.name, filename),
            };

            if bin.overwrite_object(offset, matching_json_attack).is_err() {
                panic!(
                    "error overwriting attack '{}' in '{}'",
                    attack.name, filename
                );
            }
        }

        // Write the updated .bin file to the MASTER.DAT
        if let Err(i) = master_dat.update_file(&filename, bin.raw()) {
            panic!(
                "updated file had wrong size: {} instead of {}",
                bin.raw().len(),
                i
            );
        }
    }

    // Write the updated MASTER.DAT and MASTER.DIR to a new file
    master_dat
        .write(Path::new("MASTER.DAT"), Path::new("MASTER.DIR"))
        .expect("could not write out new files");
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
    let mut master_dat = match MasterDat::from_file(&config.master_dat_path, master_dir) {
        Ok(m) => m,
        Err(e) => panic!("failed to read {:?}: {}", &config.master_dat_path, e),
    };

    match config.mode {
        // Read the attacks from the MASTER.DAT, and write them to a JSON file
        Mode::Read => attacks_to_json(&master_dat, config.console, &config.json),

        // Read in the JSON file, and write the new values to the MASTER.DAT,
        // then write a new MASTER.DAT and MASTER.DIR pair with the changes
        Mode::Write => write_new_attack_data(&mut master_dat, config.console, &config.json),
    };
}
