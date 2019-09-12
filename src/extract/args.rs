use getopts::Options;

use shrek_superslam::console::Console;

/// Possible arguments to the program
pub struct Config {
    pub master_dat_path: String, // The path to the MASTER.DAT file
    pub master_dir_path: String, // The path to the MASTER.DIR file
    pub decompress: bool,        // Whether the extracted files should be decompressed
    pub extract_texpack: bool,   // Whether decompressed texpacks should be extracted
    pub console: Console,        // The console version of the files
}

impl Config {
    /// Parse the commandline arguments and return them as a new Config
    ///
    /// \param args The commandline arguments passed to the program
    ///
    /// \returns An Ok(Config) populated with the passed commandline arguments,
    ///          or an Err(str) containing an error message if the arguments
    ///          could not be parsed.
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mut opts = Options::new();
        opts.reqopt("a", "dat", "path to MASTER.DAT", "MASTER.DAT");
        opts.reqopt("i", "dir", "path to MASTER.DIR", "MASTER.DIR");
        opts.reqopt("c", "console", "target console", "gc|pc|ps2|xbox");
        opts.optflag("d", "decompress", "decompress files");
        let args : Vec<String> = args.collect();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        let dat = matches.opt_str("a").unwrap();
        let dir = matches.opt_str("i").unwrap();
        let console = match matches.opt_str("c") {
            Some(c) => match c.as_ref() {
                "gc" => Console::Gamecube,
                "pc" => Console::PC,
                "ps2" => Console::PS2,
                "xbox" => Console::Xbox,
                _ => return Err("unrecognised console"),
            },
            _ => return Err("unrecognised console")
        };

        Ok(Config {
            master_dat_path: dat,
            master_dir_path: dir,
            decompress: false,
            extract_texpack: true,
            console: console
        })
    }
}
