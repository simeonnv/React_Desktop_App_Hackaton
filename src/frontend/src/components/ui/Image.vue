<template>
    <div class="image-display">
        <div v-if="isLoading" class="loading">Loading image...</div>
        <div v-else-if="error" class="error">Error: {{ error }}</div>
        <img v-else-if="imageUrl" :src="imageUrl" alt="Fetched image" class="image-preview" />
        <div v-else class="no-image">No image to display</div>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';

interface Props {
  id?: number | null;
}

const props = defineProps<Props>();

const imageUrl = ref('');
const isLoading = ref(false);
const error = ref('');

const fetchImage = async () => {
    if (!props.id) {
        imageUrl.value = '';
        return;
    }

    try {
        isLoading.value = true;
        error.value = '';

        const response = await fetch(`http://localhost:8080/images/${props.id}`);

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const blob = await response.blob();
        imageUrl.value = URL.createObjectURL(blob);

    } catch (err: any) {
        error.value = err.message;
        imageUrl.value = '';
    } finally {
        isLoading.value = false;
    }
};

watch(() => props.id, (newId) => {
    if (newId) fetchImage();
}, { immediate: true });

import { onUnmounted } from 'vue';
onUnmounted(() => {
    if (imageUrl.value) {
        URL.revokeObjectURL(imageUrl.value);
    }
});
</script>

<style>
.image-display {
    max-width: 100%;
    margin: 1rem 0;
}

.image-preview {
    max-width: 100%;
    height: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.loading {
    color: #666;
    padding: 1rem;
}

.error {
    color: #dc3545;
    padding: 1rem;
}

.no-image {
    color: #6c757d;
    padding: 1rem;
}
</style>