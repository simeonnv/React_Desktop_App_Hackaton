<script setup>
import { ref, onMounted, watch } from 'vue'
import { useFetch } from '../../util/useFetch'

const props = defineProps({
  id: {
    type: [String, Number],
    required: true,
  },
})

const imageSrc = ref('')
const { data, error } = useFetch(() => `http://localhost:6004/files/${props.id}`, "GET", undefined,  () => {
    const byteArray = new Uint8Array(data.value.data.file_blob)
    console.log(byteArray)
    const blob = new Blob([byteArray], { type: data.value.data.file_type })
    console.log(blob)
    imageSrc.value = URL.createObjectURL(blob)
    
})

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
