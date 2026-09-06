use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

pub const DEFAULT_MAX_HISTORY_FILES: usize = 15;

#[derive(Deserialize, Serialize)]
struct SettingsFile {
    screenshot_directory: PathBuf,
    #[serde(default = "default_max_history_files")]
    max_history_files: usize,
}

#[derive(Clone, Serialize)]
pub struct AppSettingsValue {
    pub screenshot_directory: PathBuf,
    pub max_history_files: usize,
}

pub struct AppSettings {
    screenshot_directory: Mutex<PathBuf>,
    max_history_files: Mutex<usize>,
}

impl AppSettings {
    pub fn load() -> Self {
        let settings = fs::read_to_string(settings_path())
            .ok()
            .and_then(|settings| serde_json::from_str::<SettingsFile>(&settings).ok());

        let screenshot_directory = settings
            .as_ref()
            .map(|settings| settings.screenshot_directory.clone())
            .unwrap_or_else(default_screenshot_directory);
        let max_history_files = settings
            .map(|settings| normalize_max_history_files(settings.max_history_files))
            .unwrap_or(DEFAULT_MAX_HISTORY_FILES);

        Self {
            screenshot_directory: Mutex::new(screenshot_directory),
            max_history_files: Mutex::new(max_history_files),
        }
    }

    pub fn value(&self) -> Result<AppSettingsValue, String> {
        Ok(AppSettingsValue {
            screenshot_directory: self.screenshot_directory()?,
            max_history_files: self.max_history_files()?,
        })
    }

    pub fn screenshot_directory(&self) -> Result<PathBuf, String> {
        self.screenshot_directory
            .lock()
            .map(|directory| directory.clone())
            .map_err(|_| "Failed to read settings".to_string())
    }

    pub fn max_history_files(&self) -> Result<usize, String> {
        self.max_history_files
            .lock()
            .map(|max_history_files| *max_history_files)
            .map_err(|_| "Failed to read settings".to_string())
    }

    pub fn set_value(
        &self,
        directory: PathBuf,
        max_history_files: usize,
    ) -> Result<AppSettingsValue, String> {
        if directory.as_os_str().is_empty() {
            return Err("Screenshot directory cannot be empty".to_string());
        }

        let max_history_files = normalize_max_history_files(max_history_files);

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

        save_settings(&directory, max_history_files)?;

        *self
            .screenshot_directory
            .lock()
            .map_err(|_| "Failed to update settings".to_string())? = directory.clone();
        *self
            .max_history_files
            .lock()
            .map_err(|_| "Failed to update settings".to_string())? = max_history_files;

        Ok(AppSettingsValue {
            screenshot_directory: directory,
            max_history_files,
        })
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

fn save_settings(screenshot_directory: &Path, max_history_files: usize) -> Result<(), String> {
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
        max_history_files,
    })
    .map_err(|error| format!("Failed to encode settings: {error}"))?;

    fs::write(&path, settings)
        .map_err(|error| format!("Failed to save settings {}: {error}", path.display()))
}

fn default_max_history_files() -> usize {
    DEFAULT_MAX_HISTORY_FILES
}

fn normalize_max_history_files(max_history_files: usize) -> usize {
    max_history_files.max(1)
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
