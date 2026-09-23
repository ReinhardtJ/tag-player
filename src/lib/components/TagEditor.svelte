<div class="h-full gradient-border rounded-3xl flex flex-col overflow-hidden">
  {#if tags !== undefined}
    <!-- Tag List -->
    <div class="flex-1 overflow-auto neo-scrollbar">
      <div class="sticky top-2 m-2">
        <TagEditorToolbar />
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
          class:text-green-600={tagEditorStore.saveMessage.startsWith('✓')}
          class="text-center text-sm"
          class:text-red-600={tagEditorStore.saveMessage.startsWith('Error')}
        >
          {tagEditorStore.saveMessage}
        </div>
      {/if}
    </div>
  {/if}
</div>

<Dialog />

<script lang="ts">
  import TagEditorItem from './TagEditorItem.svelte'
  import TagEditorToolbar from './TagEditorToolbar.svelte'
  import Dialog from './generic/Dialog.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import { useTagEditorStore } from '$lib/stores/tagEditorStore.svelte'

  const playerStore = usePlayerStore()
  const song = $derived(playerStore.currentSong)
  const tags = $derived(song?.tags)
  const tagEditorStore = useTagEditorStore()
</script>