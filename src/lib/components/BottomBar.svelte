<script lang="ts">
  import { errorState } from '$lib/stores/error.svelte'
  import PlayButton from './PlayButton.svelte'
  import VolumeSlider from './VolumeSlider.svelte'
  import { playerState } from '$lib/stores/player.svelte'
  import { onMount } from 'svelte'
  import { listen } from '@tauri-apps/api/event'

  interface AudioPosition {
    position_seconds: number
    duration_seconds: number
  }

  let isDragging = $state(false)

  onMount(() => {
    const unlistenPromise = listen<AudioPosition>('playback:position', (event) => {
      console.log('Full event:', event)
      console.log('Payload', event.payload)
      if (!isDragging) {
        positionMillis = Math.floor(event.payload.position_seconds * 1000)

      }
    })
    return () => {
      unlistenPromise.then(unlisten => unlisten())
    }
  })

  let positionMillis = $state(0)

  async function seek() {
    console.log('seek in component')
    await playerState.seek(positionMillis)
  }
</script>


<div class="p-2 rounded-lg ">
  <div>
    {#if playerState.currentSong}
      <input
          type="range"
          min="0"
          max={playerState.currentSong.duration_millis}
          bind:value={positionMillis}
          onchange={seek}
          onpointerdown={() => isDragging = true}
          onpointerup={() => isDragging = false}
      >
    {/if}
  </div>
  <div class="flex items-center justify-between">

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
      <VolumeSlider/>
    </div>
  </div>
</div>
