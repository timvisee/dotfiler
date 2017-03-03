use std::fs;

use super::dotpath::DotPath;
use super::dotdir::DotDir;

/// Scanner class, that is part of a dotpath.
/// This scanner scans for dotfiles and subdirectories containing dotfiles.
pub struct Scanner<'a> {
    path: &'a mut DotDir
}

impl<'a> Scanner<'a> {

    /// Constructor.
    pub fn new(path: &'a mut DotDir) -> Self {
        Scanner {
            path: path
        }
    }

    /// Scan the dotpath for dotfiles and subdirectories that contain dotfiles.
    pub fn scan(&mut self) {
        // Read the directory, and unwrap the result
        let paths = fs::read_dir(&self.path.get_path()).unwrap();

        // Loop through the list of paths
        for path in paths {
            // Get the entry's path and name
            let entry = path.unwrap().path();
            let entry_name = entry.file_name().unwrap().to_str().unwrap();

            // Create and add the dotpath or dotfile
            if entry.is_dir() {
                self.path.add_dotpath_raw(entry_name);
            } else {
                self.path.add_dotfile_raw(entry_name);
            }
        }
    }
}