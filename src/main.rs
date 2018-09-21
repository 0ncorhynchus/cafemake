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
mod ninja;

use config::*;
use core::*;
use std::fs::File;

const DEFAULT_INPUT_FILE: &str = "cafemake.toml";
// const DEFAULT_OUTPUT_FILE: &str = "build.ninja";

fn main() -> std::result::Result<(), config::ConfigError> {
    let config = Config::load(DEFAULT_INPUT_FILE)?;

    let build = BuildSystem::from_config(&config)?;

    ninja::write_build(File::create("build.ninja")?, &build);

    Ok(())
}
