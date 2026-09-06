<template>
    <section class="history-grid" aria-label="Screenshot history">
        <div v-if="items.length === 0" class="empty-state">
            No edited screenshots saved yet.
        </div>

        <button
            v-for="item in items"
            :key="item.path"
            class="history-grid__card"
            type="button"
            @click="$emit('copy', item.path)"
        >
            <img :src="item.data_url" alt="Edited screenshot" />
            <span>{{ fileName(item.path) }}</span>
        </button>
    </section>
</template>

<script setup lang="ts">
import type { HistoryItem } from "../types/screenshot";
import { fileName } from "../utils/file";

defineProps<{
    items: HistoryItem[];
}>();

defineEmits<{
    copy: [path: string];
}>();
</script>

<style scoped lang="scss">
.history-grid {
    border: 1px solid $color-border-subtle;
    border-radius: $radius-panel;
    padding: $space-4;
    display: grid;
    grid-template-columns: repeat(
        auto-fill,
        minmax($size-history-card-min, 1fr)
    );
    gap: $space-4;
    align-content: start;
    background: $color-panel-background;

    &__card {
        border: 1px solid $color-border-subtle;
        border-radius: $radius-control;
        padding: $space-gap-md;
        display: grid;
        gap: $space-gap-sm;
        color: $color-text;
        background: $color-control-background;
        cursor: pointer;
        text-align: left;

        img {
            width: 100%;
            aspect-ratio: 4 / 3;
            object-fit: cover;
            border-radius: $radius-card;
        }

        span {
            overflow: hidden;
            color: $color-text-soft;
            font-size: $font-size-caption;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    }
}
</style>
