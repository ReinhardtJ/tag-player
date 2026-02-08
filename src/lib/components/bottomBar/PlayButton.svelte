<button
  onclick={togglePlayback}
  class="neo-raised bg-linear-to-br from-purple-700 to-violet-700 rounded-full p-3 hover:from-purple-600 hover:to-violet-600"
>
  {#if playerState.isPlaying}
    <Pause size={18} fill="currentColor" />
  {:else}
    <Play size={18} fill="currentColor" />
  {/if}
</button>

<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Play, Pause } from '@lucide/svelte'
  import { playerState } from '$lib/stores/player.svelte'
  import { errorState } from '$lib/stores/error.svelte'

  async function togglePlayback() {
    const result = await invoke('toggle_playback')
    if (result !== null) {
      errorState.addError(String(result))
      return
    }
    playerState.isPlaying = !playerState.isPlaying
  }
</script>
