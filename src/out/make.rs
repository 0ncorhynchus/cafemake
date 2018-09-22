use core::*;
use std::io::prelude::*;

pub fn write_build<W: Write>(mut f: W, build: &Build) {
    writeln!(&mut f, ".SUFFIXES:");
    writeln!(&mut f);

    for (name, value) in &build.variables {
        writeln!(&mut f, "{0} = {1}", name, value);
    }

    writeln!(&mut f);

    let mut products = Vec::new();

    for link in &build.links {
        write_link(&mut f, link);
        products.push(link.product.to_string());
    }

    for lib in &build.archives {
        write_archive(&mut f, lib);
        products.push(lib.product.to_string());
    }

    for compile in &build.compiles {
        write_compile(&mut f, compile);
        products.push(compile.object.to_string());
        for module in &compile.modules {
            products.push(module.to_string());
        }
    }

    writeln!(&mut f, ".PHONY: clean");
    writeln!(&mut f, "clean:");
    writeln!(&mut f, "\trm -f {}", products.join(" "));
}

fn write_link<W: Write>(f: &mut W, link: &Link) {
    write!(f, "{}: {}", link.product, link.objects.join(" "));
    if link.libs.len() > 0 {
        write!(f, " {}", link.libs.join(" "));
    }
    writeln!(f);
    writeln!(f, "\t$(fc) -o $@ $^");
}

fn write_archive<W: Write>(f: &mut W, archive: &Archive) {
    writeln!(f, "{}: {}", archive.product, archive.objects.join(" "));
    writeln!(f, "\t$(ar) ruUc $@ $^");
}

fn write_compile<W: Write>(f: &mut W, compile: &Compile) {
    write!(f, "{}: {}", compile.object, compile.source);
    if compile.uses.len() > 0 {
        write!(f, " {}", compile.uses.join(" "));
    }
    writeln!(f);
    writeln!(f, "\t$(fc) $(fflags) -c -o $@ $<");

    for module in &compile.modules {
        writeln!(f, "{0}: {1}", module, compile.object);
        writeln!(f, "\ttouch -c $@");
    }
}
