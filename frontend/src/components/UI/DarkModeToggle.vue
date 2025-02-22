<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const DARK_MODE_LOCAL_STORAGE_KEY = 'dark-mode'
const colorSchemeMedia = window.matchMedia('(prefers-color-scheme: dark)')
const isDarkModeEnabled = ref(getInitialDarkModeState())

function getInitialDarkModeState() {
  const storedPreference = localStorage.getItem(DARK_MODE_LOCAL_STORAGE_KEY)
  return storedPreference === 'true' || (storedPreference === null && colorSchemeMedia.matches);
  
}

function setInitialPreference() {
  if (localStorage.getItem(DARK_MODE_LOCAL_STORAGE_KEY) === null) {
    localStorage.setItem(DARK_MODE_LOCAL_STORAGE_KEY, isDarkModeEnabled.value)
  }
}

function toggleColorScheme(state) {
  document.documentElement.classList.toggle('dark-mode', state)
  localStorage.setItem(DARK_MODE_LOCAL_STORAGE_KEY, state)
}

function onChangeColorScheme(event) {
  toggleColorScheme(event.matches)
}

onMounted(() => {
  setInitialPreference()
  toggleColorScheme(isDarkModeEnabled.value)
  colorSchemeMedia.addEventListener('change', onChangeColorScheme)
})

onUnmounted(() => {
  colorSchemeMedia.removeEventListener('change', onChangeColorScheme)
})
</script>

<template>
  <div class="dark-mode-toggle">
    <label class="switch" @click="toggleColorScheme(!isDarkModeEnabled)">
      <input type="checkbox" v-model="isDarkModeEnabled" />
      <span class="slider">
        <svg
          class="slider-icon"
          viewBox="0 0 24 24"
          fill="none"
          height="20"
          stroke="#000"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          width="20"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" />
        </svg>
      </span>
    </label>
  </div>
</template>

<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 34px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: lightblue;
  border-radius: 34px;
  transition: 0.6s;
}

.slider:before {
  position: absolute;
  content: '';
  height: 26px;
  width: 26px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  border-radius: 50%;
  transition: 0.4s;
}

input:checked + .slider {
  background-color: #363f72;
}

input:checked + .slider:before {
  transform: translateX(26px);
}

.slider-icon {
  transform: rotate(250deg);
  position: absolute;
  right: 6px;
  top: 7px;
}

@media (max-width: 768px) {
  .switch{
    margin-block: 0.6rem;  
  }
}

</style>
