import { ref, watchEffect, toValue } from 'vue'

export function useFetch(url: any, method: string, payload: any, post: () => void | undefined) {
  const data = ref(null)
  const error = ref(null)

  const fetchData = () => {
    // reset state before fetching..
    data.value = null
    error.value = null

    const options = {
      method: method,
      headers: method === "POST" ? {
          'Content-Type': 'application/json',
      } : undefined,
      body: payload === undefined ? JSON.stringify(payload) : undefined
    }

    fetch(toValue(url), options)
      .then((res) => res.json())
      .then((json) => {
        data.value = json
        if (post) post()
      })
      .catch((err) => (error.value = err))
  }

  watchEffect(() => {
    fetchData()
  })

  return { data, error }
}