<script setup>
import { computed } from 'vue'

const props = defineProps({
  categories: {
    type: Set,
    required: true,
  },
})
const emit = defineEmits(['category-clicked'])
const isViewAllCategoriesActive = computed(() => {
  for (const category of props.categories) {
    if (category.active) {
      return false
    }
  }
  return true
})

async function handleCategoryClicked(category) {
  emit('category-clicked', category)
}
</script>

<template>
  <nav>
    <ul class="category-tabs">
      <li :class="{ active: isViewAllCategoriesActive }" class="tab">
        <span class="nav-link">View all</span>
      </li>
      <li
        v-for="category in props.categories"
        :key="category.id"
        class="tab"
        :class="{ active: category.active }"
        @click="handleCategoryClicked(category)"
      >
        <span class="nav-link">{{ category.name }}</span>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;

.category-tabs {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  border-radius: var(--radius-md);
  border: 1px solid var(--Colors-Border-border-secondary);
  background: var(--Colors-Background-bg-secondary_alt);
  list-style: none;
  width: max-content;
}

.tab {
  padding-block: var(--spacing-md);
  border-radius: var(--spacing-md);
  background: transparent;
  box-shadow: 0 1px 2px 0 var(--Colors-Effects-Shadows-shadow-xs);
  padding: var(--spacing-md) var(--spacing-lg);
  cursor: pointer;

  &.active {
    background: var(--Colors-Background-bg-primary);
    border: 1px solid var(--Colors-Border-border-primary);

    .nav-link {
      color: var(--Colors-Text-text-secondary-700);
    }
  }

  .nav-link {
    @include text-styles.text-md-semibold;
    text-decoration: none;
    color: var(--Colors-Text-text-quaternary-500);
  }
}
</style>
