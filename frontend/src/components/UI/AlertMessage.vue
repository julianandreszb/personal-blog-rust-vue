<script setup>
import { useSlots } from 'vue'

const slots = useSlots()
</script>

<template>
  <div class="alert-message">
    <div class="alert-buttons">
      <svg
        class="x-close"
        width="20"
        height="20"
        viewBox="0 0 20 20"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M15 5L5 15M5 5L15 15"
          stroke="#A4A7AE"
          stroke-width="1.66667"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </div>

    <div class="featured-icon-outline">
      <div class="outline-inner"></div>
      <div class="outline-outer"></div>
      <svg
        class="error-icon"
        width="20"
        height="20"
        viewBox="0 0 20 20"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g clip-path="url(#clip0_22029_5448)">
          <path
            d="M9.99984 6.66663V9.99996M9.99984 13.3333H10.0082M18.3332 9.99996C18.3332 14.6023 14.6022 18.3333 9.99984 18.3333C5.39746 18.3333 1.6665 14.6023 1.6665 9.99996C1.6665 5.39759 5.39746 1.66663 9.99984 1.66663C14.6022 1.66663 18.3332 5.39759 18.3332 9.99996Z"
            stroke="#D92D20"
            stroke-width="1.66667"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </g>
        <defs>
          <clipPath id="clip0_22029_5448">
            <rect width="20" height="20" fill="white" />
          </clipPath>
        </defs>
      </svg>
    </div>

    <div class="alert-content">
      <div class="text-and-supporting-text">
        <h3 v-if="slots.title" class="alert-title">
          <slot name="title"></slot>
        </h3>
        <p class="alert-text">
          <slot></slot>
        </p>
      </div>

      <div class="alert-actions">
        <a class="btn-dismiss">Dismiss</a>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@use '../../assets/text-styles' as text-styles;

.alert-message {
  position: relative;
  display: flex;
  width: 343px;
  padding: var(--spacing-xl);
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xl);

  border-radius: var(--radius-xl);
  border: 1px solid var(--Colors-Border-border-primary);
  background: var(--Colors-Background-bg-primary_alt);
  box-shadow: 0 1px 2px 0 var(--Colors-Effects-Shadows-shadow-xs);
}

.alert-buttons {
  display: flex;
  width: 36px;
  height: 36px;
  padding: var(--spacing-md);
  justify-content: center;
  align-items: center;
  position: absolute;
  right: 8px;
  top: 8px;

  .x-close {
    cursor: pointer;

    path {
      stroke: var(--button-quinary-fg);
    }

    &:hover {
      path {
        stroke: var(--button-quinary-fg_hover);
      }
    }
  }
}

.featured-icon-outline {
  width: 20px;
  height: 20px;
  position: relative;

  .outline-inner {
    border-radius: var(--spacing-3xl);
    border: 2px solid var(--fg-error-primary);
    height: 28px;
    left: -4px;
    opacity: 0.3;
    position: absolute;
    top: -4px;
    width: 28px;
  }

  .outline-outer {
    position: absolute;
    width: 38px;
    height: 38px;
    border-radius: var(--spacing-3xl);
    border: 2px solid var(--fg-error-primary);
    opacity: 0.1;
    left: -9px;
    top: -9px;
  }
}

.alert-title {
  @include text-styles.text-sm-semibold;
  color: var(--Colors-Text-text-secondary-700);
}

.alert-text {
  @include text-styles.text-sm-regular;
  color: var(--Colors-Text-text-primary-900);
}

.alert-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-lg);
  align-self: stretch;
}

.text-and-supporting-text {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xs);
}

.alert-actions {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-lg);
}

.btn-dismiss {
  @include text-styles.text-sm-semibold;
  color: var(--button-tertiary-fg);
  user-select: none;
  cursor: pointer;

  &:hover {
    color: var(--button-tertiary-fg_hover);
  }
}
</style>
