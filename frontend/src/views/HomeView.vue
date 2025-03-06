<script setup>
import { computed, onMounted, ref, watchEffect } from 'vue'
import AppHeader from '@/components/Layout/AppHeader.vue'
import PostList from '@/components/Content/PostList.vue'
import HeroSection from '@/components/Content/HeroSection.vue'
import AlertMessage from '@/components/UI/AlertMessage.vue'
import CategoryTabs from '@/components/Layout/CategoryTabs.vue'
import { useAppStore } from '@/stores/appStore.js'
import { usePosts } from '@/composables/useBlogApi.js'

const appStore = useAppStore()
const { fetchPosts } = usePosts()
const isFetchingPosts = ref(false)
const posts = ref([])
const categories = ref(new Set())

const filteredPosts = computed(() => {
  if (isViewAllCategoriesActive.value) {
    return posts.value
  }

  return posts.value.filter((post) => {
    // Check if any of the post's categories are in the selected categories set
    return post?.categories.some((category) => categories.value.has(category) && category.active)
  })
})

const isViewAllCategoriesActive = computed(() => {
  for (const category of categories.value) {
    if (category.active) {
      return false
    }
  }
  return true
})

function toggleCategory(category) {
  category.active = !category.active
}

function showAllCategories() {
  categories.value.forEach((category) => {
    category.active = false
  })
}

watchEffect(() => {
  const categoriesSet = new Set()
  posts.value.forEach((post) => {
    post.categories.forEach((category) => categoriesSet.add(category))
  })
  categories.value = categoriesSet
})

onMounted(async () => {
  const { isFetching, data, onFetchError, onFetchResponse } = fetchPosts()

  watchEffect(async () => {
    isFetchingPosts.value = isFetching.value
  })
  onFetchError((error) => {
    // TODO: Show error using a toast or alert message
    console.log(error)
    posts.value = []
  })

  onFetchResponse(async (_) => {
    posts.value = data.value
  })
})
</script>

<template>
  <app-header />
  <div class="section hero">
    <hero-section />
  </div>
  <div class="section posts">
    <div class="section-posts">
      <category-tabs
        :categories="categories"
        class="category-filter"
        @category-clicked="toggleCategory"
        @show-all-categories="showAllCategories"
      />
      <post-list :posts="filteredPosts" :is-fetching-posts="isFetchingPosts" />
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
  scrollbar-color: var(--Colors-Background-bg-brand-solid) white;
  scrollbar-width: thin;
}

@media screen and (max-width: 768px) {
  .section-posts {
    flex-direction: column;
  }
}
</style>
