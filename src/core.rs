use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

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

fn indent(n: usize) -> String {
    static INDENT: usize = 2;
    " ".repeat(INDENT * n)
}

fn get_objname<S: AsRef<str>>(src: &S) -> String {
    format!("{}.o", src.as_ref())
}

fn get_modname(name: &str) -> String {
    format!("{}.mod", name)
}

pub fn write_rule<W: Write>(f: &mut W, rule: &Rule) {
    writeln!(f, "rule {}", rule.name);
    writeln!(f, "{}command = {}", indent(1), rule.command);
}

pub fn write_exec<W: Write>(f: &mut W, name: &str, objs: &Vec<String>) {
    writeln!(
        f,
        "build {0}: link {1}",
        name,
        objs.iter().map(get_objname).collect::<Vec<_>>().join(" ")
    );
}

pub fn write_compile<W: Write>(f: &mut W, compile: &Compile) {
    write!(f, "build {0}: fc {1}", compile.object, compile.source);
    if compile.uses.len() != 0 {
        write!(f, " | {}", compile.uses.join(" "));
    }
    writeln!(f);

    for module in &compile.modules {
        writeln!(
            f,
            "build {0}: mod | {1} {2}",
            module, compile.source, compile.object
        );
    }
}
