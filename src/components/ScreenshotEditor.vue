<template>
    <section class="screenshot-editor" aria-label="Screenshot editor">
        <EditorToolbar
            v-model:selected-color="selectedColor"
            v-model:selected-tool="selectedTool"
            v-model:stroke-width="strokeWidth"
            :can-undo="annotations.length > 0"
            :is-saving="isSaving"
            @save="saveCanvas"
            @undo="undoAnnotation"
        />

        <div class="screenshot-editor__viewport">
            <canvas
                ref="canvasElement"
                class="screenshot-editor__canvas"
                @pointerdown="startAnnotation"
                @pointermove="updateAnnotation"
                @pointerup="finishAnnotation"
                @pointercancel="cancelAnnotation"
                @pointerleave="finishAnnotation"
            ></canvas>
        </div>
    </section>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { useScreenshotCanvas } from "../composables/useScreenshotCanvas";
import type { Tool } from "../types/annotation";
import EditorToolbar from "./EditorToolbar.vue";

const props = defineProps<{
    isSaving: boolean;
    screenshotUrl: string;
}>();

const emit = defineEmits<{
    error: [message: string];
    save: [dataUrl: string];
}>();

const selectedTool = ref<Tool>("rectangle");
const selectedColor = ref("#da0037");
const strokeWidth = ref(4);

const {
    annotations,
    canvasDataUrl,
    canvasElement,
    cancelAnnotation,
    finishAnnotation,
    loadScreenshot,
    startAnnotation,
    undoAnnotation,
    updateAnnotation,
} = useScreenshotCanvas({
    selectedColor,
    selectedTool,
    strokeWidth,
});

watch(
    () => props.screenshotUrl,
    async (screenshotUrl) => {
        if (!screenshotUrl) {
            return;
        }

        try {
            await loadScreenshot(screenshotUrl);
        } catch (error) {
            emit(
                "error",
                error instanceof Error ? error.message : String(error),
            );
        }
    },
    { immediate: true },
);

function saveCanvas() {
    if (!canvasElement.value) {
        emit("error", "No screenshot is loaded");
        return;
    }

    const dataUrl = canvasDataUrl();

    if (!dataUrl) {
        emit("error", "No screenshot is loaded");
        return;
    }

    emit("save", dataUrl);
}
</script>

<style scoped lang="scss">
.screenshot-editor {
    min-height: 0;
    width: fit-content;
    min-width: min(100%, $size-editor-min-width);
    max-width: 100%;
    justify-self: center;
    border: 1px solid $color-border-subtle;
    border-radius: $radius-panel;
    padding: $space-4;
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    gap: $space-gap-md;
    background: $color-panel-background;

    &__viewport {
        min-height: 0;
        overflow: auto;
        background: $color-canvas-background;
    }

    &__canvas {
        display: block;
        height: auto;
        cursor: crosshair;
        touch-action: none;
    }
}
</style>
