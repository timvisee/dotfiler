extern crate config;

use self::config::FileFormat;

pub const APP_NAME: &'static str = "dotfiler";
pub const APP_VERSION_NAME: &'static str = "v0.0.1-alpha";
pub const APP_VERSION_CODE: i32 = 1;
pub const APP_AUTHOR: &'static str = "Tim Visee <timvisee@gmail.com>";
pub const APP_DESCRIPTION: &'static str = "A simple but useful tool to manage your dotfiles.";

pub const CONFIG_FILE_NAME: &'static str = ".dotfiler.yml";
pub const CONFIG_FILE_TYPE: FileFormat = FileFormat::Yaml;