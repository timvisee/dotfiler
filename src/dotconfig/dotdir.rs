use app::CONFIG_FILE_NAME;
use std::path::Iter;
use std::path::PathBuf;
use super::dotconfig::DotConfig;
use super::scanner::Scanner;

/// Points to a path that contains dotfiles or subdirectories with dotfiles.
/// It also holds the configuration for the specific directory.
pub struct DotDir {
    path: PathBuf,
    config: DotConfig,
    children: Vec<DotDir>
}

impl DotDir {

    /// Constructor.
    pub fn new(path: PathBuf) -> Self {
        // Create the dotdir object
        let mut dotdir = DotDir {
            path: path,
            config: DotConfig::new(),
            children: Vec::new()
        };

        // Load the configuration
        // TODO: Make sure the configuration was loaded successfully!
        dotdir.load_config();

        // Return the created instance
        dotdir
    }

    /// Get the path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Check whether this dotpath is a file.
    /// If the dotpath doesn't exist, false is returned.
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    /// Check whether this dotpath is a directory.
    /// If the dotpath doesn't exist, false is returned.
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    /// Get the name of this dotpath.
    /// If this dotpath is a directory, the name of the directory is returned.
    /// If this dotpath is a file, the name of the file is returned including it's extension.
    pub fn name(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    /// Check whether the given name equals the name of the dotpath.
    /// The given name is compared to the output of `get_name()`.
    pub fn is_name(&self, name: &str) -> bool {
        self.name() == name
    }

    /// Get the path the configuration file of this directory would be located at.
    /// A configuration might not be available in this directory.
    fn config_path(&self) -> PathBuf {
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
        let config_path = self.config_path();

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

    /// Add a new child to this dotpath by it's name as a string.
    ///
    /// The name of the child to add. This should be the name of the directory or file (with extension).
    pub fn add_child_by_name(&mut self, name: &str) {
        // Create the path for the subdirectory
        let mut path = PathBuf::from(&self.path);
        path.push(name);

        // Create the child and add it to the list of children
        self.add_child(Self::new(path));
    }

    /// Add the given child to this dotpath.
    pub fn add_child(&mut self, child: DotDir) {
        // TODO: Make sure we aren't adding duplicate entries!

        // Add the child
        self.children.push(child);
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

    // TODO: To implement!
//    pub fn find(&self, dir: &str) -> Option<&DotDir> {
//        // Loop through the dotpath's to find the matching one
//        for path in &self.children {
//            if path.is_name(path.name()) {
//                return Some(path);
//            }
//        }
//
//        // Not found, return nothing
//        None
//    }
//
//    pub fn find_dir_iter(&self, mut path: Iter) -> Option<&DotDir> {
//        // TODO: Implement this
//
//        match path.next() {
//            Some(comp) => self.find(&comp.to_str().unwrap()),
//            None => None
//        }
//    }
}
