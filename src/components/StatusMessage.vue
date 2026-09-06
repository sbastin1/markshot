<template>
    <p
        v-if="message"
        class="status-message"
        :class="{ 'status-message--error': isError }"
    >
        {{ message }}
    </p>
</template>

<script setup lang="ts">
defineProps<{
    isError: boolean;
    message: string;
}>();
</script>

<style scoped lang="scss">
.status-message {
    min-height: $size-status-height;
    margin: 0;
    color: $color-text-soft;
    overflow-wrap: anywhere;

    &--error {
        color: $color-danger;
    }
}
</style>
