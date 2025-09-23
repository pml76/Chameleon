// the following locations are being searched:
// - the home directory: e.g.: ~/.chameleon_settings.json (note: the dot in the beginning of the file name)
// - the current directory ./chameleon_settings.json

use std::env;
use std::path::{Path, PathBuf};
use crate::settings_file_name::SETTINGS_FILE_NAME;

fn check_settings_file_exists(path: &Path) -> Option<PathBuf> {

    if !path.exists() {
        return None;
    }

    let path_buf = path.join(SETTINGS_FILE_NAME);

    if path_buf.exists() {
        return Some(path_buf);
    }

    None
}

#[cfg(target_family = "windows")]
fn get_home_folder() -> Option<PathBuf> {
    let homepath = env::var("HOMEPATH");
    if let Ok(homepath) = homepath {
        let path = Path::new(&homepath);
        return check_settings_file_exists(path);
    }

    let userprofile = env::var("USERPROFILE");
    if let Ok(userprofile) = userprofile {
        let path = Path::new(&userprofile);
        return check_settings_file_exists(path);       
    }
    
    None
}

#[cfg(target_family = "unix")]
fn get_home_folder() -> Option<std::path::PathBuf> {
    let home = env::var("HOME");
    if let Ok(home) = home {
        let path = Path::new(&home);
        return check_settings_file_exists(path);
    }
    None
}

pub fn find_settings_file() -> Option<PathBuf> {
    // Check home folder first
    if let Some(path) = get_home_folder() {
        return Some(path);
    }

    let path = Path::new(".");
    check_settings_file_exists(path)
}