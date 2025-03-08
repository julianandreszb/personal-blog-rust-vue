<script setup>
import AppHeader from '@/components/Layout/AppHeader.vue'
import AlertMessage from '@/components/UI/AlertMessage.vue'
import { useAppStore } from '@/stores/appStore.js'

const appStore = useAppStore()
</script>

<template>
  <app-header />
  
  <router-view />
  <alert-message v-if="appStore.serverError">
    <template #title>There was a problem with that action</template>
    {{ appStore.serverError }}
  </alert-message>
</template>

<style lang="scss" scoped></style>