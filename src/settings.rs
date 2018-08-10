use config::{Config, ConfigError, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Source {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Archive {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub archive: Archive,
    pub source: Source,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or("default".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;

        s.try_into()
    }
}
