use std::io::prelude::*;
use ::config::*;

const INDENT: usize = 2;

pub fn indent(n: usize) -> String {
    " ".repeat(INDENT * n)
}

pub fn write_rule<W: Write>(f: &mut W, name: &str, command: &str) {
    writeln!(f, "rule {}", name);
    writeln!(f, "{}command = {}", indent(1), command);
}

pub fn get_objname(src: &String) -> String {
    format!("{}.o", src)
}

pub fn write_exec<W: Write>(f: &mut W, exec: &Exec) {
    writeln!(f, "build {0}: link {1}", exec.name,
             exec.sources.iter()
                         .map(get_objname)
                         .collect::<Vec<_>>()
                         .join(" "));
}

pub fn write_source<W: Write>(f: &mut W, src: &String) {
    writeln!(f, "build {0}: fc {1}", get_objname(src), src);
}
