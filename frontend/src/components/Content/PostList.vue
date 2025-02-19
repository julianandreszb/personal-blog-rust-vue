<script setup>
import { onMounted, ref, watchEffect } from 'vue'
import { usePosts } from '@/composables/useBlogApi.js'
import PostListItem from '@/components/Content/PostListItem.vue'

const posts = ref([])
const isFetchingPosts = ref(false)
const { fetchPosts } = usePosts()
onMounted(async () => {
  const { isFetching, data, execute, onFetchError } = fetchPosts()
  watchEffect(async () => {
    isFetchingPosts.value = isFetching.value
  })
  onFetchError((error) => console.error('Error fetching posts:', error))
  await execute()
  posts.value = data.value
})
</script>

<template>
  <div class="post-list">
    <post-list-item v-for="post in posts" :post="post" :key="post.id" />  
  </div>
</template>

<style scoped>
.post-list {
  border-radius: var(--radius-2xl);
  column-gap: var(--spacing-xl);
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(38rem, 1fr));
  row-gap: var(--spacing-4xl);
}
</style>
