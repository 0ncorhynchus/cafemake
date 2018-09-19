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

use config::*;
use core::*;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_INPUT_FILE: &str = "cafemake.toml";
const DEFAULT_OUTPUT_FILE: &str = "build.ninja";

fn main() -> std::result::Result<(), config::ConfigError> {
    let config = Config::load(DEFAULT_INPUT_FILE)?;

    let mut f = File::create(DEFAULT_OUTPUT_FILE)?;

    writeln!(
        &mut f,
        "fc = {}",
        config
            .system
            .compiler
            .clone()
            .unwrap_or("gfortran".to_string())
    );
    writeln!(
        &mut f,
        "fflags = {}",
        config.system.fflags.clone().unwrap_or("".to_string())
    );

    let rules = vec![
        Rule::new("mod", "touch -c $out"),
        Rule::new("fc", "$fc $fflags -c -o $out $in"),
        Rule::new(
            "link",
            "$fc -o $out $in -Wl,-start-group $libs -Wl,-end-group",
        ),
    ];
    for rule in &rules {
        write_rule(&mut f, rule);
    }

    let build = BuildSystem::from_config(&config)?;

    for link in &build.links {
        write_link(&mut f, link);
    }

    for compile in &build.compiles {
        write_compile(&mut f, compile);
    }

    Ok(())
}
