<template>
    <div class="editor-toolbar">
        <div class="editor-toolbar__tools">
            <div class="editor-toolbar__tool-group" aria-label="Drawing tools">
                <button
                    v-for="tool in annotationTools"
                    :key="tool.value"
                    class="editor-toolbar__tool"
                    :class="{
                        'editor-toolbar__tool--active':
                            selectedTool === tool.value,
                    }"
                    type="button"
                    @click="$emit('update:selectedTool', tool.value)"
                >
                    {{ tool.label }}
                </button>
            </div>

            <label class="editor-toolbar__control">
                Color
                <input
                    :value="selectedColor"
                    type="color"
                    @input="updateColor"
                />
            </label>

            <label class="editor-toolbar__control">
                Width
                <input
                    :value="strokeWidth"
                    type="range"
                    min="2"
                    max="18"
                    @input="updateStrokeWidth"
                />
                <span>{{ strokeWidth }}</span>
            </label>
        </div>
        <div class="editor-toolbar__actions">
            <button
                class="ghost-button"
                type="button"
                :disabled="!canUndo"
                @click="$emit('undo')"
            >
                Undo
            </button>

            <button
                class="primary-button"
                type="button"
                :disabled="isSaving"
                @click="$emit('save')"
            >
                {{ isSaving ? "Saving..." : "Save + Copy" }}
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { annotationTools, type Tool } from "../types/annotation";

defineProps<{
    canUndo: boolean;
    isSaving: boolean;
    selectedColor: string;
    selectedTool: Tool;
    strokeWidth: number;
}>();

const emit = defineEmits<{
    save: [];
    undo: [];
    "update:selectedColor": [color: string];
    "update:selectedTool": [tool: Tool];
    "update:strokeWidth": [width: number];
}>();

function updateColor(event: Event) {
    emit("update:selectedColor", (event.target as HTMLInputElement).value);
}

function updateStrokeWidth(event: Event) {
    emit(
        "update:strokeWidth",
        Number((event.target as HTMLInputElement).value),
    );
}
</script>

<style scoped lang="scss">
.editor-toolbar,
.editor-toolbar__tool-group,
.editor-toolbar__control {
    display: flex;
    align-items: center;
}

.editor-toolbar {
    gap: $space-gap-md;
    flex-wrap: wrap;
    justify-content: space-between;

    &__tool {
        border: 1px solid $color-border-control;
        border-radius: $radius-pill;
        padding: $space-button-block $space-4;
        color: $color-text;
        background: $color-control-background;
        cursor: pointer;
        font-weight: $font-weight-strong;

        &--active {
            border-color: $color-primary-strong;
            background: $color-primary-soft;
        }
    }

    &__tools {
        display: flex;
        gap: $space-gap-lg;
    }

    &__tool-group {
        gap: $space-gap-xs;
    }

    &__actions {
        display: flex;
        gap: $space-gap-md;
    }

    &__control {
        gap: $space-gap-sm;
        color: $color-text-medium;
        font-weight: $font-weight-strong;

        input[type="color"] {
            width: $size-color-input-width;
            height: $size-color-input-height;
            border: 0;
            padding: 0;
            background: transparent;
        }

        input[type="range"] {
            accent-color: $color-primary;
        }
    }
}

@media (max-width: $breakpoint-controls) {
    .editor-toolbar {
        align-items: stretch;
        flex-direction: column;

        &__tool-group {
            width: 100%;
        }

        &__tools {
            flex-direction: column;
        }
    }

    .primary-button,
    .ghost-button,
    .editor-toolbar__tool {
        flex: 1;
        max-height: $size-control-height;
    }
}
</style>
