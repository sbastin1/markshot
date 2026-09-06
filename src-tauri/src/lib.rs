use std::{env, sync::Mutex};

use tauri::Emitter;

mod clipboard;
mod commands;
mod history;
mod screenshot;
mod startup;

use startup::{has_screenshot_arg, StartupScreenshotRequest};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let should_take_screenshot = has_screenshot_arg(&env::args().collect::<Vec<_>>());

    tauri::Builder::default()
        .manage(StartupScreenshotRequest(Mutex::new(should_take_screenshot)))
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if has_screenshot_arg(&args) {
                let _ = app.emit("take-screenshot-command", ());
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::take_screenshot,
            commands::save_and_copy_edited_screenshot,
            commands::list_edited_screenshots,
            commands::copy_screenshot_to_clipboard,
            commands::startup_should_take_screenshot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
