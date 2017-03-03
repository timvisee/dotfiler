use std::path::Iter;
use std::path::PathBuf;
use super::dotdir::DotDir;

pub trait DotPath<'a> {

    /// Check whether this dotpath is a file.
    fn is_file(&'a self) -> bool;

    /// Check whether this dotpath is a directory.
    fn is_dir(&'a self) -> bool;

    /// Get the path as `PathBuf`.
    fn get_path(&'a self) -> &'a PathBuf;

    /// Get the name of the directory of file
    fn get_name(&'a self) -> &'a str;

    /// Check whether the given name matches the name of this path.
    fn is_name(&'a self, name: &'a str) -> bool {
        self.get_name() == name
    }

    /// Find a dotpath.
    ///
    /// The path given to `path` should be relative to the current path.
    fn find_dir(&'a self, path: &'a str) -> Option<&DotDir> {
        // Create a path buf instance
        let path_buf = PathBuf::from(path);

        // Iterate over the path components
        self.find_dir_iter(path_buf.iter())
    }

    fn find_dir_iter(&'a self, path: Iter) -> Option<&DotDir>;
}