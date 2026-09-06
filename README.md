# Markshot

Markshot is a small desktop screenshot tool for capturing, annotating, saving, and copying screenshots. It is built for a keyboard-driven workflow: launch it normally to open the editor, or run it with `--screenshot` to immediately start a capture flow.

The app has currently only been tested on Sway running Wayland. It may work on other Wayland compositors or desktop environments, but that has not been verified.

## Features

- Capture a screenshot from the desktop.
- Annotate screenshots with drawing tools.
- Save edited screenshots and copy them to the clipboard.
- Browse previously edited screenshots and copy them again.
- Supports a `--screenshot` startup flag for keybindings and launcher integrations.

## Tech Stack

- Tauri 2 for the desktop application shell.
- Rust for native screenshot, clipboard, history, startup, and Tauri command handling.
- Vue 3 with TypeScript for the frontend UI.
- Vite for frontend development and production builds.
- SCSS for styling, with shared tokens under `src/styles/`.
- pnpm for JavaScript package management.

## Project Structure

- `src/` contains the Vue frontend.
- `src/main.ts` mounts the app.
- `src/App.vue` is the root Vue component.
- `src/components/` contains the editor, toolbar, navbar, status, and history UI.
- `src/styles/` contains shared SCSS tokens and global styles.
- `src-tauri/` contains the Rust backend and Tauri configuration.
- `src-tauri/src/main.rs` starts the app and sets a WebKit environment fallback.
- `src-tauri/src/lib.rs` wires Tauri plugins, commands, and the `--screenshot` startup behavior.

## Requirements

- Rust and Cargo.
- Node.js v24.
- pnpm.
- Tauri system dependencies for Linux.
- A Wayland desktop environment. Sway is the only tested environment.

Optionally enable Corepack to manage pnpm:

```bash
corepack enable
```

Install JavaScript dependencies:

```bash
pnpm install
```

## Development

Start the Vite frontend only:

```bash
pnpm dev
```

Start the full Tauri desktop app:

```bash
pnpm tauri:dev
```

Run the development screenshot startup path:

```bash
pnpm screenshot:dev
```

The Vite dev server runs on port `1420`, which is the port Tauri expects during development.

## Building

Validate and build the frontend:

```bash
pnpm build
```

Build the release binary without creating a platform bundle:

```bash
pnpm build:binary
```

This project intentionally does not configure Tauri bundling targets such as Debian packages. The expected output is the release binary from Tauri/Cargo, not a `.deb` installer.

## Usage

Run the application normally:

```bash
markshot
```

Start directly in screenshot mode:

```bash
markshot --screenshot
```

For a window manager keybinding, use the absolute binary path if your compositor does not inherit your shell `PATH`. Example Sway binding:

```sway
bindsym Print exec /home/admin/.local/bin/markshot --screenshot
```

## Troubleshooting

If the application does not start, starts with a blank window, or fails when launched from a keybinding, try setting `WEBKIT_DISABLE_DMABUF_RENDERER=1`:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 markshot
```

For Sway keybindings, that can be written as:

```sway
bindsym Print exec env WEBKIT_DISABLE_DMABUF_RENDERER=1 /home/admin/.local/bin/markshot --screenshot
```

The app currently sets this variable automatically in Rust if it is missing, and the development scripts also set it explicitly. Depending on your system, you may need to add it, remove it, or test both variants.
