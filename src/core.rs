use config::Config;
use glob;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug)]
pub struct Build {
    pub variables: Vec<(String, String)>,
    pub compiles: Vec<Compile>,
    pub archives: Vec<Archive>,
    pub links: Vec<Link>,
}

fn glob_files(patterns: &Vec<String>) -> Result<Vec<PathBuf>, glob::PatternError> {
    let mut paths = Vec::new();

    for pattern in patterns {
        for entry in glob::glob(pattern)? {
            match entry {
                Ok(path) => paths.push(path),
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
    }

    Ok(paths)
}

impl Build {
    pub fn from_config(config: &Config) -> io::Result<Self> {
        let variables = vec![
            (
                "fc".to_string(),
                config
                    .system
                    .compiler
                    .clone()
                    .unwrap_or("gfortran".to_string()),
            ),
            (
                "fflags".to_string(),
                config.system.fflags.clone().unwrap_or("".to_string()),
            ),
            ("ar".to_string(), "ar".to_string()),
            ("include_path".to_string(), "/usr/local".to_string()),
        ];

        let mut sources = HashSet::new();
        let mut links = Vec::new();
        for exec in &config.target.exe {
            let mut objects = Vec::new();

            let libs: Vec<String> = match &exec.libs {
                Some(libs) => libs.iter().map(get_libname).collect(),
                None => Vec::new(),
            };

            for path in glob_files(&exec.sources).unwrap() {
                let pathstr = path.display().to_string();
                objects.push(get_objname(&pathstr));
                sources.insert(pathstr);
            }

            links.push(Link {
                product: exec.name.to_string(),
                objects: objects,
                libs: libs,
            });
        }

        let mut archives = Vec::new();
        match &config.target.lib {
            Some(libs) => {
                for lib in libs {
                    let mut objects = Vec::new();
                    for path in glob_files(&lib.sources).unwrap() {
                        let pathstr = path.display().to_string();
                        objects.push(get_objname(&pathstr));
                        sources.insert(pathstr);
                    }
                    archives.push(Archive {
                        product: get_libname(&lib.name),
                        objects: objects,
                    });
                }
            }
            None => {}
        }

        let mut compiles = Vec::new();
        for src in &sources {
            compiles.push(Compile::analyze(src)?);
        }

        Ok(Build {
            variables: variables,
            compiles: compiles,
            archives: archives,
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
            static ref mod_proc_re: Regex =
                Regex::new(r"^\s*module\s+procedure\s+([[:alpha:]][[:word:]]*)")
                    .expect("This error can be a bug. Please report to developers.");
            static ref mod_re: Regex = Regex::new(r"^\s*module\s+([[:alpha:]][[:word:]]*)")
                .expect("This error can be a bug. Please report to developers.");
            static ref use_re: Regex = Regex::new(r"^\s*use\s+([[:alpha:]][[:word:]]*)")
                .expect("This error can be a bug. Please report to developers.");
        }

        let mut modules = Vec::new();
        let mut uses = Vec::new();

        let reader = BufReader::new(File::open(source)?);
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(err) => {
                    eprintln!(
                        "Warning: An Error has occured while reading a line at {}:{}",
                        source, index
                    );
                    eprintln!("  {}", err);
                    continue;
                }
            };
            if !mod_proc_re.is_match(&line) {
                for cap in mod_re.captures_iter(&line) {
                    modules.push(get_modname(&cap[1]));
                }
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
    pub libs: Vec<String>,
}

#[derive(Debug)]
pub struct Archive {
    pub product: String,
    pub objects: Vec<String>,
}

fn get_libname<S: Display>(name: &S) -> String {
    format!("lib{}.a", name)
}

fn get_objname<S: Display>(src: &S) -> String {
    format!("{}.o", src)
}

fn get_modname(name: &str) -> String {
    format!("{}.mod", name)
}
