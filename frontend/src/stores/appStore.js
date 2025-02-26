import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const apiEntryPoint = ref(null)
  const serverError = ref("Lorem ipsum dolor sit amet consectetur adipisicing elit. Aliquid pariatur, ipsum dolor.")

  function setApiEntryPoint(newEntryPoint) {
    apiEntryPoint.value = newEntryPoint
  }

  return { apiEntryPoint, setApiEntryPoint, serverError }
})
