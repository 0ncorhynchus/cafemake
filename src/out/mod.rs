use std::str::FromStr;

pub mod make;
pub mod ninja;

pub enum BuildSystem {
    Ninja,
    Make,
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
