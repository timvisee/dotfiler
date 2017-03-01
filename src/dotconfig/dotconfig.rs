extern crate config;

use app::CONFIG_FILE_TYPE;
use std::error::Error;
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
    ///
    /// The file path should be passed to the `file` parameter.
    pub fn load_from_file(mut self, file: &str) -> Result<(), Box<Error>> {
        self.c.merge(File::new(file, CONFIG_FILE_TYPE))
    }
}
