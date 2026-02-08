<button
  onclick={togglePlayback}
  class="neo-raised bg-linear-to-br from-purple-700 to-violet-700 rounded-full p-3 hover:from-purple-600 hover:to-violet-600"
>
  {#if playerStore.isPlaying}
    <Pause size={18} fill="currentColor" />
  {:else}
    <Play size={18} fill="currentColor" />
  {/if}
</button>

<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Play, Pause } from '@lucide/svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useErrorStore } from '$lib/stores/errorStore.svelte'

  const errorStore = useErrorStore()
  const playerStore = usePlayerStore()

  async function togglePlayback() {
    const result = await invoke('toggle_playback')
    if (result !== null) {
      errorStore.addError(String(result))
      return
    }
    playerStore.isPlaying = !playerStore.isPlaying
  }
</script>
