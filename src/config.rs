
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub target: Target,
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

