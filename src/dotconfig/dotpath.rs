use app::CONFIG_FILE_NAME;
use std::path::PathBuf;
use super::dotconfig::DotConfig;

/// Points to a path that contains dotfiles or subdirectories with dotfiles.
/// It also holds the configuration for the specific directory.
pub struct DotPath<'a> {
    path: PathBuf,
    config: DotConfig,
    parent: Option<&'a DotPath<'a>>
}

impl<'a> DotPath<'a> {

    /// Constructor.
    pub fn new(path: PathBuf, parent: Option<&'a DotPath<'a>>) -> Self {
        DotPath {
            path: path,
            config: DotConfig::new(),
            parent: parent
        }
    }

    /// Get a configuration reference.
    pub fn get_config(&self) -> &DotConfig {
        &self.config
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
    pub fn load_config(mut self) -> bool {
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
}