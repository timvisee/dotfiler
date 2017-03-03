use std::path::PathBuf;
use super::dotpath::DotPath;

/// Points to a path that contains dotfiles or subdirectories with dotfiles.
/// It also holds the configuration for the specific directory.
pub struct DotFile {
    path: PathBuf
}

impl DotFile {

    /// Constructor.
    pub fn new(path: PathBuf) -> Self {
        DotFile {
            path: path
        }
    }
}

impl<'a> DotPath<'a> for DotFile {

    fn get_path(&'a self) -> &'a PathBuf {
        &self.path
    }

    fn get_name(&'a self) -> &'a str {
        self.get_path().file_name().unwrap().to_str().unwrap()
    }
}