use std::env;
use std::fs;
use std::process;

extern crate shrek_superslam;
use shrek_superslam::master_dir::MasterDir;

mod args;
mod dat;
use args::Config;
use dat::dump_master_dat;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        process::exit(1);
    });

    let master_dir = fs::read(&config.master_dir_path)
       .expect("unable to read master.dir");

    let master_dat = fs::read(&config.master_dat_path)
       .expect("unable to read master.dat");

    let dir = MasterDir::new(master_dir, &config.console);
    dump_master_dat(master_dat, dir, &config);
}
