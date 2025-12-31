<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'
  import { open } from '@tauri-apps/plugin-dialog'
  import { FolderOpen, Moon, Sun } from '@lucide/svelte'
  import { playerState } from '$lib/stores/player.svelte'

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

  async function selectFolder() {
    const selectedFolder = await open({ directory: true })
    playerState.songList = await invoke('get_music_library', { path: selectedFolder })
  }
</script>

<div class="flex justify-between items-center dark:bg-neutral-800/80 rounded-lg p-2">
  <button onclick={selectFolder} class="flex items-center gap-2 bg-gradient-to-br from-purple-700 to-violet-700 rounded-lg p-2 hover:from-purple-600 hover:to-violet-600">
    <FolderOpen size={20} />
    <span>Open Folder</span>
  </button>
  <button onclick={toggleDarkMode} class="p-2 rounded-lg bg-gray-300 dark:bg-neutral-700">
    {#if isDarkMode}
      <Sun size={20} />
    {:else}
      <Moon size={20} />
    {/if}
  </button>
</div>
