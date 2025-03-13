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
    <article>
      <section class="header-section">
        <div class="container">
          <div class="heading-and-supporting-text">
            <div class="heading-and-subheading">
              <time class="published">Last updated on {{ post.updated_at }}</time>
              <h1 class="heading-title">{{ post.title }}</h1>
            </div>
            <p class="supporting-text">{{ post.excerpt }}</p>
          </div>
          <div>
            <ul class="card-tags">
              <li class="card-badge" v-for="tag in post.tags" :key="tag.id">
                {{ tag.name }}
              </li>
            </ul>
          </div>
        </div>
        <div class="container">
          <img class="featured-image" :src="post.featured_image" alt="" />
        </div>
      </section>
    </article>
</template>

<style lang="scss" scoped>
@use '../assets/text-styles' as text-styles;

.section {
  .post {
    align-items: normal;
    gap: var(--spacing-4xl);
  }
}

.header-section {
  display: flex;
  padding: var(--spacing-9xl) 0;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-7xl);
}

.container {
  display: flex;
  max-width: var(--container-max-width-desktop);
  padding: 0 var(--container-padding-desktop);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4xl);
}

.heading-and-supporting-text {
  display: flex;
  max-width: var(--width-xl);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-3xl);
}

.heading-and-subheading {
  display: flex;
  flex-direction: column;
  align-items: normal;
  gap: var(--spacing-lg);
}

.published {
  @include text-styles.text-sm-semibold;
  color: var(--Colors-Text-text-brand-secondary-700);
  text-align: center;
}

.heading-title {
  @include text-styles.display-lg-semibold;
  color: var(--Colors-Text-text-primary-900);
  text-align: center;
}

.supporting-text {
  @include text-styles.text-xl-regular;
  color: var(--Colors-Text-text-tertiary-600);
  text-align: center;
}

.featured-image {
  width: 100%;
}
</style>
