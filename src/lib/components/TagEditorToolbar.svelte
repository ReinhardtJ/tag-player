<div class="p-2 bg-neutral-800 rounded-2xl flex items-center gap-2 neo-raised-sm justify-between">
  <SortByToolbar
    bind:sortOrder={tagEditorStore.sortOrder}
    bind:sortBy={tagEditorStore.sortBy}
    sortOptions={tagEditorStore.sortByOptions}
  ></SortByToolbar>
  <div class="flex flex-row gap-2">
    <!-- Add Button -->
    <button onclick={() => addedTagStore.addTag()} class="btn-secondary">
      <Plus size={16} />
    </button>
    <!-- Reset Button  -->
    <button
      onclick={() => tagEditorStore.setTags(playerStore.currentSong?.tags)}
      class="btn-secondary"
    >
      <RotateCcw size={16} />
    </button>
    <!-- Search Button  -->
    <button onclick={searchOnWeb} class="btn-secondary">
      <Globe size={16} />
    </button>
    <!-- Save Button  -->
    <button
      onclick={() => tagEditorStore.saveTags(playerStore.currentSong)}
      disabled={tagEditorStore.isSaving}
      class="btn-primary"
    >
      <Save size={16} />
    </button>
  </div>
</div>

<script lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener'
  import { Plus, RotateCcw, Save, Globe } from '@lucide/svelte'
  import SortByToolbar from './SortByToolbar.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useTagEditorStore } from '$lib/stores/tagEditorStore.svelte'
  import { useAddedTagStore } from '$lib/stores/addedTagStore.svelte.ts'
  import { some } from 'lodash'

  const playerStore = usePlayerStore()
  const tagEditorStore = useTagEditorStore()
  const addedTagStore = useAddedTagStore()

  function searchOnWeb() {
    const song = playerStore.currentSong
    if (!song)
      return

    const searchTags = [
      song.tags.get('TrackTitle') ?? '',
      song.tags.get('TrackArtist') ?? '',
      song.tags.get('AlbumTitle') ?? '',
    ]

    let searchQuery = some(searchTags)
      ? searchTags.join(' ')
      : song.name.replace(/\.[^.]+$/, '')

    const url = `https://kagi.com/search?q=${encodeURIComponent(searchQuery + ' MusicBrainz')}`
    openUrl(url)
  }
</script>
