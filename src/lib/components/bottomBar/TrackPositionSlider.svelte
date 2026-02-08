{#if playerStore.currentSong}
  <div class="flex items-center gap-3">
    <span class="text-sm text-neutral-400 dark:text-neutral-500 font-mono tabular-nums">
      {formatTime(playerStore.positionMillis)} / {formatTime(
        playerStore.currentSong.duration_millis
      )}
    </span>
    <input
      type="range"
      min="0"
      max={playerStore.currentSong.duration_millis}
      bind:value={playerStore.positionMillis}
      onchange={() => playerStore.seek()}
      onpointerdown={() => (isDragging = true)}
      onpointerup={() => (isDragging = false)}
      class="flex-1 neo-slider"
      style="--position-percent: {(playerStore.positionMillis /
        playerStore.currentSong.duration_millis) *
        100}%"
    />
  </div>
{/if}

<script lang="ts">
  import { onMount } from 'svelte'
  import { listen } from '@tauri-apps/api/event'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'

  const playerStore = usePlayerStore()

  interface TrackPosition {
    position_seconds: number
  }

  let isDragging = $state(false)

  onMount(() => {
    const unlistenPromise = listen<TrackPosition>('playback:position', (event) => {
      if (!isDragging && !playerStore.isSeeking) {
        playerStore.positionMillis = Math.floor(event.payload.position_seconds * 1000)
      }
    })
    return () => {
      unlistenPromise.then((unlisten) => unlisten())
    }
  })

  function formatTime(millis: number): string {
    const totalSeconds = Math.floor(millis / 1000)
    const minutes = Math.floor(totalSeconds / 60)
    const seconds = totalSeconds % 60
    return `${minutes}:${seconds.toString().padStart(2, '0')}`
  }
</script>

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
    height: 8px;
    background: linear-gradient(
      to right,
      var(--color-purple-700) 0%,
      var(--color-violet-700) var(--position-percent),
      var(--color-neutral-700) var(--position-percent),
      var(--color-neutral-700) 100%
    );
    border-radius: 8px;
  }

  .neo-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    /*background: var(--color-neutral-700);*/
    background: linear-gradient(
      to top left,
      var(--color-purple-700),
      var(--color-violet-700)
    ) !important;
    box-shadow:
      3px 3px 6px rgba(0, 0, 0, 0.5),
      -3px -3px 6px rgba(255, 255, 255, 0.1);
    cursor: grab;
    margin-top: -5px;
  }

  .neo-slider::-webkit-slider-thumb:hover {
    background: linear-gradient(
      to top left,
      var(--color-purple-600),
      var(--color-violet-600)
    ) !important;
  }
</style>
