<div class="flex justify-between items-center p-2">
  <OpenFolderButton />
  <SearchBar />
  <div class="flex">
    {#if errorState.errors.length > 0}
      <a
        href="/error"
        class="mr-2 neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600"
        >Errors ({errorState.errors.length})</a
      >
    {/if}
    <ThemeToggle />
  </div>
</div>

<script lang="ts">
  import OpenFolderButton from './OpenFolderButton.svelte'
  import SearchBar from './SearchBar.svelte'
  import ThemeToggle from './ThemeToggle.svelte'
  import { errorState } from '$lib/stores/error.svelte'
</script>
