use std::path::{Path, PathBuf};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::find_settings_file::find_settings_file;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalSettings {
    pub locales_directory: PathBuf,
    pub default_locale: String,
}

impl GlobalSettings {

    pub fn load_global_settings(path: &Path) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let settings: GlobalSettings = serde_json::from_reader(file)?;
        Ok(settings)
    }
    pub fn save_global_settings(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

pub fn get_global_settings() -> Option<GlobalSettings> {
    let path_buf = find_settings_file();
    if let Some(path_buf) = path_buf {
        let s = GlobalSettings::load_global_settings(path_buf.as_path());
        if let Ok(s) = s {
            return Some(s);
        }
    }
    panic!("Failed to load global settings");
}
static GLOBAL_SETTINGS: Mutex<Option<GlobalSettings>> = Mutex::new(None);

pub fn get_global_settings_locales_directory() -> PathBuf {

    let mut gs = GLOBAL_SETTINGS.lock().unwrap();

    if gs.as_ref().is_none() {
        *gs = get_global_settings();
    }

    if let Some(gs) = gs.as_ref() {
        return gs.locales_directory.clone();
    }
    panic!("Failed to load global settings to find locales directory");

}
