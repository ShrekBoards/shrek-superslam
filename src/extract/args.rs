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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        args.next();
        let dir = match args.next() {
            Some(arg) => arg,
            _ => return Err("didn't get dir path")
        };

        let console = match args.next() {
            Some(arg) => match arg.as_ref() {
                "gc" => Console::Gamecube,
                "pc" => Console::PC,
                "ps2" => Console::PS2,
                "xbox" => Console::Xbox,
                _ => return Err("unrecognised console")
            },
            _ => return Err("didn't get console")
        };

        Ok(Config {
            master_dat_path: "".to_string(),
            master_dir_path: dir,
            decompress: true,
            extract_texpack: true,
            console: console
        })
    }
}
