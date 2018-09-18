#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate getopts;
extern crate toml;

mod core;
mod config;

use std::fs::File;
use std::io::prelude::*;
use ::config::*;
use ::core::*;

const DEFAULT_INPUT_FILE: &str = "cafemake.toml";
const DEFAULT_OUTPUT_FILE: &str = "build.ninja";

fn main() -> std::io::Result<()> {
    let contents = {
        let mut f = File::open(DEFAULT_INPUT_FILE)?;
        let mut c = String::new();
        f.read_to_string(&mut c)?;
        c
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => { panic!(e.to_string()) },
    };

    let mut f = File::create(DEFAULT_OUTPUT_FILE)?;

    writeln!(&mut f, "fc = {}",
             config.system.compiler.unwrap_or("gfortran".to_string()));
    writeln!(&mut f, "fflags = {}",
             config.system.fflags.unwrap_or("".to_string()));

    write_rule(&mut f, "mod", "touch -c $out");
    write_rule(&mut f, "fc", "$fc $fflags -c -o $out $in");
    write_rule(&mut f, "link", "$fc -o $out $in -Wl,-start-group $libs -Wl,-end-group");

    let mut sources = Vec::new();

    for exec in config.target.exe {
        sources.append(&mut exec.sources.clone());
        write_exec(&mut f, &exec);
    }

    for src in sources {
        write_source(&mut f, &src);
    }

    Ok(())
}
