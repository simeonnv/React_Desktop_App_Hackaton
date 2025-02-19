<template>
    <div class="image-upload">
        <input type="file" accept="image/*" @change="handleFileUpload" ref="fileInput" />
        <div v-if="previewUrl" class="preview-container">
            <img :src="previewUrl" alt="Preview" class="preview-image" />
        </div>
        <div v-if="uploadError" class="error-message">{{ uploadError }}</div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

const props = defineProps({
    modelValue: String
});

const emit = defineEmits(['update:modelValue']);

const fileInput: any = ref(null);
const previewUrl = ref('');
const uploadError = ref('');

const handleFileUpload = async (event: any) => {
    const file = event.target.files[0];

    if (!file) return;

    if (!file.type.startsWith('image/')) {
        uploadError.value = 'Please upload an image file';
        return;
    }

    if (file.size > 5 * 1024 * 1024) {
        uploadError.value = 'File size must be less than 5MB';
        return;
    }

    // Show preview (same as before)
    const reader = new FileReader();
    reader.onload = (e: any) => {
        previewUrl.value = e.target.result;
    };
    reader.readAsDataURL(file);

    try {
        const arrayBuffer = await file.arrayBuffer();
        const byteArray = new Uint8Array(arrayBuffer);

        const payload = {
            file_blob: [...byteArray]
        };

        const response = await fetch('http://localhost:6004/files', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorData = await response.json();
            throw new Error(errorData.message || 'Upload failed');
        }

        const data = await response.json();
        emit('update:modelValue', data.id);
        uploadError.value = '';
        fileInput.value.value = '';

    } catch (err: any) {
        uploadError.value = err.message || 'Failed to upload image';
        console.error('Upload error:', err);
    }
};
</script>

<!-- Keep the same styles as previous version -->