import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
// @ts-expect-error type error without @types/node package
import process from "node:process";
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [vue()],
  css: {
    preprocessorOptions: {
      scss: {
        loadPaths: ["src"],
        additionalData:
          '@use "styles/colors.scss" as *; @use "styles/spacing.scss" as *; @use "styles/radius.scss" as *; @use "styles/typography.scss" as *; @use "styles/sizes.scss" as *; @use "styles/effects.scss" as *; @use "styles/breakpoints.scss" as *;',
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
