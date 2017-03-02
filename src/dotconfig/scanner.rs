use std::fs;
use std::path::PathBuf;

use super::dotpath::DotPath;
use super::dotconfig::DotConfig;

/// Scanner class, that is part of a dotpath.
/// This scanner scans for dotfiles and subdirectories containing dotfiles.
pub struct Scanner<'a> {
    path: &'a mut DotPath
}

impl<'a> Scanner<'a> {

    /// Constructor.
    pub fn new(path: &'a mut DotPath) -> Self {
        Scanner {
            path: path
        }
    }

    /// Get the configuration of the path this scanner is for.
    pub fn get_config(&self) -> &DotConfig {
        self.path.get_config()
    }

    /// Scan the dotpath for dotfiles and subdirectories that contain dotfiles.
    pub fn scan(&mut self) {
        // Read the directory, and unwrap the result
        let paths = fs::read_dir(&self.path.get_path()).unwrap();

        // Loop through the list of paths
        for path in paths {
            // Get the entry's path
            let entry = path.unwrap().path();

            // Check whether it's a directory
            if entry.is_dir() {
                // Create a child path
                // TODO: Should we load the configuration for this child path?
                self.path.create_child(entry.file_name().unwrap().to_str().unwrap());

            } else {
                // TODO: Load the dotfile
            }
        }
    }
}