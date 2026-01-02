<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { open } from '@tauri-apps/plugin-dialog'
  import { FolderOpen } from '@lucide/svelte'
  import { playerState } from '$lib/stores/player.svelte'

  async function selectFolder() {
    const selectedFolder = await open({ directory: true })
    playerState.library = await invoke('get_music_library', { path: selectedFolder })
  }
</script>

<button
    onclick={selectFolder}
    class="
      flex items-center gap-2
      bg-gradient-to-br from-purple-700 to-violet-700 rounded-lg p-2
      hover:from-purple-600 hover:to-violet-600
      neo-raised"
    >
  <FolderOpen size={20}/>
  <span>Open Folder</span>
</button>
