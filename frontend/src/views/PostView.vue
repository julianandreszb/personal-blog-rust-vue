<script setup>
import { onMounted, ref, watchEffect } from 'vue'
import { usePosts } from '@/composables/useBlogApi.js'
import { useAppStore } from '@/stores/appStore.js'

const appStore = useAppStore()
const { fetchPostBySlug } = usePosts()
const props = defineProps({
  slug: {
    type: String,
    required: true,
  },
})
const isFetchingPost = ref(false)
const post = ref({})

onMounted(() => {
  const { isFetching, data, onFetchError, onFetchResponse } = fetchPostBySlug(props.slug)

  watchEffect(async () => {
    isFetchingPost.value = isFetching.value
  })
  onFetchError((error) => {
    appStore.serverError = error
    post.value = {}
  })

  onFetchResponse(async (_) => {
    post.value = data.value
  })
})
</script>

<template>
  <div class="section post">
    <h1>Slug: {{ props.slug }}</h1>
    <pre>{{ post }}</pre>
  </div>
</template>

<style scoped>
.section {
  .post {
    align-items: normal;
    gap: var(--spacing-4xl);
  }
}
</style>