use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;
use toml;

#[derive(Debug)]
pub enum ConfigError {
    TOML(toml::de::Error),
    IO(io::Error),
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::TOML(err)
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IO(err)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::TOML(err) => write!(f, "{}", err),
            ConfigError::IO(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match self {
            ConfigError::TOML(ref err) => err.description(),
            ConfigError::IO(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            ConfigError::TOML(ref err) => Some(err),
            ConfigError::IO(ref err) => Some(err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub package: Package,
    pub system: System,
}

impl Config {
    pub fn load(fname: &str) -> Result<Self, ConfigError> {
        let mut file = File::open(fname)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Config::from_str(&contents)?)
    }
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub compiler: Option<String>,
    pub fflags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
}
