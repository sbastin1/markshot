<template>
  <div class="app-frame">
    <header class="titlebar" data-tauri-drag-region>
      <div class="titlebar__brand" data-tauri-drag-region>
        <span data-tauri-drag-region>Markshot</span>
      </div>

      <div class="titlebar__controls">
        <button class="titlebar__button" type="button" aria-label="Minimize" @click="minimizeAppWindow">&minus;</button>
        <button class="titlebar__button titlebar__button--maximize" type="button" aria-label="Maximize" @click="toggleMaximizeAppWindow">&#9633;</button>
        <button class="titlebar__button titlebar__button--close" type="button" aria-label="Close" @click="closeAppWindow">&times;</button>
      </div>
    </header>

    <main class="app-shell">
      <AppNavbar :is-capturing="isCapturing" @capture="captureScreenshot" @history="showHistory" @settings="showSettings" />

      <StatusMessage :is-error="hasError" :message="statusMessage" />

      <HistoryGrid v-if="view === 'history'" :items="historyItems" @copy="copyHistoryItem" />

      <SettingsPanel
        v-else-if="view === 'settings'"
        :directory="screenshotDirectory"
        :is-saving="isSavingSettings"
        @save="saveSettings"
      />

      <ScreenshotEditor
        v-else-if="screenshotUrl"
        :is-saving="isSaving"
        :screenshot-url="screenshotUrl"
        @error="(message) => setStatus(message, true)"
        @save="saveAndCopyScreenshot"
      />

      <section v-else class="empty-state empty-state--large">
        Take a screenshot to start editing.
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted, onUnmounted, ref } from "vue";
import AppNavbar from "./components/AppNavbar.vue";
import HistoryGrid from "./components/HistoryGrid.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import ScreenshotEditor from "./components/ScreenshotEditor.vue";
import StatusMessage from "./components/StatusMessage.vue";
import type { HistoryItem } from "./types/screenshot";
import { errorMessage } from "./utils/errors";
import { fileName } from "./utils/file";
import {
  copyScreenshotToClipboard,
  getScreenshotDirectory,
  listEditedScreenshots,
  saveAndCopyEditedScreenshot,
  setScreenshotDirectory,
  startupShouldTakeScreenshot,
  takeScreenshot,
} from "./utils/tauriCommands";
import {
  closeAppWindow,
  hideAppWindow,
  minimizeAppWindow,
  showAppWindow,
  toggleMaximizeAppWindow,
} from "./utils/window";

const view = ref<"editor" | "history" | "settings">("editor");
const isCapturing = ref(false);
const isSaving = ref(false);
const isSavingSettings = ref(false);
const statusMessage = ref("");
const hasError = ref(false);
const screenshotUrl = ref("");
const screenshotDirectory = ref("");
const historyItems = ref<HistoryItem[]>([]);

let unlistenScreenshotCommand: (() => void) | null = null;

onMounted(async () => {
  unlistenScreenshotCommand = await listen("take-screenshot-command", () => {
    void captureScreenshot({ showAfterCapture: true });
  });

  try {
    const shouldTakeScreenshot = await startupShouldTakeScreenshot();

    if (shouldTakeScreenshot) {
      await captureScreenshot({ showAfterCapture: true });
    } else {
      await showAppWindow();
    }
  } catch (error) {
    await showAppWindow();
    setStatus(errorMessage(error), true);
  }
});

onUnmounted(() => {
  unlistenScreenshotCommand?.();
});

async function captureScreenshot(options: { showAfterCapture?: boolean } = {}) {
  if (isCapturing.value) {
    return;
  }

  view.value = "editor";
  isCapturing.value = true;
  setStatus("", false);

  try {
    if (options.showAfterCapture) {
      await hideAppWindow();
    }

    const screenshot = await takeScreenshot();
    screenshotUrl.value = screenshot.data_url;

    if (options.showAfterCapture) {
      await showAppWindow();
    }

    setStatus("Screenshot loaded. Add annotations, then save + copy.", false);
  } catch (error) {
    if (options.showAfterCapture) {
      await showAppWindow();
    }

    setStatus(errorMessage(error), true);
  } finally {
    isCapturing.value = false;
  }
}

async function showHistory() {
  view.value = "history";
  setStatus("", false);

  try {
    historyItems.value = await listEditedScreenshots();
  } catch (error) {
    setStatus(errorMessage(error), true);
  }
}

async function showSettings() {
  view.value = "settings";
  setStatus("", false);

  try {
    screenshotDirectory.value = await getScreenshotDirectory();
  } catch (error) {
    setStatus(errorMessage(error), true);
  }
}

async function saveSettings(directory: string) {
  isSavingSettings.value = true;
  setStatus("", false);

  try {
    screenshotDirectory.value = await setScreenshotDirectory(directory);
    historyItems.value = [];
    setStatus("Screenshot save directory updated.", false);
  } catch (error) {
    setStatus(errorMessage(error), true);
  } finally {
    isSavingSettings.value = false;
  }
}

async function copyHistoryItem(path: string) {
  try {
    await copyScreenshotToClipboard(path);
    setStatus(`Copied ${fileName(path)} to clipboard.`, false);
  } catch (error) {
    setStatus(errorMessage(error), true);
  }
}

async function saveAndCopyScreenshot(dataUrl: string) {
  isSaving.value = true;
  setStatus("", false);

  try {
    const result = await saveAndCopyEditedScreenshot(dataUrl);

    if (result.copied) {
      setStatus(`Saved and copied ${fileName(result.path)}.`, false);
    } else {
      setStatus(`Saved ${fileName(result.path)}, but clipboard copy failed: ${result.warning}`, true);
    }
  } catch (error) {
    setStatus(errorMessage(error), true);
  } finally {
    isSaving.value = false;
  }
}

function setStatus(message: string, isError: boolean) {
  statusMessage.value = message;
  hasError.value = isError;
}
</script>
