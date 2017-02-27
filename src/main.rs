extern crate clap;
extern crate dotfiler;

use clap::App;
use dotfiler::app;

/// Application entry point.
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    println!("Hello, world!");
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
    // Configure the application object with help information, and show matches
    App::new(app::APP_NAME)
        .version(app::APP_VERSION_NAME)
        .author(app::APP_AUTHOR)
        .about(app::APP_DESCRIPTION)
        .get_matches();
}
