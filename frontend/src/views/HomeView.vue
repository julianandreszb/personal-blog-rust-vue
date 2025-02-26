<script setup>
import AppHeader from '@/components/Layout/AppHeader.vue'
import PostList from '@/components/Content/PostList.vue'
import HeroSection from '@/components/Content/HeroSection.vue'
/*import HorizontalTabs from '@/components/Layout/HorizontalTabs.vue'*/
import SideBar from '@/components/Layout/SideBar.vue'

import { useAppStore } from '@/stores/appStore.js'
import AlertMessage from '@/components/UI/AlertMessage.vue'

const appStore = useAppStore()
</script>

<template>
  <app-header />
  <div class="section hero">
    <hero-section />
  </div>
  <div class="section posts">
    <div class="section-posts">
      <side-bar class="sidebar" />
      <post-list />
    </div>
  </div>

  <alert-message v-if="appStore.serverError">
    <template #title>
      <h2>Server Error</h2>
    </template>
    <template #content>{{ appStore.serverError }}</template>
  </alert-message>
</template>

<style lang="scss" scoped>
.section {
  display: flex;
  padding-bottom: var(--spacing-9xl);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-7xl);
  align-self: stretch;
  margin-inline: var(--spacing-3xl);

  &.hero {
    padding: var(--spacing-9xl) 0;
  }

  &.posts {
    align-items: normal;
    gap: var(--spacing-4xl);
  }
}

.section-posts {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  gap: var(--spacing-7xl);
  align-self: stretch;
}

.sidebar {
  position: sticky;
  top: 7.2rem;
}
</style>