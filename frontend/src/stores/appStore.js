import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const apiEntryPoint = ref(null)
  const serverError = ref("Server error found")

  function setApiEntryPoint(newEntryPoint) {
    apiEntryPoint.value = newEntryPoint
  }

  return { apiEntryPoint, setApiEntryPoint, serverError }
})
