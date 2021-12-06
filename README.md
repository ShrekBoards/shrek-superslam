# shreksuperslam

A Rust library and programs for interacting with the Shrek SuperSlam game files.

## Example

```rust
use std::path::Path;
use shrek_superslam::{Console, MasterDat, MasterDir};
use shrek_superslam::classes::AttackMoveType;
use shrek_superslam::files::Bin;

// Read the MASTER.DAT and MASTER.DIR pair
let mut master_dir = MasterDir::from_file(Path::new("MASTER.DIR"), Console::PC).unwrap();
let mut master_dat = MasterDat::from_file(Path::new("MASTER.DAT"), master_dir).unwrap();

// Parse the Shrek character player.db.bin file, and get the last attack
let mut bin = Bin::new(
    master_dat.decompressed_file("data\\players\\shrek\\player.db.bin").unwrap(),
    Console::PC
).unwrap_or_else(|e| panic!("Failed to read bin file: {:?}", e));
let mut attacks = bin.get_all_objects_of_type::<AttackMoveType>();
let (offset, mut attack) = attacks.pop().unwrap();

// Modify the contents of the attack
attack.damage1 = 100.0;

// Write the new attack back to the .bin file
bin.overwrite_object(offset, &attack).unwrap();

// Write the updated .bin file back to the MASTER.DAT
master_dat.update_file("data\\players\\shrek\\player.db.bin", bin.raw()).unwrap();

// Write the updated MASTER.DAT and MASTER.DIR pair to disk
master_dat.write(Path::new("MASTER.DAT"), Path::new("MASTER.DIR"));

// We have now overwritten the damage of Shrek's last attack!
```

## Binaries

This repository includes multiple programs for reading, interacting with and
modifying the game's files.

### shreksuperslam-extract

A program for extracting the game's compressed file - MASTER.DAT, using the
associated MASTER.DIR file. Tested and working for PC and Gamecube versions.

#### Usage

```sh
./shreksuperslam-extract --dat MASTER.DAT --dir MASTER.DIR --console gc
```

Where:

* `--dat` is the path to the MASTER.DAT
* `--dir` is the path to the associated MASTER.DIR
* `--console` is the game version the files are from. Defaults to `pc` if not given:
   - `pc`: PC
   - `gc`: Gamecube
   - `ps2`: PS2
   - `xbox`: Xbox

### shreksuperslam-repackage

A program for taking an extracted pair of files and repacking them into the
files to be read by the game. This can be used to facilitate modification of the
game's files, for example changing attack values or textures. Tested and working
for PC and Gamecube versions.

#### Usage

```sh
./shreksuperslam-repackage --data data/ --console gc
```

Where:

* `--data` is the path to a 'data' directory created by `shreksuperslam-extract`.
 Currently, there is a restriction that the path must not contain more than one
 directories named 'data', and it must be the last.
* `--console` is the game version the files are from. Defaults to `pc` if not given:
   - `pc`: PC
   - `gc`: Gamecube
   - `ps2`: PS2
   - `xbox`: Xbox

### shreksuperslam-classes

Prints the name and offset of every serialised object within the game's .bin
files.

#### Usage

```sh
./shreksuperslam-classes --dat MASTER.DAT --dir MASTER.DIR --console gc
```

Where:

* `--dat` is the path to the MASTER.DAT
* `--dir` is the path to the associated MASTER.DIR
* `--console` is the game version the files are from. Defaults to `pc` if not given:
   - `pc`: PC
   - `gc`: Gamecube
   - `ps2`: PS2
   - `xbox`: Xbox