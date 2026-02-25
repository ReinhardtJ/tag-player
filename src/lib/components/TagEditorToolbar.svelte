<div
  class="p-2 bg-neutral-800 rounded-2xl flex items-center gap-2 neo-raised-sm justify-between"
>
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
  import { Plus, RotateCcw, Save } from '@lucide/svelte'
  import SortByToolbar from './SortByToolbar.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useTagEditorStore } from '$lib/stores/tagEditorStore.svelte'
  import { useAddedTagStore } from '$lib/stores/addedTagStore.svelte.ts'

  const playerStore = usePlayerStore()
  const tagEditorStore = useTagEditorStore()
  const addedTagStore = useAddedTagStore()
</script>
