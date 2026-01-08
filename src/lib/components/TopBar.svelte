<div class="flex justify-between items-center rounded-lg p-2">
  <OpenFolderButton/>
  <SearchBar/>
  {#if errorState.errors.length > 0}
    <a
        href="/error"
        class="neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600 inline-flex items-center justify-center"
    >Errors ({errorState.errors.length})</a>
  {/if}
  <ThemeToggle/>
</div>

<script lang="ts">
  import OpenFolderButton from './OpenFolderButton.svelte'
  import SearchBar from './SearchBar.svelte'
  import ThemeToggle from './ThemeToggle.svelte'
  import { errorState } from '$lib/stores/error.svelte'

</script>
