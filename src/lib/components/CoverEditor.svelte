<div class="grid grid-cols-[auto_1fr_2fr_auto_auto] items-center gap-2">
  <!-- Status Dot -->
  <div class="w-1.5 h-1.5 rounded-full self-center {statusDotClass}"></div>

  <!-- Cover Preview -->
  <div
    class="w-14 h-14 rounded-lg overflow-hidden flex items-center justify-center inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700"
  >
    {#if coverPreviewSrc}
      <img src={coverPreviewSrc} alt="Cover" class="w-full h-full object-cover" />
    {:else}
      <Music size={16} class="text-gray-500 dark:text-gray-400" />
    {/if}
  </div>

  <!-- Cover File Picker -->
  <button
    type="button"
    onclick={chooseCover}
    class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 w-full text-left cursor-pointer text-gray-900 dark:text-white"
  >
    <span class="truncate">{coverFileName}</span>
  </button>

  <!-- Remove / Restore -->
  {#if tagEditorStore.isCoverRemoved}
    <button
      onclick={() => tagEditorStore.restoreCover()}
      class="hover:cursor-pointer dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
    >
      <RotateCcw size={16} />
    </button>
  {:else}
    <button
      onclick={() => tagEditorStore.removeCover()}
      class="hover:cursor-pointer dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
    >
      <X size={16} />
    </button>
  {/if}

  <!-- Decorative pin: the cover is always shown at the top -->
  <div class="text-gray-400 dark:text-neutral-600" title="Cover is always shown at the top">
    <Pin size={16} />
  </div>
</div>

<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core'
  import { open } from '@tauri-apps/plugin-dialog'
  import { last } from 'lodash'
  import { Music, Pin, RotateCcw, X } from '@lucide/svelte'
  import { useTagEditorStore, TagStatus } from '$lib/stores/tagEditorStore.svelte.ts'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte.ts'

  const tagEditorStore = useTagEditorStore()
  const playerStore = usePlayerStore()

  const coverPreviewSrc = $derived(
    tagEditorStore.pendingCoverPath
      ? convertFileSrc(tagEditorStore.pendingCoverPath)
      : tagEditorStore.isCoverRemoved
        ? null
        : playerStore.currentSong?.cover_base64 ?? null
  )

  const coverFileName = $derived.by(() => {
    if (tagEditorStore.pendingCoverPath)
      return last(tagEditorStore.pendingCoverPath.split(/[\\/]/)) ?? tagEditorStore.pendingCoverPath

    if (tagEditorStore.isCoverRemoved)
      return 'No cover'

    return playerStore.currentSong?.cover_base64 ? 'Current cover' : 'Choose image…'
  })

  // A picked cover is "edited" when the song already had one, otherwise "added".
  const coverStatus = $derived(
    tagEditorStore.isCoverRemoved
      ? TagStatus.REMOVED
      : tagEditorStore.pendingCoverPath
        ? playerStore.currentSong?.cover_base64
          ? TagStatus.EDITED
          : TagStatus.ADDED
        : TagStatus.UNCHANGED
  )

  const statusDotClass = $derived(
    coverStatus === TagStatus.EDITED
      ? 'bg-violet-500 shadow-[0_0_10px_1px_--theme(--color-violet-500)]'
      : coverStatus === TagStatus.REMOVED
        ? 'bg-red-500 shadow-[0_0_10px_1px_--theme(--color-red-500)]'
        : coverStatus === TagStatus.ADDED
          ? 'bg-green-500 shadow-[0_0_10px_1px_--theme(--color-green-500)]'
          : ''
  )

  async function chooseCover() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        { name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'tif', 'tiff'] }
      ]
    })

    if (typeof selected === 'string') 
      tagEditorStore.setPendingCover(selected)
  }
</script>
