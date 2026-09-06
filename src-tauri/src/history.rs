use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

pub const EDITED_SCREENSHOT_PREFIX: &str = "edited-screenshot";
pub const RAW_SCREENSHOT_PREFIX: &str = "screenshot";
pub const MAX_RAW_SCREENSHOTS: usize = 3;

#[derive(Serialize)]
pub struct HistoryItem {
    path: String,
    created_at: u64,
}

pub struct ScreenshotFile {
    pub path: PathBuf,
    pub created_at: u64,
}

pub fn edited_screenshot_path(directory: &Path) -> Result<PathBuf, String> {
    screenshot_path(directory, EDITED_SCREENSHOT_PREFIX)
}

pub fn list_edited_screenshots(
    directory: &Path,
    max_history_files: usize,
) -> Result<Vec<HistoryItem>, String> {
    let files = cleanup_screenshots(directory, EDITED_SCREENSHOT_PREFIX, max_history_files)?;

    Ok(files
        .into_iter()
        .map(|file| HistoryItem {
            path: file.path.to_string_lossy().to_string(),
            created_at: file.created_at,
        })
        .collect())
}

pub fn cleanup_known_screenshots(directory: &Path, max_history_files: usize) -> Result<(), String> {
    cleanup_screenshots(directory, EDITED_SCREENSHOT_PREFIX, max_history_files)?;
    cleanup_screenshots(directory, RAW_SCREENSHOT_PREFIX, MAX_RAW_SCREENSHOTS)?;

    Ok(())
}

pub fn is_edited_screenshot_path(directory: &Path, path: &Path) -> bool {
    if !path.starts_with(directory) {
        return false;
    }

    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .is_some_and(|file_name| {
            file_name.starts_with(&format!("{EDITED_SCREENSHOT_PREFIX}-"))
                && file_name.ends_with(".png")
        })
}

fn screenshot_path(directory: &Path, prefix: &str) -> Result<PathBuf, String> {
    fs::create_dir_all(&directory).map_err(|error| {
        format!(
            "Failed to create screenshot directory {}: {error}",
            directory.display()
        )
    })?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("System clock error: {error}"))?
        .as_secs();

    Ok(directory.join(format!("{prefix}-{timestamp}.png")))
}

fn screenshot_files(directory: &Path, prefix: &str) -> Result<Vec<ScreenshotFile>, String> {
    if !directory.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&directory).map_err(|error| {
        format!(
            "Failed to read screenshot directory {}: {error}",
            directory.display()
        )
    })?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|error| format!("Failed to read screenshot entry: {error}"))?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
            continue;
        };

        if !file_name.starts_with(&format!("{prefix}-")) || !file_name.ends_with(".png") {
            continue;
        }

        let metadata = entry.metadata().map_err(|error| {
            format!(
                "Failed to read screenshot metadata {}: {error}",
                path.display()
            )
        })?;
        let created_at = metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        files.push(ScreenshotFile { path, created_at });
    }

    Ok(files)
}

fn cleanup_screenshots(
    directory: &Path,
    prefix: &str,
    max_files: usize,
) -> Result<Vec<ScreenshotFile>, String> {
    let mut files = screenshot_files(directory, prefix)?;
    files.sort_by(|left, right| right.created_at.cmp(&left.created_at));

    let old_files = files.split_off(max_files.min(files.len()));

    for file in old_files {
        fs::remove_file(&file.path).map_err(|error| {
            format!(
                "Failed to delete old screenshot {}: {error}",
                file.path.display()
            )
        })?;
    }

    Ok(files)
}
