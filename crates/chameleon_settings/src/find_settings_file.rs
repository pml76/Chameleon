// the following locations are being searched:
// - the home directory: e.g.: ~/.chameleon_settings.json (note: the dot in the beginning of the file name)
// - the current directory ./chameleon_settings.json

use std::env;
use std::fs::canonicalize;
use std::path::{PathBuf};
use crate::settings_file_name::SETTINGS_FILE_NAME;

fn check_settings_path_exists(path: &PathBuf) -> Option<PathBuf> {

    if !path.exists() {
        return None;
    }
    Some(path.clone())

}

#[cfg(target_family = "windows")]
fn check_home_folder() -> Option<PathBuf> {
    let homepath = env::var("HOMEPATH");
    if let Ok(homepath) = homepath {
        let mut path = PathBuf::from(homepath);
        path.push(format!(".{}", SETTINGS_FILE_NAME));
        let e = check_settings_path_exists(&path);
        if let Some(_) = e {
            return e;
        }
    }

    let userprofile = env::var("USERPROFILE");
    if let Ok(userprofile) = userprofile {
        let mut path = PathBuf::from(userprofile);
        path.push(format!(".{}", SETTINGS_FILE_NAME));
        let e = check_settings_path_exists(&path);
        if let Some(_) = e {
            return e;
        }
    }
    
    None
}

#[cfg(target_family = "unix")]
fn check_home_folder() -> Option<PathBuf> {
    let home = env::var("HOME");
    if let Ok(home) = home {
        let mut path = PathBuf::from(home);
        path.push(format!(".{}", SETTINGS_FILE_NAME));
        let e = check_settings_path_exists(path);
        if let Some(_) = e {
            return e;
        }
    }
    None
}

pub fn find_settings_file() -> Option<PathBuf> {
    // Check home folder first
    let home = check_home_folder();
    if let Some(_) = home {
        return home;
    }

    let mut path = canonicalize(PathBuf::from(".")).unwrap();
    path.push(SETTINGS_FILE_NAME);
    check_settings_path_exists(&path)
}