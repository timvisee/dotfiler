extern crate config;

use self::config::{Config, File, FileFormat};

pub struct DotConfig {
    c: Config
}

impl DotConfig {

    /// Constructor
    pub fn new() -> Self {
        DotConfig {
            c: Config::new()
        }
    }

    /// Load configuration from the given file.
    pub fn load_from_file(&mut self, file: &str) {
        &self.c.merge(File::new(file, FileFormat::Yaml)).unwrap();
    }
}
