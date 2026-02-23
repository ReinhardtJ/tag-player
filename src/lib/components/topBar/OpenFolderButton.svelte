<button onclick={selectFolder} class="flex items-center gap-2 btn-primary">
  <FolderOpen size={16} />
  <span>Open Folder</span>
</button>

<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog'
  import { FolderOpen } from '@lucide/svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'

  const playerStore = usePlayerStore()

  async function selectFolder() {
    const selectedFolder = await open({ directory: true })
    if (selectedFolder) {
      await playerStore.loadMusicLibrary(selectedFolder)
    }
  }
</script>
