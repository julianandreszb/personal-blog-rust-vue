<script setup>
</script>

<template>
  <nav>
    <ul class="horizontal-tabs">
      <li class="tab active">
        <router-link to="/" class="nav-link">View all</router-link>
      </li>
      <li class="tab">
        <router-link to="/" class="nav-link">Design</router-link>
      </li>
      <li class="tab">
        <router-link to="/" class="nav-link">Algorithms</router-link>
      </li>
      <li class="tab">
        <router-link to="/" class="nav-link">Backend</router-link>
      </li>
      <li class="tab">
        <router-link to="/" class="nav-link">Product</router-link>
      </li>
      <li class="tab">
        <router-link to="/" class="nav-link">Frontend</router-link>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;
.horizontal-tabs {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  border-radius: var(--radius-md);
  border: 1px solid var(--Colors-Border-border-secondary);
  background: var(--Colors-Background-bg-secondary_alt);
  list-style: none;
}

.tab {
  padding-block: var(--spacing-md);
  border-radius: var(--spacing-md);
  background: transparent;
  box-shadow: 0 1px 2px 0 var(--Colors-Effects-Shadows-shadow-xs);
  padding: var(--spacing-md) var(--spacing-lg);

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
