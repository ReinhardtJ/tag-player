<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Volume2 } from '@lucide/svelte'
  import { errorState } from '$lib/stores/error.svelte'
  import PlayButton from './PlayButton.svelte'

  let volume = $state(50)

  async function onVolumeChange() {
    await invoke('volume_change', { volume: volume / 100 })
  }
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
  <div class="flex-1 flex items-center justify-end gap-2">
    <Volume2 size={20}/>
    <input
        type="range"
        min="0"
        max="100"
        bind:value={volume}
        oninput={onVolumeChange}
        class="w-24 neo-slider"
        style="--volume-percent: {volume}%"
    >
  </div>
</div>

<style>
    /* Remove default styling */
    .neo-slider {
        -webkit-appearance: none;
        appearance: none;
        background: transparent;
        cursor: pointer;
    }

    /* Track - Neomorphic inset with progressive purple fill and rounded end */
    .neo-slider::-webkit-slider-runnable-track {
        width: 100%;
        height: 18px;
        background: 
            radial-gradient(circle at var(--volume-percent) 50%, 
                var(--color-violet-700) 0%, 
                var(--color-violet-700) 9px,
                transparent 9px
            ),
            linear-gradient(
                to right,
                var(--color-purple-700) 0%,
                var(--color-violet-700) var(--volume-percent),
                var(--color-neutral-800) var(--volume-percent),
                var(--color-neutral-800) 100%
            );
        border-radius: 12px;
        box-shadow: inset 2px 2px 4px rgba(0, 0, 0, 0.4),
                    inset -2px -2px 4px rgba(255, 255, 255, 0.05);
    }

    /* Thumb - Invisible but still draggable */
    .neo-slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 0;
        height: 18px;
        margin-top: 0;
    }

</style>
