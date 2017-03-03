use std::path::PathBuf;

pub trait DotPath<'a> {

    /// Get the path as `PathBuf`.
    fn get_path(&'a self) -> &'a PathBuf;

    /// Get the name of the directory of file
    fn get_name(&'a self) -> &'a str;
}