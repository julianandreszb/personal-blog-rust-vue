<script setup>
</script>

<template>
  <section class="container">
    <div class="heading-supporting-text">
      <div class="heading-subheading">
        <h2 class="subheading">Our blog</h2>
        <h1 class="heading">The latest writings from our team</h1>
      </div>
      <p class="supporting-text">
        The latest industry news, interviews, technologies, and resources.
      </p>
    </div>
  </section>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;

.container {
  display: flex;
  max-width: var(--container-max-width-desktop);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4xl);
}

.heading-supporting-text {
  display: flex;
  max-width: var(--width-xl);
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-3xl);
  align-self: stretch;
}

.heading-subheading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-lg);
  align-self: stretch;
  
}

.subheading {
  @include text-styles.text-md-semibold;
  color: var(--Colors-Text-text-brand-secondary-700);
  text-align: center;
}

.heading {
  @include text-styles.display-lg-semibold;
  color: var(--Colors-Text-text-primary-900);
  text-align: center;
}

.supporting-text {
  @include text-styles.text-xl-regular;
  color: var(--Colors-Text-text-tertiary-600);
  text-align: center;
}
</style>
