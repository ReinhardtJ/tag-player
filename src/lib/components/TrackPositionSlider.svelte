<script lang="ts">
  import { playerState } from '$lib/stores/player.svelte'
  import { onMount } from 'svelte'
  import { listen } from '@tauri-apps/api/event'

  interface TrackPosition {
    position_seconds: number
  }

  let positionMillis = $state(0)
  let isDragging = $state(false)

  onMount(() => {
    const unlistenPromise = listen<TrackPosition>('playback:position', (event) => {
      if (!isDragging) {
        positionMillis = Math.floor(event.payload.position_seconds * 1000)
      }
    })
    return () => {
      unlistenPromise.then(unlisten => unlisten())
    }
  })

  async function seek() {
    await playerState.seek(positionMillis)
  }
</script>

{#if playerState.currentSong}
  <input
      type="range"
      min="0"
      max={playerState.currentSong.duration_millis}
      bind:value={positionMillis}
      onchange={seek}
      onpointerdown={() => isDragging = true}
      onpointerup={() => isDragging = false}
      class="w-full neo-slider"
      style="--position-percent: {(positionMillis / playerState.currentSong.duration_millis) * 100}%"
  >
{/if}

<style>
    /* Remove default styling */
    .neo-slider {
        -webkit-appearance: none;
        appearance: none;
        background: transparent;
        cursor: pointer;
    }

    .neo-slider::-webkit-slider-runnable-track {
        width: 100%;
        height: 4px;
        background: linear-gradient(
                to right,
                var(--color-purple-700) 0%,
                var(--color-violet-700) var(--position-percent),
                var(--color-neutral-700) var(--position-percent),
                var(--color-neutral-700) 100%
        );
        border-radius: 2px;
    }

    .neo-slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 18px;
        height: 18px;
        border-radius: 50%;
        /*background: var(--color-neutral-700);*/
        background: linear-gradient(to top left, var(--color-purple-700), var(--color-violet-700)) !important;
        box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.5),
        -3px -3px 6px rgba(255, 255, 255, 0.1);
        cursor: grab;
        margin-top: -7px;
    }

    .neo-slider::-webkit-slider-thumb:hover {
        background: linear-gradient(to top left, var(--color-purple-600), var(--color-violet-600)) !important;

    }
</style>
