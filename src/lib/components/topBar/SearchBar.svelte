<div
  class="flex inset-shadow-sm inset-shadow-neutral-800 items-center gap-2
 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2 flex-1 mx-4 max-w-md"
>
  <Search size={20} class="text-gray-600 dark:text-neutral-300" />
  <input
    type="text"
    bind:value={playerStore.searchQuery}
    placeholder="Search songs..."
    class="bg-transparent outline-none flex-1 text-gray-900 dark:text-white placeholder-neutral-600 dark:placeholder-neutral-400"
  />
</div>

<script lang="ts">
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { Search } from '@lucide/svelte'

  const playerStore = usePlayerStore()
</script>
