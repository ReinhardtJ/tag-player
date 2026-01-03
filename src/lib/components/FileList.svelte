<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { playerState, type Song } from '$lib/stores/player.svelte'
  import { errorState } from '$lib/stores/error.svelte'

  async function play(song: Song) {
    try {
      const result = await invoke('load_and_play', { path: song.path })
      if (result !== null) {
        errorState.error = String(result)
        return
      }
      playerState.isLoaded = true
      playerState.isPlaying = true
      playerState.currentSong = song
      errorState.error = ''
    } catch (e) {
      errorState.error = String(e)
    }
  }
</script>

<div class="h-full gradient-border rounded-3xl overflow-hidden">
  <div class="h-full overflow-auto p-2 custom-scrollbar">
    {#each playerState.filteredSongs as song}
      <div>
        <button
            onclick={() => play(song)}
            class={`neo-raised-xs bg-neutral-800 hover:bg-neutral-700 rounded-lg p-2 my-1.5 w-full cursor-pointer text-left
          ${song === playerState.currentSong ? 'bg-linear-to-br dark:from-purple-700 to-violet-700' : ''}`}
        >
          {song.name}
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px !important;
    }

    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent !important;
        border-radius: 0 1.5rem 1.5rem 0;
    }

    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: linear-gradient(to bottom right, var(--color-purple-700), var(--color-violet-700)) !important;
        border-radius: 12px;
    }

    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: linear-gradient(to bottom right, var(--color-purple-600), var(--color-violet-600)) !important;
    }
</style>