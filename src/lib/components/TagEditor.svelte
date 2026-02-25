<div class="h-full gradient-border rounded-3xl flex flex-col overflow-hidden">
  {#if tags !== undefined}
    <!-- Tag List -->
    <div class="flex-1 overflow-auto neo-scrollbar">
      <div
        class="sticky top-2 m-2 px-2 py-2 bg-neutral-800 rounded-2xl flex items-center gap-2 neo-raised-sm justify-between"
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
      <div class="p-2">
        {#each tagEditorStore.sortedTagFields as tagField (tagField.id)}
          <TagEditorItem {tagField} />
        {/each}
      </div>
    </div>

    <div class="flex flex-col gap-2">
      {#if tagEditorStore.saveMessage}
        <div
          class="text-center text-sm"
          class:text-green-600={tagEditorStore.saveMessage.startsWith('✓')}
          class:text-red-600={tagEditorStore.saveMessage.startsWith('Error')}
        >
          {tagEditorStore.saveMessage}
        </div>
      {/if}
    </div>
  {/if}
</div>

<script lang="ts">
  import { Plus, RotateCcw, Save } from '@lucide/svelte'
  import TagEditorItem from './TagEditorItem.svelte'
  import SortByToolbar from './SortByToolbar.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useTagEditorStore } from '$lib/stores/tagEditorStore.svelte'
  import { useAddedTagStore } from '$lib/stores/addedTagStore.svelte.ts'

  const playerStore = usePlayerStore()
  const song = $derived(playerStore.currentSong)
  const tags = $derived(song?.tags)
  const tagEditorStore = useTagEditorStore()
  const addedTagStore = useAddedTagStore()
</script>
