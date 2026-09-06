import { getCurrentWindow } from "@tauri-apps/api/window";

export async function showAppWindow() {
  const window = getCurrentWindow();
  await window.unminimize();
  await window.show();
  await window.setFocus();
}

export async function hideAppWindow() {
  await getCurrentWindow().hide();
}

export async function minimizeAppWindow() {
  await getCurrentWindow().minimize();
}

export async function toggleMaximizeAppWindow() {
  await getCurrentWindow().toggleMaximize();
}

export async function closeAppWindow() {
  await getCurrentWindow().close();
}
