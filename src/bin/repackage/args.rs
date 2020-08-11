use std::path::PathBuf;

use getopts::Options;

use shrek_superslam::console::Console;

/// Possible arguments to the program
pub struct Config {
    pub data_path: PathBuf, // The path to the extracted data/ directory
    pub console: Console,   // The console version of the files
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
        opts.reqopt(
            "d",
            "data",
            "path to the extracted data/ directory",
            "data/",
        );
        opts.optopt("c", "console", "target console", "gc|pc|ps2|xbox");
        let args: Vec<String> = args.collect();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => return Err(f.to_string()),
        };

        let data = PathBuf::from(matches.opt_str("d").unwrap());
        let console = match matches.opt_str("console") {
            Some(c) => match c.as_ref() {
                "gc" => Console::Gamecube,
                "pc" => Console::PC,
                "ps2" => Console::PS2,
                "xbox" => Console::Xbox,
                _ => return Err(String::from("unrecognised console")),
            },
            _ => Console::PC,
        };

        Ok(Config {
            data_path: data,
            console,
        })
    }
}
