<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { playerState, type MusicFile } from '$lib/stores/player.svelte'
  import { errorState } from '$lib/stores/error.svelte'

  async function play(song: MusicFile) {
    try {
      const result = await invoke('load_and_play', { path: song.path })
      if (result !== null) {
        errorState.setError(String(result))
        return
      }
      playerState.isLoaded = true
      playerState.isPlaying = true
      errorState.clearError()
    } catch (e) {
      errorState.setError(String(e))
    }
  }
</script>

<div class="h-full overflow-auto p-2 bg-gray-300 dark:bg-gray-800 rounded-lg">
  {#each playerState.songList as song}
    <button
      onclick={() => play(song)}
      class="bg-purple-400 dark:bg-purple-600 hover:bg-purple-500 rounded-lg p-2 my-1 w-full cursor-pointer text-left"
    >
      {song.name}
    </button>
  {/each}
</div>
