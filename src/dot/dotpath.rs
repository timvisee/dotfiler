use app::CONFIG_FILE_NAME;
use std::error::Error;
use std::path::Iter;
use std::path::PathBuf;
use super::dotconfig::DotConfig;
use super::scanner::Scanner;

/// Points to a path that contains dotfiles or subdirectories with dotfiles.
/// It also holds the configuration for the specific directory.
pub struct DotPath {
    path: PathBuf,
    config: DotConfig,
    children: Vec<DotPath>
}

impl DotPath {

    /// Constructor.
    pub fn new(path: PathBuf) -> Self {
        // Create the dotpath object
        let mut dotpath = DotPath {
            path: path,
            config: DotConfig::new(),
            children: Vec::new()
        };

        // Load the configuration and unwrap it
        dotpath.load_config().unwrap();

        // Return the created instance
        dotpath
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
    /// Returns a result object containing a boolean on success.
    /// The boolean is true if any configuration was loaded, or false when there wasn't any configuration.
    /// An error is returned if loading a configuration failed.
    fn load_config(&mut self) -> Result<bool, Box<Error>> {
        // Get the configuration path
        let config_path = self.config_path();

        // Make sure the configuration file exists, return false if it doesn't
        if !config_path.as_path().is_file() {
            return Ok(false);
        }

        // Get the configuration path as a string.
        let config_path_str = config_path.as_path().to_str().unwrap();

        // Load the configuration, then process and return the result
        let result = self.config.load_from_file(config_path_str);
        if result.is_ok() {
            Ok(true)
        } else {
            Err(result.unwrap_err())
        }
    }

    /// Add a new child to this dotpath by it's name as a string.
    ///
    /// The name of the child to add. This should be the name of the directory or file (with extension).
    pub fn add_child_by_name(&mut self, name: &str) {
        // Don't add a child with a name that's already added
        if self.has_child_by_name(&name) {
            return;
        }

        // Create the path for the subdirectory
        let mut path = PathBuf::from(&self.path);
        path.push(name);

        // Create the child and add it to the list of children
        self.add_child(Self::new(path));
    }

    /// Add the given child to this dotpath.
    pub fn add_child(&mut self, child: DotPath) {
        // Don't add a child with a name that's already added
        if self.has_child_by_name(child.name()) {
            return;
        }

        // Add the child
        self.children.push(child);
    }

    /// Check whether there's any child with the given name.
    ///
    /// The name of the child to check for should be passed to the `name` parameter.
    pub fn has_child_by_name(&self, name: &str) -> bool {
        // Loop through the children
        for child in &self.children {
            // Compare the name, return true if it's equal
            if child.is_name(&name) {
                return true;
            }
        }

        // No child found with this name, return false
        false
    }

    /// Scan this dotpath for dotfiles and subdirectories that contain dotfiles.
    pub fn scan(&mut self, recursive: &bool) {
        // Only scan if this is a directory
        if !self.is_dir() {
            return;
        }

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

    /// Find a child dotpath.
    ///
    /// The path given to the `path` parameter should be relative to the current dotpath.
    pub fn find(&self, path: &str) -> Option<&DotPath> {
        // Return self if the path is empty
        if path.trim().is_empty() {
            return Some(&self);
        }

        // Find the dotpath using a helper function and an iterator
        self.find_iter(PathBuf::from(path).iter())
    }

    /// Find a child dotpath, using the given iterator.
    ///
    /// The iterator given to `path` must be an iterator over path components.
    pub fn find_iter(&self, mut path: Iter) -> Option<&DotPath> {
        // Iterate over the next path component
        match path.next() {
            Some(comp) => {
                // Get the component name
                let name = comp.to_str().unwrap();

                // Loop through the children to find a matching dotpath
                for child in &self.children {
                    if child.is_name(name) {
                        // Find on the child
                        return child.find_iter(path);
                    }
                }

                // No child found, return none
                None
            },

            // If there are no components left, return self
            None => Some(&self)
        }
    }
}
