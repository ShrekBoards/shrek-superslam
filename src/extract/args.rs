use std::env::Args;

pub enum Console {
    Gamecube,
    PC,
    PS2,
    Xbox,
}

pub struct Config {
    pub master_dat_path: String,
    pub master_dir_path: String,
    pub decompress: bool,
    pub extract_texpack: bool,
    pub console: Console,
}

impl Config {
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
