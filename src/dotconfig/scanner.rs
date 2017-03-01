use std::path::PathBuf;

use super::dotpath::DotPath;
use super::dotconfig::DotConfig;

/// Scanner class, that is part of a dotpath.
/// This scanner scans for dotfiles and subdirectories containing dotfiles.
pub struct Scanner<'a> {
    path: &'a DotPath
}

impl<'a> Scanner<'a> {

    /// Constructor.
    pub fn new(path: &'a DotPath) -> Self {
        Scanner {
            path: path
        }
    }

    /// Get the configuration of the path this scanner is for.
    pub fn get_config(&self) -> &DotConfig {
        self.path.get_config()
    }
}