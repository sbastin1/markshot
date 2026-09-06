use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SettingsFile {
    screenshot_directory: PathBuf,
}

pub struct AppSettings {
    screenshot_directory: Mutex<PathBuf>,
}

impl AppSettings {
    pub fn load() -> Self {
        let screenshot_directory = fs::read_to_string(settings_path())
            .ok()
            .and_then(|settings| serde_json::from_str::<SettingsFile>(&settings).ok())
            .map(|settings| settings.screenshot_directory)
            .unwrap_or_else(default_screenshot_directory);

        Self {
            screenshot_directory: Mutex::new(screenshot_directory),
        }
    }

    pub fn screenshot_directory(&self) -> Result<PathBuf, String> {
        self.screenshot_directory
            .lock()
            .map(|directory| directory.clone())
            .map_err(|_| "Failed to read settings".to_string())
    }

    pub fn set_screenshot_directory(&self, directory: PathBuf) -> Result<PathBuf, String> {
        if directory.as_os_str().is_empty() {
            return Err("Screenshot directory cannot be empty".to_string());
        }

        fs::create_dir_all(&directory).map_err(|error| {
            format!(
                "Failed to create screenshot directory {}: {error}",
                directory.display()
            )
        })?;

        let directory = directory.canonicalize().map_err(|error| {
            format!(
                "Failed to resolve screenshot directory {}: {error}",
                directory.display()
            )
        })?;

        save_settings(&directory)?;

        *self
            .screenshot_directory
            .lock()
            .map_err(|_| "Failed to update settings".to_string())? = directory.clone();

        Ok(directory)
    }
}

pub fn default_screenshot_directory() -> PathBuf {
    let mut directory = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);

    directory.push("Pictures");
    directory.push("Screenshots");
    directory
}

fn save_settings(screenshot_directory: &Path) -> Result<(), String> {
    let path = settings_path();

    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(|error| {
            format!(
                "Failed to create settings directory {}: {error}",
                directory.display()
            )
        })?;
    }

    let settings = serde_json::to_string_pretty(&SettingsFile {
        screenshot_directory: screenshot_directory.to_path_buf(),
    })
    .map_err(|error| format!("Failed to encode settings: {error}"))?;

    fs::write(&path, settings)
        .map_err(|error| format!("Failed to save settings {}: {error}", path.display()))
}

fn settings_path() -> PathBuf {
    let mut directory = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("HOME").map(|home| {
                let mut directory = PathBuf::from(home);
                directory.push(".config");
                directory
            })
        })
        .unwrap_or_else(env::temp_dir);

    directory.push("markshot");
    directory.push("settings.json");
    directory
}
