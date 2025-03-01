<script setup>
import AppHeader from '@/components/Layout/AppHeader.vue'
import PostList from '@/components/Content/PostList.vue'
import HeroSection from '@/components/Content/HeroSection.vue'
import { useAppStore } from '@/stores/appStore.js'
import AlertMessage from '@/components/UI/AlertMessage.vue'
import CategoryTabs from '@/components/Layout/CategoryTabs.vue'

const appStore = useAppStore()
</script>

<template>
  <app-header />
  <div class="section hero">
    <hero-section />
  </div>
  <div class="section posts">
    <div class="section-posts">
      <category-tabs class="category-filter"/>
      <post-list />
    </div>
  </div>

  <alert-message v-if="appStore.serverError">
    <template #title>There was a problem with that action</template>
    {{ appStore.serverError }}
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
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
  gap: var(--spacing-3xl);
  align-self: stretch;
}

.category-filter {
  width: 100%;
  overflow-x: auto;
  position: sticky;
  top: 4.9rem;
  padding-block: 1rem;
  background-color: var(--Colors-Background-bg-primary);
}

@media screen and (max-width: 768px) {
  .section-posts{
    flex-direction: column;
  }
}

</style>