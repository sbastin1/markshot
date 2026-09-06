<template>
    <nav class="app-navbar">
        <div class="app-navbar__brand">
            <img :src="markshotLogo" alt="Markshot" />
            <span>Markshot</span>
        </div>

        <div class="app-navbar__actions">
            <button
                class="primary-button"
                type="button"
                :disabled="isCapturing"
                @click="$emit('capture')"
            >
                {{ isCapturing ? "Selecting..." : "Screenshot" }}
            </button>
            <button
                class="ghost-button"
                type="button"
                @click="$emit('history')"
            >
                History
            </button>
        </div>
    </nav>
</template>

<script setup lang="ts">
import markshotLogo from "../assets/markshot-logo.svg";

defineProps<{
    isCapturing: boolean;
}>();

defineEmits<{
    capture: [];
    history: [];
}>();
</script>

<style scoped lang="scss">
.app-navbar {
    display: flex;
    justify-content: space-between;
    gap: $space-4;
    align-items: center;
    border: 1px solid $color-border-subtle;
    border-radius: $radius-navbar;
    padding: $space-3;
    background: $color-control-background;

    &__brand,
    &__actions {
        display: flex;
        align-items: center;
    }

    &__brand {
        gap: $space-gap-brand;
        font-weight: $font-weight-brand;
        letter-spacing: $letter-spacing-brand;

        img {
            width: $size-control-height;
            height: $size-control-height;
        }
    }

    &__actions {
        gap: $space-gap-md;
        flex-wrap: wrap;
    }
}

@media (max-width: $breakpoint-controls) {
    .app-navbar {
        align-items: stretch;
        flex-direction: column;

        &__actions {
            width: 100%;
        }
    }

    .primary-button,
    .ghost-button {
        flex: 1;
        max-height: $size-control-height;
    }
}
</style>
