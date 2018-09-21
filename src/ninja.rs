use core::*;
use std::io::prelude::*;

fn indent(n: usize) -> String {
    static INDENT: usize = 2;
    " ".repeat(INDENT * n)
}

pub fn write_rule<W: Write>(f: &mut W, rule: &Rule) {
    writeln!(f, "rule {}", rule.name);
    writeln!(f, "{}command = {}", indent(1), rule.command);
}

pub fn write_build<W: Write>(f: &mut W, build: &BuildSystem) {
    for link in &build.links {
        write_link(f, link);
    }

    for compile in &build.compiles {
        write_compile(f, compile);
    }
}

fn write_link<W: Write>(f: &mut W, link: &Link) {
    writeln!(
        f,
        "build {0}: link {1}",
        link.product,
        link.objects.join(" ")
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
