<template>
    <nav class="app-navbar">
        <div class="app-navbar__brand">
            <img :src="markshotLogo" alt="Markshot" />
            <span>Markshot</span>
        </div>

        <div class="app-navbar__actions">
            <button
                v-if="canReturnToEditor"
                class="ghost-button app-navbar__icon-button app-navbar__icon-button--back"
                type="button"
                aria-label="Return to editor"
                title="Return to editor"
                @click="$emit('editor')"
            >
                <span aria-hidden="true"></span>
            </button>
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
            <button
                class="ghost-button app-navbar__icon-button app-navbar__icon-button--settings"
                type="button"
                aria-label="Settings"
                title="Settings"
                @click="$emit('settings')"
            >
                <span aria-hidden="true"></span>
            </button>
        </div>
    </nav>
</template>

<script setup lang="ts">
import markshotLogo from "../assets/markshot-logo.svg";

defineProps<{
    canReturnToEditor: boolean;
    isCapturing: boolean;
}>();

defineEmits<{
    capture: [];
    editor: [];
    history: [];
    settings: [];
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

    &__icon-button {
        width: $size-control-height;
        height: $size-control-height;
        padding: 0;
        display: grid;
        place-items: center;

        span {
            width: 1.2rem;
            height: 1.2rem;
            background: currentColor;
        }

        &--back span {
            mask: url("../assets/arrow-left-icon.svg") center / contain no-repeat;
        }

        &--settings span {
            mask: url("../assets/settings-icon.svg") center / contain no-repeat;
        }
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
