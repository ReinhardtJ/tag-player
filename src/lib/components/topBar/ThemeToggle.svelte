<button onclick={toggleDarkMode} class="btn-secondary">
  {#if isDarkMode}
    <Sun size={20} />
  {:else}
    <Moon size={20} />
  {/if}
</button>

<script lang="ts">
  import { onMount } from 'svelte'
  import { Moon, Sun } from '@lucide/svelte'

  let isDarkMode = $state(true)

  onMount(() => {
    // Load dark mode preference from localStorage
    const savedMode = localStorage.getItem('darkMode')
    isDarkMode = savedMode !== 'false' // default to true if not set

    if (isDarkMode) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  })

  function toggleDarkMode() {
    isDarkMode = !isDarkMode

    if (isDarkMode) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }

    // Persist preference
    localStorage.setItem('darkMode', String(isDarkMode))
  }
</script>
