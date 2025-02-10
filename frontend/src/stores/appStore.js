import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const apiEntryPoint = ref(null)

  function setApiEntryPoint(newEntryPoint) {
    apiEntryPoint.value = newEntryPoint
  }

  return { apiEntryPoint, setApiEntryPoint }
})
