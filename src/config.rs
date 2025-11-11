use std::{fs, path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};


/// The container for persistent plugin settings.
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub enable_change_outfit: ConfigParameter<bool>,

}

impl Config {
    /// The path to the configuration file.
    const PATH: &str = "sd:/ultimate/trail_changeoutfit/config.toml";

    /// Creates a new `Config`.
    fn new() -> Self {
        let path = Path::new(Self::PATH);

        if path.exists() {
            return Self::read(path);
        }

        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }

        let config = Self::default();

        config.write(path);

        config
    }

    /// Returns a reference to the lazily initialized `Config`.
    pub const fn get() -> &'static LazyLock<Self> {
        static INSTANCE: LazyLock<Config> = LazyLock::new(Config::new);

        &INSTANCE
    }

    /// Reads and deserializes the configuration file from disk.
    fn read<P: AsRef<Path>>(path: P) -> Self {
        match fs::read_to_string(&path) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => return config,
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to deserialize configuration file: {error}",
                        module_path!(),
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to read configuration file: {error}",
                    module_path!(),
                );
            }
        }

        Self::default()
    }

    /// Serializes and writes the current configuration to disk.
    fn write<P: AsRef<Path>>(&self, path: P) {
        match toml::to_string(self) {
            Ok(toml) => match fs::write(&path, toml) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to write configuration file: {error}",
                        module_path!(),
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to serialize configuration: {error}",
                    module_path!(),
                );
            }
        }
    }
}

/// A configuration parameter that can have different values for each outfit slot.
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigParameter<T> {
    
    pub c00: T,
    pub c01: T,
    pub c02: T,
    pub c03: T,
    pub c04: T,
    pub c05: T,
    pub c06: T,
    pub c07: T,
    pub all_slots: T,
}
