#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub system: System,
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub compiler: Option<String>,
    pub fflags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub exe: Vec<Exec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exec {
    pub name: String,
    pub sources: Vec<String>,
}
