<script lang="ts">
  import { errorState } from '$lib/stores/error.svelte'
  import PlayButton from './PlayButton.svelte'
  import VolumeSlider from './VolumeSlider.svelte'
</script>

<div class="p-2  rounded-lg flex items-center justify-between">
  <!-- Left: Error message -->
  <div class="flex-1">
    {#if errorState.error}
      <p class="text-red-600 dark:text-red-400 text-sm">Error: {errorState.error}</p>
    {/if}
  </div>

  <!-- Center: Play/Pause button -->
  <div class="flex-1 flex justify-center">
    <PlayButton/>
  </div>

  <!-- Right: Volume controls -->
  <div class="flex-1 flex justify-end">
    <VolumeSlider />
  </div>
</div>
