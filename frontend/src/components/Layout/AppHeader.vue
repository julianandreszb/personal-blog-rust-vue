<script setup>
import DarkModeToggle from '@/components/UI/DarkModeToggle.vue'
</script>

<template>
  <header class="app-header">
    <nav class="navbar">
      <div class="navbar-brand">
        <router-link to="/" class="brand-link">JulianZB | Blog</router-link>
      </div>

      <input type="checkbox" id="menu-toggle" class="menu-toggle" aria-label="Toggle menu" />
      <label for="menu-toggle" class="hamburger-menu">
        <span class="hamburger-icon"></span>
        <span class="hamburger-icon"></span>
        <span class="hamburger-icon"></span>
      </label>

      <ul class="nav-links">
        <li>
          <router-link to="/" class="nav-link">Home</router-link>
        </li>
        <li>
          <dark-mode-toggle />
        </li>
      </ul>
    </nav>
  </header>
</template>

<style lang="scss" scoped>
@use '../../assets/text-styles' as text-styles;

.app-header {
  background-color: var(--Colors-Background-bg-primary);
  border-bottom: 0.1rem solid var(--Colors-Border-border-primary);
  height: 4rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2.4rem 0;
  margin-inline: var(--spacing-3xl);
  position: sticky;
  top: 0;
}

.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: relative; /* Keep this! */
}

.navbar-brand {
  /*margin-inline-start: 1rem;*/
  font-size: 1.5rem;
  font-weight: bold;
  z-index: 20; /* Higher z-index than .nav-links */
  position: relative; /* Needed for z-index to work */
}

.brand-link {
  text-decoration: none;
  color: inherit;
}

.nav-links {
  /*margin-inline-end: 1rem;*/
  list-style: none;
  display: flex;
  padding: 0;
}

.nav-links li {
  margin-left: 1rem;
}

.nav-link {
  @include text-styles.text-md-semibold;
  background-color: var(--Colors-Background-bg-primary);
  text-decoration: none;
  color: var(--button-tertiary-fg);
  padding: 0.5rem;
  border-radius: 4px;
  transition: background-color 0.2s ease;
  display: block;
  user-select: none;
}

.nav-link:hover {
  color: var(--button-tertiary-fg_hover);
}

.logout-button {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  color: inherit;
  font: inherit;
}

/* --- Hamburger Menu and Checkbox --- */
.menu-toggle {
  display: none;
}

.hamburger-menu {
  display: none;
  cursor: pointer;
  padding: 0.5rem;
  z-index: 20; /* Higher z-index than .nav-links */
  position: relative; /* Needed for z-index to work*/
}

.hamburger-icon {
  display: block;
  width: 25px;
  height: 3px;
  margin: 5px 0;
  transition: transform 0.2s ease;
  background-color: var(--fg-secondary-700);
  border-radius: var(--radius-full);
}

/* --- Mobile Menu Styles (using :checked) --- */
@media (max-width: 768px) {
  
  .app-header {
    margin-inline: 0;
  }
  
  .hamburger-menu {
    display: block;
    margin-inline-end: var(--spacing-lg);
  }

  .nav-links {
    display: none;
    flex-direction: column;
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
    z-index: 10; /* Lower z-index than hamburger and brand */
    overflow: hidden;
    max-height: 0;
    transition: max-height 0.3s ease-in-out;
  }

  .menu-toggle:checked ~ .nav-links {
    display: flex;
    max-height: 500px;
    /*height: 500px;*/
  }

  .nav-links li {
    margin-left: 0;
    width: 100%;
    text-align: center;
  }

  .nav-links li:last-child {
    border-bottom: 0.1rem solid var(--Colors-Border-border-primary);
  }

  .nav-links li a {
    width: 100%; /*To make the entire area clickable*/
  }

  /* Hamburger icon animation */
  .menu-toggle:checked + .hamburger-menu .hamburger-icon:first-child {
    transform: translateY(8px) rotate(45deg);
  }

  .menu-toggle:checked + .hamburger-menu .hamburger-icon:nth-child(2) {
    opacity: 0;
  }

  .menu-toggle:checked + .hamburger-menu .hamburger-icon:last-child {
    transform: translateY(-8px) rotate(-45deg);
  }
}
</style>
