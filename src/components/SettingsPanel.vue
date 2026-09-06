<template>
    <section class="settings-panel" aria-label="Settings">
        <div class="settings-panel__header">
            <h1>Settings</h1>
            <p>Choose where edited screenshots are saved and how many stay in history.</p>
        </div>

        <form class="settings-panel__form" @submit.prevent="submitSettings">
            <label class="settings-panel__field">
                Screenshot save directory
                <input
                    v-model="directoryValue"
                    type="text"
                    placeholder="/home/user/Pictures/Screenshots"
                    autocomplete="off"
                />
            </label>

            <label class="settings-panel__field">
                Maximum history files
                <input
                    v-model="maxHistoryFilesValue"
                    type="number"
                    min="1"
                    step="1"
                    inputmode="numeric"
                    autocomplete="off"
                />
                <span class="settings-panel__hint">
                    Older edited screenshots are deleted when this maximum is exceeded.
                </span>
            </label>

            <div class="settings-panel__actions">
                <button class="primary-button" type="submit" :disabled="isSaving || !isHistoryLimitValid">
                    {{ isSaving ? "Saving..." : "Save Settings" }}
                </button>
            </div>
        </form>
    </section>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";

const props = defineProps<{
    directory: string;
    isSaving: boolean;
    maxHistoryFiles: number;
}>();

const emit = defineEmits<{
    save: [directory: string, maxHistoryFiles: number];
}>();

const directoryValue = ref(props.directory);
const maxHistoryFilesValue = ref(String(props.maxHistoryFiles));

const isHistoryLimitValid = computed(() => {
    const value = Number(maxHistoryFilesValue.value);

    return Number.isInteger(value) && value >= 1;
});

watch(
    () => props.directory,
    (directory) => {
        directoryValue.value = directory;
    },
);

watch(
    () => props.maxHistoryFiles,
    (maxHistoryFiles) => {
        maxHistoryFilesValue.value = String(maxHistoryFiles);
    },
);

function submitSettings() {
    const maxHistoryFiles = Number(maxHistoryFilesValue.value);

    if (!Number.isInteger(maxHistoryFiles) || maxHistoryFiles < 1) {
        return;
    }

    emit("save", directoryValue.value.trim(), maxHistoryFiles);
}
</script>

<style scoped lang="scss">
.settings-panel {
    border: 1px solid $color-border-subtle;
    border-radius: $radius-panel;
    padding: $space-4;
    display: grid;
    gap: $space-4;
    align-content: start;
    background: $color-panel-background;

    &__header {
        display: grid;
        gap: $space-gap-xs;

        h1,
        p {
            margin: 0;
        }

        h1 {
            font-size: 1.2rem;
        }

        p {
            color: $color-text-soft;
        }
    }

    &__form,
    &__field {
        display: grid;
        gap: $space-gap-md;
    }

    &__field {
        color: $color-text-medium;

        input {
            width: 100%;
            border: 1px solid $color-border-control;
            border-radius: $radius-control;
            padding: $space-button-block $space-4;
            color: $color-text;
            background: $color-control-background;
        }
    }

    &__hint {
        color: $color-text-soft;
        font-size: 0.9rem;
    }

    &__actions {
        display: flex;
        justify-content: flex-start;
    }
}
</style>
