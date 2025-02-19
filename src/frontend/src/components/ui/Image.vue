<script setup>
import { ref, onMounted, watch } from 'vue'

const props = defineProps({
  id: {
    type: [String, Number],
    required: true,
  },
})

const imageSrc = ref('')
const error = ref(null)

async function fetchImage(id) {
  try {
    const response = await fetch(`http://localhost:6004/files/${id}`)
    const result = await response.json()
    if (result.status !== 'success') {
      throw new Error('Failed to fetch image')
    }
    const imageData = result.data

    const byteArray = new Uint8Array(imageData.file_blob)
    const blob = new Blob([byteArray], { type: imageData.file_type })
    imageSrc.value = URL.createObjectURL(blob)
  } catch (err) {
    console.error(err)
    error.value = err
  }
}

onMounted(() => {
  fetchImage(props.id)
})

watch(
  () => props.id,
  (newId) => {
    if (newId) {
      fetchImage(newId)
    }
  }
)
</script>

<template>
  <div>
    <div v-if="error">
      <p>Error loading image: {{ error.message }}</p>
    </div>
    <div v-else-if="!imageSrc">
      <p>Loading image...</p>
    </div>
    <img v-else :src="imageSrc" alt="Fetched image" />
  </div>
</template>
