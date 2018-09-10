extern crate config;
use self::config::{Config, ConfigError, File};
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
    pub crlf: bool
}

impl Settings {
    pub fn new(config: bool) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.set_default("debug", false)?;
        s.set_default("archive.path", "./")?;
        s.set_default("source.path", "./")?;
        s.set_default("delimiter", ",")?;
        s.set_default("crlf", false)?;

        if config {
            s.merge(File::with_name("config/default").required(false))?;

            let env = env::var("RUN_MODE").unwrap_or("default".into());
            s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

            s.merge(File::with_name("config/local").required(false))?;
        }

        s.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::new(false).unwrap();
        assert_eq!(settings.debug, false);
        assert_eq!(settings.archive.path, "./");
        assert_eq!(settings.source.path, "./");
        assert_eq!(settings.delimiter, ",");
    }
}
