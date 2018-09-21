#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate getopts;
extern crate glob;
extern crate regex;
extern crate toml;

mod config;
mod core;
mod out;

use config::*;
use core::*;
use std::env;
use std::fs::File;

const DEFAULT_INPUT_FILE: &str = "cafemake.toml";
// const DEFAULT_OUTPUT_FILE: &str = "build.ninja";

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    eprintln!("{}", opts.usage(&brief));
}

fn main() -> std::result::Result<(), config::ConfigError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("G", "", "specify the build system", "SYSTEM");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    let build_system = match matches.opt_str("G") {
        Some(s) => s.parse().unwrap(),
        None => out::BuildSystem::Ninja,
    };

    let config = Config::load(DEFAULT_INPUT_FILE)?;

    let build = Build::from_config(&config)?;

    match build_system {
        out::BuildSystem::Ninja => {
            out::ninja::write_build(File::create("build.ninja")?, &build);
        }
        out::BuildSystem::Make => {
            out::make::write_build(File::create("Makefile")?, &build);
        }
    }

    Ok(())
}
