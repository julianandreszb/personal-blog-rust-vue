<script setup>
import AppHeader from '@/components/Layout/AppHeader.vue'
import PostList from '@/components/Content/PostList.vue'
import HeroSection from '@/components/Content/HeroSection.vue'
import HorizontalTabs from '@/components/Layout/HorizontalTabs.vue'
</script>

<template>
  <app-header />
  <div class="section hero">
    <hero-section />
  </div>
  <div class="section posts">
    <horizontal-tabs />
    <post-list />
    <!--    <div class="section-posts">-->
    <!--&lt;!&ndash;      <side-bar />&ndash;&gt;-->
    <!--&lt;!&ndash;      <post-list />&ndash;&gt;-->
    <!--    </div>-->
  </div>
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

/*.section-posts {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  gap: var(--spacing-7xl);
  align-self: stretch;
}*/
</style>