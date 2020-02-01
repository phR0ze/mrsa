use fungus::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::BufWriter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Config structure version.
    pub version: u8,
}

impl Config {
    /// Create a new Config
    pub fn new() -> Config {
        Default::default()
    }

    /// Load the config from the given `path`
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Config> {
        let path = path.as_ref().abs()?;
        let file = fs::File::open(path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    /// Save the config to disk at the given `path`
    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let path = path.as_ref().abs()?;
        sys::mkdir(path.dir()?)?;

        let mut file = BufWriter::new(File::create(path)?);
        file.write_all(&serde_yaml::to_vec(self)?)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { version: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test setup
    fn setup<T: AsRef<Path>>(path: T) -> PathBuf {
        let temp = PathBuf::from("tests/temp").abs().unwrap();
        sys::mkdir(&temp).unwrap();
        temp.mash(path.as_ref())
    }

    #[test]
    fn test_config_load_save() {
        let tmpdir = setup("config_load_save");
        let config_path = tmpdir.mash("config.yaml");
        assert!(sys::remove_all(&tmpdir).is_ok());

        // Empty config
        assert_eq!(sys::exists(&config_path), false);
        let config = Config::new();
        assert!(config.save(&config_path).is_ok());
        assert_eq!(sys::exists(&config_path), true);

        let config = Config::load(&config_path).unwrap();
        assert_eq!(config.version, 1);

        // Populated config
        assert!(sys::remove(&config_path).is_ok());
        assert_eq!(sys::exists(&config_path), false);
        let mut config = Config::new();
        config.version = 2;
        assert!(config.save(&config_path).is_ok());
        assert_eq!(sys::exists(&config_path), true);

        let config = Config::load(&config_path).unwrap();
        assert_eq!(config.version, 2);

        assert!(sys::remove_all(&tmpdir).is_ok());
    }
}
