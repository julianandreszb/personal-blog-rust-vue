<script setup>
import PostListItem from '@/components/Content/PostListItem.vue'

const props = defineProps({
  posts: {
    type: Array,
    required: true,
  },
  isFetchingPosts: {
    type: Boolean,
    required: true,
  }
})

</script>

<template>
  <div class="post-list">
    <template v-if="props.isFetchingPosts">
      <div>
        <p>Loading posts...</p>
      </div>
    </template>
    <template v-else-if="props.posts?.length === 0">
      <div>
        <p>No posts found.</p>
      </div>
    </template>
    <template v-else>
      <post-list-item v-for="post in props.posts" :post="post" :key="post.id" />
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
