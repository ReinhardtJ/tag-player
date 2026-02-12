<div class="flex justify-between items-center p-2">
  <OpenFolderButton />
  <div class="flex">
    {#if errorStore.errors.length > 0}
      <a
        href="/error"
        class="mr-2 btn-secondary"
        >Errors ({errorStore.errors.length})</a
      >
    {/if}
    <ThemeToggle />
  </div>
</div>

<script lang="ts">
  import { useErrorStore } from '$lib/stores/errorStore.svelte'
  import OpenFolderButton from './OpenFolderButton.svelte'
  import SearchBar from './SearchBar.svelte'
  import ThemeToggle from './ThemeToggle.svelte'

  const errorStore = useErrorStore()
</script>
