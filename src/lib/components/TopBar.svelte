<script lang="ts">
  import { Search } from '@lucide/svelte'
  import OpenFolderButton from './OpenFolderButton.svelte'
  import ThemeToggle from './ThemeToggle.svelte'
  import { playerState } from '$lib/stores/player.svelte'


</script>

<div class="flex justify-between items-center dark:bg-neutral-800/80 rounded-lg p-2">
  <!-- Open Folder Button -->
  <OpenFolderButton />
  <!-- Search Bar -->
  <div class="flex items-center gap-2 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2 flex-1 mx-4 max-w-md">
    <Search size={20} class="text-gray-600 dark:text-gray-400" />
    <input
      type="text"
      bind:value={playerState.searchQuery}
      placeholder="Search songs..."
      class="bg-transparent outline-none flex-1 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400"
    />
  </div>
  <!-- Theme Toggle -->
  <ThemeToggle />
</div>
