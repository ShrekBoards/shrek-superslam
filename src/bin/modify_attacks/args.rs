use std::path::PathBuf;

use getopts::Options;

use shrek_superslam::Console;

/// The mode of the program
pub enum Mode {
    /// Reads the attacks and outputs to a JSON file
    Read,

    /// Reads the JSON file and writes out the attacks
    Write,
}

/// Possible arguments to the program
pub struct Config {
    /// The path to the MASTER.DAT file
    pub master_dat_path: PathBuf,

    /// The path to the MASTER.DIR file
    pub master_dir_path: PathBuf,

    /// The path to the attacks JSON to read or write
    pub json: PathBuf,

    /// The mode of the program
    pub mode: Mode,

    /// The console version of the files
    pub console: Console,
}

impl Config {
    /// Parse the commandline arguments and return them as a new Config
    ///
    /// # Parameters
    ///
    /// - `args`: The commandline arguments passed to the program
    ///
    /// # Returns
    ///
    /// An `Ok(Config)` populated with the passed commandline arguments, or an
    /// `Err(str)` containing an error message if the arguments could not be
    /// parsed.
    pub fn new(args: std::env::Args) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("not enough arguments"));
        }

        let mut opts = Options::new();
        opts.reqopt("a", "dat", "path to MASTER.DAT", "MASTER.DAT");
        opts.reqopt("i", "dir", "path to MASTER.DIR", "MASTER.DIR");
        opts.reqopt(
            "j",
            "json",
            "path to the JSON file to read or write to",
            "shreksuperslam-character-attacks.json",
        );
        opts.reqopt("m", "mode", "read or write mode", "read|write");
        opts.optopt("c", "console", "target console", "gc|pc|ps2|xbox");
        let args: Vec<String> = args.collect();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => return Err(f.to_string()),
        };

        let dat = PathBuf::from(matches.opt_str("a").unwrap());
        let dir = PathBuf::from(matches.opt_str("i").unwrap());
        let json = PathBuf::from(matches.opt_str("j").unwrap());
        let mode = match matches.opt_str("m") {
            Some(m) => match m.to_ascii_lowercase().as_ref() {
                "read" => Mode::Read,
                "write" => Mode::Write,
                _ => {
                    return Err(format!(
                        "unrecognised mode '{}': must be 'read' or 'write'",
                        m
                    ))
                }
            },
            _ => return Err(String::from("no mode given - must be 'read' or 'write'")),
        };
        let console = match matches.opt_str("console") {
            Some(c) => match c.as_ref() {
                "gc" => Console::Gamecube,
                "pc" => Console::PC,
                "ps2" => Console::PS2,
                "xbox" => Console::Xbox,
                _ => {
                    return Err(format!(
                        "unrecognised console '{}': must be one of 'gc', 'pc', 'ps2' or 'xbox'",
                        c
                    ))
                }
            },
            _ => Console::PC,
        };

        Ok(Config {
            master_dat_path: dat,
            master_dir_path: dir,
            json,
            mode,
            console,
        })
    }
}
