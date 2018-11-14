use core::*;
use std::fs::File;
use std::io;
use std::str::FromStr;

mod make;
mod ninja;

pub enum BuildSystem {
    Ninja,
    Make,
}

impl BuildSystem {
    pub fn write_build(&self, build: &Build) -> io::Result<()> {
        match self {
            BuildSystem::Ninja => ninja::write_build(File::create("build.ninja")?, build),
            BuildSystem::Make => make::write_build(File::create("Makefile")?, build),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseBuildSystemError;

impl FromStr for BuildSystem {
    type Err = ParseBuildSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ninja" => Ok(BuildSystem::Ninja),
            "make" => Ok(BuildSystem::Make),
            _ => Err(ParseBuildSystemError),
        }
    }
}
