<script setup>
import BaseCard from '@/components/UI/BaseCard.vue' // Import the BaseCard component

const props = defineProps({
  post: {
    type: Object,
    required: true,
  },
})
</script>

<template>
  <BaseCard class="post-list-item">
    <img class="post-cover-image" :src="props.post.featured_image" alt="Cover image" />
    <section class="card-content">
      <div class="card-heading-and-text" >
        <time class="card-date">{{ props.post.updated_at}}</time>
        <div class="card-heading-and-icon" >
          <h1 class="card-heading-title" >{{ props.post.title }}</h1>
          <div class="icon-wrap" >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none">
              <path d="M7 17L17 7M17 7H7M17 7V17" stroke="#181D27" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </div>
        <p class="card-supporting-text" >{{ props.post.excerpt }}</p>
      </div>
      <ul class="card-categories" >
          <li class="card-badge" v-for="category in props.post.categories" :key="category.id" >{{ category.name }}</li>
      </ul>
    </section>
  </BaseCard>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;
.post-list-item {
  .post-cover-image {
    border-top-left-radius: var(--spacing-xl);
    border-top-right-radius: var(--spacing-xl);
    background: lightgray 50% / cover no-repeat;
    object-fit: cover;
    display: block;
    width: 100%;
    height: 240px;
  }

  .card-content {
    display: flex;
    padding: var(--spacing-3xl, 24px);
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-3xl, 24px);
    align-self: stretch;
  }
  
  .card-heading-and-text {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-md, 8px);
    align-self: stretch;
  }
  
  .card-date {
    @include text-styles.text-sm-semibold;
    color: var(--Colors-Text-text-brand-secondary-700);
  }
  
  .card-heading-and-icon {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-xl, 16px);
    align-self: stretch;
    justify-content: space-between;
  }
  
  .card-heading-title {
    @include text-styles.display-xs-semibold;
    color: var(--Colors-Text-text-primary-900);
  }
  
  .icon-wrap{
    display: flex;
    padding-top: var(--spacing-xs, 4px);
    flex-direction: column;
    align-items: flex-start;
  }
  
  .card-supporting-text {
    @include text-styles.text-md-regular;

    overflow: hidden;
    color: var(--Colors-Text-text-tertiary-600);
    text-overflow: ellipsis;
  }
  
  .card-categories {
    display: flex;
    align-items: flex-start;
    align-content: flex-start;
    gap: 0.8rem var(--spacing-md);
    align-self: stretch;
    flex-wrap: wrap;
    list-style: none;
  }
  
  .card-badge{
    @include text-styles.text-sm-medium;
    /* Layout */
    display: flex;
    padding: var(--spacing-xxs, 2px) 10px;
    align-items: center;
    
    /* Style */
    border-radius: var(--radius-full);
    border: 1px solid var(--Component-colors-Utility-Brand-utility-brand-200);
    background: var(--Component-colors-Utility-Brand-utility-brand-50);
  
    color: var(--Component-colors-Utility-Brand-utility-brand-700);
    text-align: center;
  }
}
</style>
