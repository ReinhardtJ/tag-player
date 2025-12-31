<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Play, Pause, Volume2 } from '@lucide/svelte'
  import { playerState } from '$lib/stores/player.svelte'
  import { errorState } from '$lib/stores/error.svelte'

  let volume = $state(50)

  async function onVolumeChange() {
    await invoke('volume_change', { volume: volume / 100 })
  }

  async function togglePlayback() {
    const result = await invoke('toggle_playback')
    if (result !== null) {
      errorState.setError(String(result))
      return
    }
    playerState.isPlaying = !playerState.isPlaying
  }
</script>

<div class="p-2 bg-gray-300 dark:bg-neutral-800/80 rounded-lg flex items-center justify-between">
  <!-- Left: Error message -->
  <div class="flex-1">
    {#if errorState.error}
      <p class="text-red-600 dark:text-red-400 text-sm">Error: {errorState.error}</p>
    {/if}
  </div>

  <!-- Center: Play/Pause button -->
  <div class="flex-1 flex justify-center">
    <button
      onclick={togglePlayback}
      class="bg-linear-to-br from-purple-700 to-violet-700 rounded-full p-3 hover:from-purple-600 hover:to-violet-600"
    >
      {#if playerState.isPlaying}
        <Pause size={18} fill="currentColor" />
      {:else}
        <Play size={18} fill="currentColor" />
      {/if}
    </button>
  </div>

  <!-- Right: Volume controls -->
  <div class="flex-1 flex items-center justify-end gap-2">
    <Volume2 size={20} />
    <input
      type="range"
      min="0"
      max="100"
      bind:value={volume}
      oninput={onVolumeChange}
      class="w-24"
    >
  </div>
</div>
