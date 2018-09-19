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
use std::path::PathBuf;

const DEFAULT_INPUT_FILE: &str = "cafemake.toml";
const DEFAULT_OUTPUT_FILE: &str = "build.ninja";

fn glob_files(s: &String) -> std::result::Result<Vec<PathBuf>, glob::PatternError> {
    let mut paths = Vec::new();
    for entry in glob::glob(s)? {
        match entry {
            Ok(path) => paths.push(path),
            Err(err) => println!("{:?}", err),
        }
    }
    Ok(paths)
}

fn main() -> std::result::Result<(), config::ConfigError> {
    let config = Config::load(DEFAULT_INPUT_FILE)?;

    let mut f = File::create(DEFAULT_OUTPUT_FILE)?;

    writeln!(
        &mut f,
        "fc = {}",
        config.system.compiler.unwrap_or("gfortran".to_string())
    );
    writeln!(
        &mut f,
        "fflags = {}",
        config.system.fflags.unwrap_or("".to_string())
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

    let mut compiles = Vec::new();

    for exec in config.target.exe {
        let mut src = Vec::new();
        for s in exec.sources {
            for path in glob_files(&s).unwrap() {
                let pathstr = path.display().to_string();
                compiles.push(Compile::analyze(&pathstr)?);
                src.push(pathstr);
            }
        }
        write_exec(&mut f, &exec.name, &src);
    }

    for compile in compiles {
        write_compile(&mut f, &compile);
    }

    Ok(())
}
