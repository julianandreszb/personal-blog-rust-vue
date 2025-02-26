import { createFetch } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useAppStore } from '@/stores/appStore.js'

export const useFetchApi = () => {
  const appStore = useAppStore()
  const { apiEntryPoint } = storeToRefs(appStore)

  const fetchApi = createFetch({
    baseUrl: apiEntryPoint.value,
    combination: 'chaining',
    options: {
      updateDataOnError: true, // Required to ensure data returns the error JSON from backend
      async onFetchError(response) {
        console.log(response.data)
        return response
      }
    },
    fetchOptions: {
      mode: 'cors'
    }
  })

  return {
    fetchApi
  }
}
