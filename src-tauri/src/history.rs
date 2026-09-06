use std::{
    env, fs,
    io::Cursor,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use image::ImageFormat;
use serde::Serialize;

use crate::screenshot::png_data_url;

pub const EDITED_SCREENSHOT_PREFIX: &str = "edited-screenshot";
pub const RAW_SCREENSHOT_PREFIX: &str = "screenshot";
pub const MAX_EDITED_SCREENSHOTS: usize = 15;
pub const MAX_RAW_SCREENSHOTS: usize = 3;

#[derive(Serialize)]
pub struct HistoryItem {
    path: String,
    data_url: String,
    created_at: u64,
}

pub struct ScreenshotFile {
    pub path: PathBuf,
    pub created_at: u64,
}

pub fn edited_screenshot_path() -> Result<PathBuf, String> {
    screenshot_path(EDITED_SCREENSHOT_PREFIX)
}

pub fn list_edited_screenshots() -> Result<Vec<HistoryItem>, String> {
    cleanup_known_screenshots()?;

    let mut items = screenshot_files(EDITED_SCREENSHOT_PREFIX)?;
    items.sort_by(|left, right| right.created_at.cmp(&left.created_at));
    items.truncate(MAX_EDITED_SCREENSHOTS);

    items
        .into_iter()
        .map(|file| {
            let image_bytes = fs::read(&file.path).map_err(|error| {
                format!("Failed to read screenshot {}: {error}", file.path.display())
            })?;
            let thumbnail = thumbnail_png(&image_bytes)?;

            Ok(HistoryItem {
                path: file.path.to_string_lossy().to_string(),
                data_url: png_data_url(&thumbnail),
                created_at: file.created_at,
            })
        })
        .collect()
}

pub fn cleanup_known_screenshots() -> Result<(), String> {
    cleanup_screenshots(EDITED_SCREENSHOT_PREFIX, MAX_EDITED_SCREENSHOTS)?;
    cleanup_screenshots(RAW_SCREENSHOT_PREFIX, MAX_RAW_SCREENSHOTS)
}

pub fn is_edited_screenshot_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .is_some_and(|file_name| {
            file_name.starts_with(&format!("{EDITED_SCREENSHOT_PREFIX}-"))
                && file_name.ends_with(".png")
        })
}

fn screenshot_path(prefix: &str) -> Result<PathBuf, String> {
    let mut directory = screenshot_directory();

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

    directory.push(format!("{prefix}-{timestamp}.png"));
    Ok(directory)
}

fn screenshot_directory() -> PathBuf {
    let mut directory = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);

    directory.push("Pictures");
    directory.push("Screenshots");
    directory
}

fn screenshot_files(prefix: &str) -> Result<Vec<ScreenshotFile>, String> {
    let directory = screenshot_directory();

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

fn cleanup_screenshots(prefix: &str, max_files: usize) -> Result<(), String> {
    let mut files = screenshot_files(prefix)?;
    files.sort_by(|left, right| right.created_at.cmp(&left.created_at));

    for file in files.into_iter().skip(max_files) {
        fs::remove_file(&file.path).map_err(|error| {
            format!(
                "Failed to delete old screenshot {}: {error}",
                file.path.display()
            )
        })?;
    }

    Ok(())
}

fn thumbnail_png(image_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let image = image::load_from_memory(image_bytes)
        .map_err(|error| format!("Failed to decode history thumbnail: {error}"))?;
    let thumbnail = image.thumbnail(420, 280);
    let mut thumbnail_bytes = Vec::new();

    thumbnail
        .write_to(&mut Cursor::new(&mut thumbnail_bytes), ImageFormat::Png)
        .map_err(|error| format!("Failed to encode history thumbnail: {error}"))?;

    Ok(thumbnail_bytes)
}
