use config::Config;
use glob;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub command: String,
}

impl Rule {
    pub fn new(name: &str, command: &str) -> Self {
        Rule {
            name: name.to_string(),
            command: command.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct BuildSystem {
    pub compiles: Vec<Compile>,
    pub links: Vec<Link>,
}

pub fn glob_files(s: &String) -> Result<Vec<PathBuf>, glob::PatternError> {
    let mut paths = Vec::new();
    for entry in glob::glob(s)? {
        match entry {
            Ok(path) => paths.push(path),
            Err(err) => eprintln!("{:?}", err),
        }
    }
    Ok(paths)
}

impl BuildSystem {
    pub fn from_config(config: &Config) -> io::Result<Self> {
        let mut sources = HashSet::new();
        let mut links = Vec::new();
        for exec in &config.target.exe {
            let mut objects = Vec::new();
            for src in &exec.sources {
                for path in glob_files(src).unwrap() {
                    let pathstr = path.display().to_string();
                    objects.push(get_objname(&pathstr));
                    sources.insert(pathstr);
                }
            }
            links.push(Link {
                product: exec.name.to_string(),
                objects: objects,
            });
        }

        let mut compiles = Vec::new();
        for src in &sources {
            compiles.push(Compile::analyze(src)?);
        }

        Ok(BuildSystem {
            compiles: compiles,
            links: links,
        })
    }
}

#[derive(Debug)]
pub struct Compile {
    pub source: String,
    pub object: String,
    pub modules: Vec<String>,
    pub uses: Vec<String>,
}

impl Compile {
    pub fn analyze(source: &str) -> io::Result<Self> {
        lazy_static! {
            static ref mod_re: Regex = Regex::new(r"^\s*module\s+([[:alpha:]][[:word:]]*)")
                .expect("This error can be a bug. Please report to developers.");
            static ref use_re: Regex = Regex::new(r"^\s*use\s+([[:alpha:]][[:word:]]*)")
                .expect("This error can be a bug. Please report to developers.");
        }

        let mut modules = Vec::new();
        let mut uses = Vec::new();

        let reader = BufReader::new(File::open(source)?);
        for line in reader.lines() {
            let line = line?;
            for cap in mod_re.captures_iter(&line) {
                modules.push(get_modname(&cap[1]));
            }

            for cap in use_re.captures_iter(&line) {
                uses.push(get_modname(&cap[1]));
            }
        }

        Ok(Compile {
            source: source.to_string(),
            object: get_objname(&source),
            modules: modules,
            uses: uses,
        })
    }
}

#[derive(Debug)]
pub struct Link {
    pub product: String,
    pub objects: Vec<String>,
}

fn get_objname<S: AsRef<str>>(src: &S) -> String {
    format!("{}.o", src.as_ref())
}

fn get_modname(name: &str) -> String {
    format!("{}.mod", name)
}
