import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import type { AppSettings, HistoryItem, SaveResult, ScreenshotResult } from "../types/screenshot";

type HistoryItemResponse = Omit<HistoryItem, "image_url">;

export function takeScreenshot() {
  return invoke<ScreenshotResult>("take_screenshot");
}

export function saveAndCopyEditedScreenshot(dataUrl: string) {
  return invoke<SaveResult>("save_and_copy_edited_screenshot", { dataUrl });
}

export function listEditedScreenshots() {
  return invoke<HistoryItemResponse[]>("list_edited_screenshots").then((items) =>
    items.map((item) => ({
      ...item,
      image_url: convertFileSrc(item.path),
    })),
  );
}

export function copyScreenshotToClipboard(path: string) {
  return invoke("copy_screenshot_to_clipboard", { path });
}

export function getSettings() {
  return invoke<AppSettings>("get_settings");
}

export function setSettings(settings: AppSettings) {
  return invoke<AppSettings>("set_settings", {
    screenshotDirectory: settings.screenshot_directory,
    maxHistoryFiles: settings.max_history_files,
  });
}

export function startupShouldTakeScreenshot() {
  return invoke<boolean>("startup_should_take_screenshot");
}
