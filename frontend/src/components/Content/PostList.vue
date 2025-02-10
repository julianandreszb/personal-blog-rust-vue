<script setup>
import { onMounted, ref, watchEffect } from 'vue'
import { usePosts } from '@/composables/useBlogApi.js'

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

<template><pre>{{ posts }}</pre></template>

<style scoped></style>
