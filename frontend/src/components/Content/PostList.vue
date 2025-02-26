<script setup>
import { onMounted, ref, watchEffect } from 'vue'
import { usePosts } from '@/composables/useBlogApi.js'
import PostListItem from '@/components/Content/PostListItem.vue'

const posts = ref([])
const isFetchingPosts = ref(false)
const { fetchPosts } = usePosts()
onMounted(async () => {
  const { isFetching, data, onFetchError, onFetchResponse } = fetchPosts()
  //execute()
  watchEffect(async () => {
    isFetchingPosts.value = isFetching.value
  })
  onFetchError((error) => {
    console.log(error)
    posts.value = []
  })

  onFetchResponse(async (response) => {
    console.log(`response: ${response}`)
    posts.value = data.value
  })
  //
  // await execute()
  // posts.value = data.value
})
</script>

<template>
  <div class="post-list">
    <template v-if="isFetchingPosts">
      <div>
        <p>Loading posts...</p>  
      </div>
    </template>
    <template v-else-if="posts?.length === 0">
      <div>
        <p>No posts found.</p>
      </div>
    </template>
    <template v-else>
      <post-list-item v-for="post in posts" :post="post" :key="post.id" />
    </template>
  </div>
</template>

<style scoped>
.post-list {
  width: 100%;
  column-gap: var(--spacing-xl);
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(28rem, 1fr));
  row-gap: var(--spacing-4xl);
}
</style>
