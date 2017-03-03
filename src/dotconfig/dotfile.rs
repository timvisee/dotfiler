use std::path::Iter;
use std::path::PathBuf;
use super::dotpath::DotPath;
use super::dotdir::DotDir;

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

    fn is_file(&'a self) -> bool {
        true
    }

    fn is_dir(&'a self) -> bool {
        false
    }

    fn get_path(&'a self) -> &'a PathBuf {
        &self.path
    }

    fn get_name(&'a self) -> &'a str {
        self.get_path().file_name().unwrap().to_str().unwrap()
    }

    fn find_dir_iter(&'a self, path: Iter) -> Option<&DotDir> {
        unimplemented!();
        None
    }
}