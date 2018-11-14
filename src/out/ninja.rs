use core::*;
use std::io::prelude::*;

fn indent(n: usize) -> String {
    static INDENT: usize = 2;
    " ".repeat(INDENT * n)
}

pub fn write_build<W: Write>(mut f: W, build: &Build) {
    for (name, value) in &build.variables {
        writeln!(&mut f, "{0} = {1}", name, value);
    }

    writeln!(&mut f);

    write_rule(&mut f, "mod", "touch -c $out");
    write_rule(&mut f, "fc", "$fc $fflags -c -o $out $in");
    write_rule(&mut f, "ar", "$ar ruUc $out $in");
    write_rule(
        &mut f,
        "link",
        "$fc -o $out $in -Wl,-start-group $libs -Wl,-end-group",
    );
    write_rule(&mut f, "cp", "cp $in $out");

    writeln!(&mut f);

    for link in &build.links {
        write_link(&mut f, link);
    }

    writeln!(
        f,
        "default {}",
        build
            .links
            .iter()
            .map(|x| x.product.clone())
            .collect::<Vec<_>>()
            .join(" ")
    );

    writeln!(
        f,
        "build install: phony {}",
        build
            .links
            .iter()
            .map(|x| format!("$include_path/bin/{}", x.product))
            .collect::<Vec<_>>()
            .join(" ")
    );

    for lib in &build.archives {
        write_archive(&mut f, lib);
    }

    for compile in &build.compiles {
        write_compile(&mut f, compile);
    }
}

fn write_rule<W: Write>(f: &mut W, name: &str, command: &str) {
    writeln!(f, "rule {}", name);
    writeln!(f, "{}command = {}", indent(1), command);
}

fn write_link<W: Write>(f: &mut W, link: &Link) {
    write!(
        f,
        "build {0}: link {1}",
        link.product,
        link.objects.join(" ")
    );
    if link.libs.len() > 0 {
        writeln!(f, "| {0}", link.libs.join(" "));
        writeln!(f, "{0}libs = {1}", indent(1), link.libs.join(" "));
    } else {
        writeln!(f);
    }
    writeln!(f, "build $include_path/bin/{0}: cp {0}", link.product);
}

fn write_archive<W: Write>(f: &mut W, archive: &Archive) {
    writeln!(
        f,
        "build {0}: ar {1}",
        archive.product,
        archive.objects.join(" ")
    );
}

fn write_compile<W: Write>(f: &mut W, compile: &Compile) {
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
