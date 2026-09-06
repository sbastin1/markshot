use std::{fs, path::PathBuf};

use serde::Serialize;

use crate::{
    clipboard::copy_png_bytes_to_clipboard,
    history::{
        cleanup_known_screenshots, edited_screenshot_path, is_edited_screenshot_path,
        list_edited_screenshots as load_edited_screenshots, HistoryItem,
    },
    screenshot::{decode_png_data_url, take_screenshot as capture_screenshot, ScreenshotResult},
    settings::AppSettings,
    startup::StartupScreenshotRequest,
};

#[derive(Serialize)]
pub struct SaveResult {
    path: String,
    copied: bool,
    warning: Option<String>,
}

#[tauri::command]
pub fn take_screenshot() -> Result<ScreenshotResult, String> {
    capture_screenshot()
}

#[tauri::command]
pub fn save_and_copy_edited_screenshot(
    data_url: String,
    settings: tauri::State<AppSettings>,
) -> Result<SaveResult, String> {
    let image_bytes = decode_png_data_url(&data_url)?;
    let screenshot_directory = settings.screenshot_directory()?;
    let edited_path = edited_screenshot_path(&screenshot_directory)?;

    fs::write(&edited_path, &image_bytes).map_err(|error| {
        format!(
            "Failed to save edited screenshot {}: {error}",
            edited_path.display()
        )
    })?;

    let copy_result = copy_png_bytes_to_clipboard(&image_bytes);
    cleanup_known_screenshots(&screenshot_directory)?;

    Ok(SaveResult {
        path: edited_path.to_string_lossy().to_string(),
        copied: copy_result.is_ok(),
        warning: copy_result.err(),
    })
}

#[tauri::command]
pub fn list_edited_screenshots(
    settings: tauri::State<AppSettings>,
) -> Result<Vec<HistoryItem>, String> {
    load_edited_screenshots(&settings.screenshot_directory()?)
}

#[tauri::command]
pub fn copy_screenshot_to_clipboard(
    path: String,
    settings: tauri::State<AppSettings>,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    let screenshot_directory = settings.screenshot_directory()?;

    if !is_edited_screenshot_path(&screenshot_directory, &path) {
        return Err("Only edited screenshots can be copied from history".to_string());
    }

    let image_bytes = fs::read(&path)
        .map_err(|error| format!("Failed to read screenshot {}: {error}", path.display()))?;
    copy_png_bytes_to_clipboard(&image_bytes)
}

#[tauri::command]
pub fn get_screenshot_directory(settings: tauri::State<AppSettings>) -> Result<String, String> {
    Ok(settings
        .screenshot_directory()?
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
pub fn set_screenshot_directory(
    directory: String,
    settings: tauri::State<AppSettings>,
) -> Result<String, String> {
    Ok(settings
        .set_screenshot_directory(PathBuf::from(directory))?
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
pub fn startup_should_take_screenshot(
    request: tauri::State<StartupScreenshotRequest>,
) -> Result<bool, String> {
    let mut should_take_screenshot = request
        .0
        .lock()
        .map_err(|_| "Failed to read startup screenshot request".to_string())?;
    let value = *should_take_screenshot;
    *should_take_screenshot = false;

    Ok(value)
}
