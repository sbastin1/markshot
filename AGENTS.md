# AGENTS.md

## Project Shape
- Single-package Tauri 2 app with a Vue 3 + TypeScript + Vite frontend in `src/` and Rust backend in `src-tauri/`.
- Frontend entrypoint is `src/main.ts`; the root Vue component is `src/App.vue`.
- Rust entrypoint is `src-tauri/src/main.rs`, which calls `tauri_app_lib::run()` from `src-tauri/src/lib.rs`.

## Commands
- Use `pnpm`, not `npm`; `pnpm-lock.yaml` is the lockfile and Tauri config invokes `pnpm`.
- Frontend dev server: `pnpm dev`.
- Start the desktop app with `pnpm tauri:dev`, not `pnpm tauri dev`; the script sets `WEBKIT_DISABLE_DMABUF_RENDERER=1` before `tauri dev`.
- Production frontend validation/build: `pnpm build`; this runs `vue-tsc --noEmit` before `vite build`.
- Build the release binary without bundling: `pnpm build:binary`; this runs `tauri build --no-bundle`.
- Screenshot startup path for development: `pnpm screenshot:dev`; the app also accepts `--screenshot` in release builds.
- Tauri command passthrough: `pnpm tauri`.
- There are no configured lint or test scripts in `package.json`.

## Toolchain Notes
- Vite dev server is fixed to port `1420` with `strictPort: true`; Tauri expects `http://localhost:1420`.
- `src-tauri/tauri.conf.json` runs `pnpm dev` before dev and `pnpm build` before packaging; frontend output is `dist/`.
- Tauri bundling is intentionally not configured; do not re-add Debian/deb bundle targets unless explicitly requested.
- Keep `src-tauri/icons/icon.png`: `tauri::generate_context!()` requires it even for `tauri build --no-bundle`.
- Vite ignores file watching under `src-tauri/`, so Rust-side changes are handled by Tauri/Cargo, not Vite.
- TypeScript is strict and fails on unused locals/parameters via `tsconfig.json`.

## Styling
- SCSS tokens live in `src/styles/{colors,spacing,radius,typography,sizes,effects,breakpoints}.scss` and are injected globally into every SCSS block by `vite.config.ts`.
- In Vue SFC styles, use existing token variables directly; do not add local token imports unless the Vite SCSS injection changes.
- Component-scoped classes follow component-owned BEM-style names, e.g. `editor-toolbar__tool` and `status-message--error`; keep shared `.primary-button`/`.ghost-button` global.

## Generated And Build Artifacts
- Do not edit `dist/`, `target/`, `src-tauri/target/`, or generated schemas under `src-tauri/gen/schemas/` unless a task explicitly targets generated output.
