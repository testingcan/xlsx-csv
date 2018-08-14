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
    pub delimiter: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.set_default("debug", false)?;
        s.set_default("archive.path", "./")?;
        s.set_default("source.path", "./")?;
        s.set_default("delimiter", ",")?;

        s.merge(File::with_name("config/default").required(false))?;

        let env = env::var("RUN_MODE").unwrap_or("default".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;

        s.try_into()
    }
}
