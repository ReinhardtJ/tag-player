<div class="h-full gradient-border rounded-3xl flex flex-col">
  {#if tags !== undefined}
    <!-- Tag List -->
    <div class="flex-1 overflow-auto neo-scrollbar">
      <div class="sticky top-2 m-2 px-4 py-2 bg-neutral-800 rounded-2xl flex items-center gap-2 neo-raised-sm w-fit">
        <SortByToolbar
          bind:sortAscending={tagEditorStore.sortAscending}
          bind:sortBy={tagEditorStore.sortBy}
          sortOptions={tagEditorStore.sortByOptions}
        ></SortByToolbar>
      </div>

      {#each tagEditorStore.sortedTagFields as tagField, index}
        <TagEditorItem {tagField} {index} />
      {/each}
    </div>

    <div class="mt-4 flex flex-col gap-2">
      <div class="flex gap-2 justify-end">
        <!-- Reset Button  -->
        <button
          onclick={() => tagEditorStore.resetTags()}
          class="neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600"
        >
          <RotateCcw size={20} />
        </button>
        <!-- Save Button  -->
        <button
          onclick={() => tagEditorStore.applyTags()}
          disabled={tagEditorStore.isSaving}
          class="neo-raised-sm p-2 rounded-lg transition-all
                 bg-linear-to-r from-purple-700 to-violet-700 hover:from-purple-600 hover:to-violet-600
                 text-white disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Save size={20} />
        </button>
      </div>

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
  import { RotateCcw, Save } from '@lucide/svelte'
  import TagEditorItem from './TagEditorItem.svelte'
  import SortByToolbar from './SortByToolbar.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useTagEditorStore } from '$lib/stores/tagEditorStore.svelte'

  const playerStore = usePlayerStore()
  const song = $derived(playerStore.currentSong)
  const tags = $derived(song?.tags)
  const tagEditorStore = useTagEditorStore()

  // Update local state when song changes
  $effect(() => {
    tagEditorStore.resetTags()
  })
</script>
