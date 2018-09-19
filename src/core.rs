use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use regex::Regex;

const INDENT: usize = 2;

pub fn indent(n: usize) -> String {
    " ".repeat(INDENT * n)
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub command: String
}

impl Rule {
    pub fn new(name: &str, command: &str) -> Self {
        Rule {
            name:    name.to_string(),
            command: command.to_string(),
        }
    }
}

pub fn write_rule<W: Write>(f: &mut W, rule: &Rule) {
    writeln!(f, "rule {}", rule.name);
    writeln!(f, "{}command = {}", indent(1), rule.command);
}

pub fn get_objname(src: &String) -> String {
    format!("{}.o", src)
}

pub fn write_exec<W: Write>(f: &mut W, name: &str, objs: &Vec<String>) {
    writeln!(f, "build {0}: link {1}", name,
             objs.iter()
                 .map(get_objname)
                 .collect::<Vec<_>>()
                 .join(" "));
}

pub fn write_source<W: Write>(f: &mut W, src: &String) {
    let dependency = Dependency::analyze(src).unwrap();
    let obj = get_objname(src);

    write!(f, "build {0}: fc {1}", obj, src);
    if dependency.uses.len() != 0 {
        write!(f, " | {}",
               dependency.uses.iter()
                              .map(|x| format!("{}.mod", x))
                              .collect::<Vec<_>>()
                              .join(" "));
    }
    writeln!(f);

    for module in dependency.modules {
        writeln!(f, "build {0}.mod: mod | {1} {2}", module, src, obj);
    }
}

#[derive(Debug)]
struct Dependency {
    pub modules: Vec<String>,
    pub uses: Vec<String>,
}

impl Dependency {
    pub fn analyze(source: &str) -> io::Result<Self> {
        lazy_static! {
            static ref mod_re: Regex = Regex::new(r"^\s*module\s+([[:alpha:]][[:word:]]*)").unwrap();
            static ref use_re: Regex = Regex::new(r"^\s*use\s+([[:alpha:]][[:word:]]*)").unwrap();
        }

        let mut modules = Vec::new();
        let mut uses = Vec::new();

        let reader = BufReader::new(File::open(source)?);
        for line in reader.lines() {
            let line = line?;
            for cap in mod_re.captures_iter(&line) {
                modules.push(cap[1].to_string());
            }

            for cap in use_re.captures_iter(&line) {
                uses.push(cap[1].to_string());
            }
        }

        Ok(Dependency {
            modules: modules,
            uses: uses
        })
    }
}
