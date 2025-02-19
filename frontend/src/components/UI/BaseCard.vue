<script setup></script>

<template>
  <article class="card-base">
    <slot />
  </article>
</template>

<style scoped>
.card-base {
  display: flex;
  width: 384px;
  min-width: 320px;
  flex-direction: column;
  align-items: flex-start;

  border-radius: var(--spacing-xl);
  border: 1px solid var(--Colors-Border-border-secondary);
  background: var(--Colors-Background-bg-primary_alt);
}

</style>