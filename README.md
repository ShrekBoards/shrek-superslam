# shreksuperslam

A Rust library and programs for interacting with the Shrek SuperSlam game files.

## shreksuperslam-extract

A program for extracting the game's compressed file - MASTER.DAT, using the
associated MASTER.DIR file. Tested and working for PC and Gamecube versions.

### Usage

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

## shreksuperslam-repackage

A program for taking an extracted pair of files and repacking them into the
files to be read by the game. This can be used to facilitate modification of the
game's files, for example changing attack values or textures. Tested and working
for PC and Gamecube versions.

### Usage

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

## shreksuperslam-classes

Prints the name and offset of every serialised object within the game's .bin
files.

### Usage

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