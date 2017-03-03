use app::CONFIG_FILE_NAME;
use std::path::PathBuf;
use super::dotfile::DotFile;
use super::dotpath::DotPath;
use super::dotconfig::DotConfig;
use super::scanner::Scanner;

/// Points to a path that contains dotfiles or subdirectories with dotfiles.
/// It also holds the configuration for the specific directory.
pub struct DotDir {
    path: PathBuf,
    config: DotConfig,
    children: Vec<DotDir>,
    files: Vec<DotFile>
}

impl DotDir {

    /// Constructor.
    pub fn new(path: PathBuf) -> Self {
        // Create the dotdir object
        let mut dotdir = DotDir {
            path: path,
            config: DotConfig::new(),
            children: Vec::new(),
            files: Vec::new()
        };

        // Load the configuration
        dotdir.load_config();

        // Return the created instance
        dotdir
    }

    /// Get the path the configuration file of this directory would be located at.
    /// A configuration might not be available in this directory.
    fn get_config_path(&self) -> PathBuf {
        // Get the directory path
        let mut config_path = PathBuf::from(&self.path);

        // Append the configuration file name
        config_path.push(CONFIG_FILE_NAME);

        // Return
        config_path
    }

    /// Load the configuration file
    ///
    /// Returns true if any configuration was loaded.
    /// False is returned if the file doesn't exist.
    fn load_config(&mut self) -> bool {
        // Get the configuration path
        let config_path = self.get_config_path();

        // Make sure the configuration file exists, return false if it doesn't
        if !config_path.as_path().is_file() {
            return false;
        }

        // Get the configuration path as a string.
        let config_path_str = config_path.as_path().to_str().unwrap();

        // Load the configuration and return the result
        self.config.load_from_file(config_path_str).unwrap();
        true
    }

    /// Add a new dotpath by it's name.
    ///
    /// The name of the subdirectory should be passed to the `dir` parameter.
    pub fn add_dotpath_raw(&mut self, dir: &str) {
        // Create the path for the subdirectory
        let mut path = PathBuf::from(&self.path);
        path.push(dir);

        // Create the child and add it to the list of children
        self.add_dotpath(Self::new(path));
    }

    /// Add the given child to this dotpath.
    pub fn add_dotpath(&mut self, child: DotDir) {
        self.children.push(child);
    }

    /// Add a dotfile by it's name.
    ///
    /// The name of the dotfile including the extension should be passed to the `name` parameter.
    pub fn add_dotfile_raw(&mut self, name: &str) {
        // Create the path for the file
        let mut path = PathBuf::from(&self.path);
        path.push(name);

        // Create the file and add it to the list of files
        self.add_dotfile(DotFile::new(path));
    }

    /// Add the given dotfile to this dotpath.
    pub fn add_dotfile(&mut self, file: DotFile) {
        self.files.push(file);
    }

    /// Scan this dotpath for dotfiles and subdirectories that contain dotfiles.
    pub fn scan(&mut self, recursive: &bool) {
        // Create a new scanner and initiate a scan
        Scanner::new(self).scan();

        // If to scan recursive, loop through the children
        if *recursive {
            for child in &mut self.children {
                // Scan on each subdirectory
                child.scan(&recursive);
            }
        }
    }
}

impl<'a> DotPath<'a> for DotDir {

    fn get_path(&'a self) -> &'a PathBuf {
        &self.path
    }

    fn get_name(&'a self) -> &'a str {
        self.get_path().file_name().unwrap().to_str().unwrap()
    }
}