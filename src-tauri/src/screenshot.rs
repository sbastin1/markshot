use std::process::Command;

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

#[derive(Serialize)]
pub struct ScreenshotResult {
    data_url: String,
}

pub fn take_screenshot() -> Result<ScreenshotResult, String> {
    let slurp_output = Command::new("slurp")
        .output()
        .map_err(|error| format!("Failed to start slurp: {error}"))?;

    if !slurp_output.status.success() {
        let stderr = String::from_utf8_lossy(&slurp_output.stderr)
            .trim()
            .to_string();

        if stderr.is_empty() {
            return Err("Screenshot selection was cancelled".to_string());
        }

        return Err(format!("slurp failed: {stderr}"));
    }

    let geometry = String::from_utf8_lossy(&slurp_output.stdout)
        .trim()
        .to_string();

    if geometry.is_empty() {
        return Err("Screenshot selection was empty".to_string());
    }

    let grim_output = Command::new("grim")
        .args(["-g", geometry.as_str()])
        .arg("-")
        .output()
        .map_err(|error| format!("Failed to start grim: {error}"))?;

    if !grim_output.status.success() {
        let stderr = String::from_utf8_lossy(&grim_output.stderr)
            .trim()
            .to_string();

        if stderr.is_empty() {
            return Err("grim failed without an error message".to_string());
        }

        return Err(format!("grim failed: {stderr}"));
    }

    Ok(ScreenshotResult {
        data_url: png_data_url(&grim_output.stdout),
    })
}

pub fn decode_png_data_url(data_url: &str) -> Result<Vec<u8>, String> {
    let image_base64 = data_url
        .strip_prefix("data:image/png;base64,")
        .ok_or_else(|| "Edited image must be a PNG data URL".to_string())?;

    general_purpose::STANDARD
        .decode(image_base64)
        .map_err(|error| format!("Failed to decode edited image: {error}"))
}

pub fn png_data_url(image_bytes: &[u8]) -> String {
    let image_base64 = general_purpose::STANDARD.encode(image_bytes);

    format!("data:image/png;base64,{image_base64}")
}
