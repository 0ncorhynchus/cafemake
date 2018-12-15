use super::format_paths;
use crate::core::*;
use std::io;
use std::io::prelude::*;

pub fn write_build<W: Write>(mut f: W, build: &Build) -> io::Result<()> {
    writeln!(&mut f, ".SUFFIXES:")?;
    writeln!(&mut f)?;

    for (name, value) in &build.variables {
        writeln!(&mut f, "{0} = {1}", name, value)?;
    }
    writeln!(&mut f, "moddir = {}", build.mod_dir.display())?;

    writeln!(&mut f)?;

    let mut products = Vec::new();

    for link in &build.links {
        write_link(&mut f, link)?;
        products.push(link.product.display().to_string());
    }

    for lib in &build.archives {
        write_archive(&mut f, lib)?;
        products.push(lib.product.display().to_string());
    }

    for compile in &build.compiles {
        write_compile(&mut f, compile)?;
        products.push(compile.object.display().to_string());
        for module in &compile.modules {
            products.push(module.display().to_string());
        }
    }

    writeln!(&mut f, ".PHONY: clean")?;
    writeln!(&mut f, "clean:")?;
    writeln!(&mut f, "\t@rm -f {}", products.join(" "))?;
    writeln!(&mut f, "\t@echo Cleaning up...")?;

    Ok(())
}

fn write_link<W: Write>(f: &mut W, link: &Link) -> io::Result<()> {
    let libs = format_paths(&link.libs);
    write!(
        f,
        "{}: {}",
        link.product.display(),
        format_paths(&link.objects)
    )?;
    if libs.len() > 0 {
        write!(f, " {}", libs)?;
    }
    writeln!(f)?;

    write!(f, "\t$(fc) -o $@ $^")?;
    // if libs.len() > 0 {
    //     write!(f, " -Wl,-start-group {} -Wl,-end-group", libs)?;
    // }
    writeln!(f)?;

    Ok(())
}

fn write_archive<W: Write>(f: &mut W, archive: &Archive) -> io::Result<()> {
    writeln!(
        f,
        "{}: {}",
        archive.product.display(),
        format_paths(&archive.objects)
    )?;
    writeln!(f, "\t$(ar) ruUc $@ $^")?;
    Ok(())
}

fn write_compile<W: Write>(f: &mut W, compile: &Compile) -> io::Result<()> {
    let object = compile.object.display();
    write!(f, "{}: {}", object, compile.source.display())?;
    if compile.uses.len() > 0 {
        write!(f, " {}", format_paths(&compile.uses))?;
    }
    writeln!(f)?;
    writeln!(f, "\t$(fc) $(fflags) -I$(moddir) -J$(moddir) -c -o $@ $<")?;

    for module in &compile.modules {
        writeln!(f, "{0}: {1}", module.display(), object)?;
        writeln!(f, "\ttouch -c $@")?;
    }

    Ok(())
}
