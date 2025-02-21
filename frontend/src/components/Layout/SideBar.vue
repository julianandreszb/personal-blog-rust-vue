<script setup>

</script>

<template>
  <aside class="sidebar">
    <input class="search-input" type="text" placeholder="Search categories" />
    
    <div class="subheading-vertical-tabs" >
      <h2 class="sidebar-title">Blog categories</h2>
      <nav>
        <ul class="nav-links">
          <li>
            <router-link to="/" class="nav-link active">Home</router-link>
          </li>
          <li>
            <router-link to="/" class="nav-link">About</router-link>
          </li>
          <li>
            <router-link to="/" class="nav-link">Contact</router-link>
          </li>
        </ul>
      </nav>  
    </div>
  </aside>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;
.search-input {
  @include text-styles.text-md-regular;
  background: var(--Colors-Background-bg-primary);
  border-radius: var(--radius-md);
  border: 0.1rem solid var(--Colors-Border-border-primary);
  box-shadow: 0 1px 2px 0 var(--Colors-Effects-Shadows-shadow-xs);
  padding: 1rem 1.4rem;
  width: 100%;
  color: var(--Colors-Text-text-placeholder, #717680);

  &:active, &:focus, &:focus-within {
    border: 0.2rem solid var(--Colors-Border-border-brand);
    outline: none;
    color: var(--Colors-Text-text-primary-900);
  }
}

.sidebar{
  display: flex;
  width: 280px;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-4xl, 32px);
}

.subheading-vertical-tabs {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-2xl, 20px);
  align-self: stretch;
}

.sidebar-title {
  @include text-styles.text-sm-semibold;
  color: var(--Colors-Text-text-brand-secondary-700);
}

nav {
  width: 100%;
}

.nav-links {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xs, 4px);
  align-self: stretch;
  list-style: none;

  li {
    width: 100%;
  }
}

.nav-link {
  @include text-styles.text-md-semibold;
  display: flex;
  height: 4.4rem;
  padding: var(--spacing-md) var(--spacing-lg);
  align-items: center;
  gap: var(--spacing-md, 0.8rem);
  align-self: stretch;
  text-decoration: none;
  color: var(--Colors-Text-text-quaternary-500);
  border-radius: var(--spacing-sm);
  
  &.active {
    background: var(--Colors-Background-bg-active);
  }
}

</style>