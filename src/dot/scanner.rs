use std::fs;

use super::dotpath::DotPath;

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

    /// Scan the dotpath for dotfiles and subdirectories that contain dotfiles.
    pub fn scan(&mut self) {
        // Read the directory, and unwrap the result
        let paths = fs::read_dir(&self.path.path()).unwrap();

        // Loop through the list of paths
        for path in paths {
            // Get the entry's path
            let entry = path.unwrap().path();

            // Get the entry name
            let entry_name = entry.file_name().unwrap().to_str().unwrap();

            // Add the entry as a child
            self.path.add_child_by_name(entry_name);
        }
    }
}