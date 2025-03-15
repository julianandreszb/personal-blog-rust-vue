<script setup>
import { onMounted, ref, watchEffect } from 'vue'
import { usePosts } from '@/composables/useBlogApi.js'
import { useAppStore } from '@/stores/appStore.js'
import MarkdownRenderer from '@/components/MarkdownRenderer.vue'
import VueDivider from '@/components/UI/VueDivider.vue'

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

const markdownInput = ref(`
# Heading 1

This is some example Markdown.

\`\`\`javascript
function greet(name) {
  console.log("Hello, " + name + "!");
}

greet("World");
\`\`\`

- List item 1
- List item 2

> A blockquote

[A link](https://www.example.com)

\`inline code\`
`)

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
    <section class="blog-section">
      <div class="blog-container">
        <div class="blog-content-and-sidebar">
          <aside class="sidebar">
            <vue-divider />
            <div class="heading-and-toc">
              <h2 class="heading">Table of contents</h2>
              <ul class="toc">
                <li class="toc-item">Test Row</li>
                <li class="toc-item">Test Row</li>
                <li class="toc-item">Test Row</li>
                <li class="toc-item">Test Row</li>
              </ul>
            </div>
            <vue-divider />
            <div class="social-links">
              <a href="#" class="social-link">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none">
                  <path d="M10.5893 15.3032L9.4108 16.4817C7.78361 18.1089 5.14542 18.1089 3.51824 16.4817C1.89106 14.8546 1.89106 12.2164 3.51824 10.5892L4.69675 9.41068M15.3034 10.5892L16.4819 9.41067C18.109 7.78349 18.109 5.1453 16.4819 3.51812C14.8547 1.89093 12.2165 1.89093 10.5893 3.51812L9.4108 4.69663M7.08339 12.9166L12.9167 7.08325" stroke="#414651" stroke-width="1.66667" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </a>
              <a href="#" class="social-link">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none">
                  <path d="M18.5195 0H1.47656C0.660156 0 0 0.644531 0 1.44141V18.5547C0 19.3516 0.660156 20 1.47656 20H18.5195C19.3359 20 20 19.3516 20 18.5586V1.44141C20 0.644531 19.3359 0 18.5195 0ZM5.93359 17.043H2.96484V7.49609H5.93359V17.043ZM4.44922 6.19531C3.49609 6.19531 2.72656 5.42578 2.72656 4.47656C2.72656 3.52734 3.49609 2.75781 4.44922 2.75781C5.39844 2.75781 6.16797 3.52734 6.16797 4.47656C6.16797 5.42188 5.39844 6.19531 4.44922 6.19531ZM17.043 17.043H14.0781V12.4023C14.0781 11.2969 14.0586 9.87109 12.5352 9.87109C10.9922 9.87109 10.7578 11.0781 10.7578 12.3242V17.043H7.79688V7.49609H10.6406V8.80078H10.6797C11.0742 8.05078 12.043 7.25781 13.4844 7.25781C16.4883 7.25781 17.043 9.23438 17.043 11.8047V17.043Z" fill="#A4A7AE"/>
                </svg>
              </a>
            </div>
          </aside>
          <MarkdownRenderer class="blog-content" :markdown="markdownInput" />
        </div>
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

.blog-section {
  display: flex;
  padding-bottom: var(--spacing-9xl);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-7xl);
}

.blog-container {
  display: flex;
  max-width: var(--container-max-width-desktop);
  padding: 0 var(--container-padding-desktop);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-7xl);
}

.blog-content-and-sidebar {
  display: flex;
  max-width: var(--width-2xl);
  align-items: flex-start;
  gap: var(--spacing-7xl);
  align-self: stretch;
}

.sidebar {
  display: flex;
  min-width: 240px;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-4xl);
}

.blog-content {
  display: flex;
  max-width: var(--paragraph-max-width);
  flex-direction: column;
  align-items: flex-start;
  flex: 1 0 0;
}

.heading-and-toc {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xl);

  .heading {
    @include text-styles.text-md-semibold;
    color: var(--Colors-Text-text-brand-tertiary-600);
  }

  .toc {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-lg);
    list-style: none;

    .toc-item {
      @include text-styles.text-md-semibold;
      color: var(--button-tertiary-fg);
    }
  }
}

.social-links {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-lg, 12px);

  .social-link {
    display: flex;
    padding: 10px;
    justify-content: center;
    align-items: center;
    
    border-radius: var(--radius-md);
    border: 1px solid var(--button-secondary-border);
    background: var(--button-secondary-bg);

    /* Shadows/shadow-xs-skeuomorphic */
    box-shadow:
      0 0 0 1px
        rgba(10, 13, 18, 0.18) inset,
      0 -2px 0 0
        rgba(10, 13, 18, 0.05) inset,
      0 1px 2px 0 var(--Colors-Effects-Shadows-shadow-xs, rgba(10, 13, 18, 0.05));
  }
}
</style>
