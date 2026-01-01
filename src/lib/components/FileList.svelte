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
      errorState.error = ""
    } catch (e) {
      errorState.error = String(e)
    }
  }
</script>

<div class="h-full overflow-auto p-2 bg-gray-300/30 dark:bg-neutral-800/80 rounded-lg">
  {#each playerState.filteredSongs as song}
    <button
      onclick={() => play(song)}
      class={`hover:bg-violet-500/30 rounded-lg p-2 my-1 w-full cursor-pointer text-left ${song === playerState.currentSong ? 'dark:bg-violet-700' : ''}`}
    >
      {song.name}
    </button>
  {/each}
</div>
