import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router/router.js'
import { useAppStore } from '@/stores/appStore.js'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')

const { setApiEntryPoint } = useAppStore()
const entryPoint = import.meta.env.VITE_API_ENTRY_POINT || 'http://localhost:3000/api'
setApiEntryPoint(entryPoint)


