<div class="flex justify-between items-center">
  <OpenFolderButton />
  <div class="flex gap-2">
    <ErrorButton />
    <ThemeToggle />
  </div>
</div>

<script lang="ts">
  import ErrorButton from './ErrorButton.svelte'
  import OpenFolderButton from './OpenFolderButton.svelte'
  import ThemeToggle from './ThemeToggle.svelte'
</script>
