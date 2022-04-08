use std::env;
use std::fs;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::classes::ShrekSuperSlamObject;
use shrek_superslam::files::Bin;

mod args;
use args::Config;

fn print_db(name: &str, obj: ShrekSuperSlamObject, tab: usize) {
    print!("{}{}", ("\t".repeat(tab)), name);

    match obj {
        ShrekSuperSlamObject::LocalizedString(s) => {
            print!(" ({})", &s.string);
        }
        ShrekSuperSlamObject::ScriptDb(db) => {
            for (n, o) in db.objects {
                print_db(&n, o, tab + 1);
            }
        },
        _ => (),
    }

    print!("\n");
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Unable to parse args: {}", err);
        process::exit(1);
    });

    // Read the .bin file and extract the gf::DB object
    let bin = Bin::new(fs::read(&config.db_path).unwrap(), config.console).unwrap();
    let parsed = bin.parse().unwrap();

    // Print each entry in the DB.
    for (name, obj) in parsed {
        print_db(&name, obj, 0);
    }
}
