use crate::config::Config;
use regex::Regex;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Build {
    pub variables: Vec<(String, String)>,
    pub compiles: Vec<Compile>,
    pub archives: Vec<Archive>,
    pub links: Vec<Link>,
    pub mod_dir: PathBuf,
    pub build_dir: PathBuf,
    source_dir: PathBuf,
}

impl Build {
    pub fn new() -> Self {
        Build {
            variables: Vec::new(),
            compiles: Vec::new(),
            archives: Vec::new(),
            links: Vec::new(),
            build_dir: PathBuf::from("build"),
            source_dir: PathBuf::from("src"),
            mod_dir: PathBuf::from("build"),
        }
    }

    fn push_variables(&mut self, name: &str, value: String) {
        self.variables.push((String::from(name), value));
    }

    pub fn from_config(config: &Config) -> io::Result<Self> {
        let mut build = Self::new();

        build.push_variables(
            "fc",
            config
                .system
                .compiler
                .clone()
                .unwrap_or(String::from("gfortran")),
        );

        build.push_variables(
            "fflags",
            config.system.fflags.clone().unwrap_or(String::new()),
        );

        build.push_variables("ar", String::from("ar"));
        build.push_variables("install_prefix", String::from("/usr/local"));

        let mut sources = Vec::new();
        visit_dirs(build.source_dir.clone(), &mut |path| {
            if let Some(ext) = path.extension() {
                if let Some(ext) = ext.to_str() {
                    if ext.to_lowercase().starts_with("f") {
                        sources.push(path)
                    }
                }
            }
        })?;

        for source in &sources {
            build.compiles.push(build.resolve_dependencies(source)?);
        }

        build.links.push(Link {
            product: build.build_dir.join(&config.package.name),
            objects: sources.iter().map(|path| build.get_objpath(path)).collect(),
            libs: Vec::new(),
        });

        Ok(build)
    }

    fn get_objpath<P: AsRef<Path>>(&self, source: P) -> PathBuf {
        self.build_dir
            .join(source.as_ref().strip_prefix(&self.source_dir).unwrap())
            .with_extension("o")
    }

    fn get_mod_path(&self, name: &str) -> PathBuf {
        self.mod_dir.join(name).with_extension("mod")
    }

    fn resolve_dependencies<P: AsRef<Path>>(&self, source: P) -> io::Result<Compile> {
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

        let reader = BufReader::new(File::open(&source)?);
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(err) => {
                    eprintln!(
                        "Warning: An Error has occured while reading a line at {}:{}",
                        source.as_ref().display(),
                        index
                    );
                    eprintln!("  {}", err);
                    continue;
                }
            };
            if !mod_proc_re.is_match(&line) {
                for cap in mod_re.captures_iter(&line) {
                    modules.push(self.get_mod_path(&cap[1]));
                }
            }

            for cap in use_re.captures_iter(&line) {
                uses.push(self.get_mod_path(&cap[1]));
            }
        }

        Ok(Compile {
            source: source.as_ref().to_path_buf(),
            object: self.get_objpath(source),
            modules: modules,
            uses: uses,
        })
    }
}

fn visit_dirs(dir: PathBuf, f: &mut FnMut(PathBuf)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let path = entry?.path();
            visit_dirs(path, f)?;
        }
    } else {
        f(dir);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Compile {
    pub source: PathBuf,
    pub object: PathBuf,
    pub modules: Vec<PathBuf>,
    pub uses: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Link {
    pub product: PathBuf,
    pub objects: Vec<PathBuf>,
    pub libs: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Archive {
    pub product: PathBuf,
    pub objects: Vec<PathBuf>,
}
