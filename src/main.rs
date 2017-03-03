extern crate clap;

mod app;
mod dot;

use clap::App;
use dotconfig::dotpath::DotPath;
use std::path::PathBuf;

/// Application entry point.
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    println!("Hello, world!");

    // Test the scanning functionality
    let base_path = "/home/timvisee/Documents";
    let mut path = DotPath::new(PathBuf::from(base_path));
    path.scan(&false);
}

/// Handle program arguments passed along with the command line to show things like help pages.
///
/// # Examples
///
/// Parse application parameters, and show relevant information such as help files in the console:
/// ```no_run
/// handle_arguments();
/// ```
fn handle_arguments() {
    // Format the version string
    let version_str = format!("{} ({})", app::APP_VERSION_NAME, app::APP_VERSION_CODE);

    // Configure the application object with help information, and show matches
    App::new(app::APP_NAME)
        .version(version_str.as_str())
        .version_short(app::APP_VERSION_NAME)
        .author(app::APP_AUTHOR)
        .about(app::APP_DESCRIPTION)
        .get_matches();
}
