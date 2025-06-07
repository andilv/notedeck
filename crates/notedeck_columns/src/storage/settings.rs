use serde_json;
use std::fs;
use tracing::{error, info};

use notedeck::{DataPath, DataPathType};
use notedeck_ui::NoteOptions;

static SETTINGS_FILE: &str = "settings.json";

pub fn load_note_options(path: &DataPath) -> Option<NoteOptions> {
    let settings_path = path.path(DataPathType::Setting).join(SETTINGS_FILE);
    if !settings_path.exists() {
        return None;
    }

    match fs::read_to_string(&settings_path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(options) => {
                info!("Loaded settings from {}", settings_path.display());
                Some(options)
            }
            Err(e) => {
                error!(
                    "Failed to parse settings file {}: {}",
                    settings_path.display(),
                    e
                );
                None
            }
        },
        Err(e) => {
            error!(
                "Failed to read settings file {}: {}",
                settings_path.display(),
                e
            );
            None
        }
    }
}

pub fn save_note_options(path: &DataPath, note_options: &NoteOptions) {
    let settings_path = path.path(DataPathType::Setting).join(SETTINGS_FILE);
    match serde_json::to_string_pretty(note_options) {
        Ok(json_string) => {
            if let Err(e) = notedeck::storage::write_file(&path.path(DataPathType::Setting), SETTINGS_FILE.to_string(), &json_string) {
                error!(
                    "Failed to write settings to {}: {}",
                    settings_path.display(),
                    e
                );
            } else {
                info!("Saved settings to {}", settings_path.display());
            }
        }
        Err(e) => {
            error!("Failed to serialize NoteOptions: {}", e);
        }
    }
}
