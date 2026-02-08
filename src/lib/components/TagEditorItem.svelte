<div class="grid grid-cols-[auto_1fr_2fr_auto_auto] gap-2 mx-4">
    <!-- Add Button -->
  <button
    onclick={() => tagEditorStore.addTagBelow(index)}
    class="hover:text-gray-400 dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
  >
    <Plus size={16} />
  </button>
  <!-- Tag Name Input -->
  <div class="flex items-center gap-2">
    <input
      type="text"
      bind:value={tagField.tagName}
      oninput={(e) => tagEditorStore.renameTag(index, e.currentTarget.value)}
      list="supported-tags"
      class="inset-shadow-sm inset-shadow-neutral-800 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 bg-transparent outline-none w-full
      {tagEditorStore.isPinnedTag(tagField.tagName)
        ? 'text-yellow-600 dark:text-yellow-400 dark:font-semibold'
        : tagEditorStore.isTagSupported(tagField.tagName)
          ? 'text-purple-700 dark:text-purple-400 dark:font-semibold'
          : ''}
      "
      placeholder="Tag name"
    />
    <datalist id="supported-tags">
      {#each tagEditorStore.tagsNotYetUsed as tag}
        <option value={tag}>{tag}</option>
      {/each}
    </datalist>
  </div>
  <!-- Tag Value Input -->
  <div class="py-1">
    <div
      class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2"
    >
      <input
        type="text"
        bind:value={tagField.tagValue}
        oninput={(e) => tagEditorStore.updateTagValue(index, e.currentTarget.value)}
        class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
      />
    </div>
  </div>
  <!-- Delete Button -->
  <button
    onclick={() => tagEditorStore.removeTag(index)}
    class="hover:cursor-pointer hover:text-red-600 dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
  >
    <Trash size={16} />
  </button>
  <button
      onclick={() => tagEditorStore.togglePin(tagField.tagName)}
      class="text-gray-600 dark:text-gray-400 hover:cursor-pointer hover:text-gray-300"
  >
      {#if tagEditorStore.isPinnedTag(tagField.tagName)}

          <Pin size={16}></Pin>
        {:else}
      <PinOff size={16}></PinOff>
      {/if}

  </button>
</div>

<script lang="ts">
  import { Pin, PinOff, Plus, Trash } from '@lucide/svelte'
  import { useTagEditorStore, type TagField } from '$lib/stores/tagEditorStore.svelte'

  let {
    tagField,
    index
  }: {
    tagField: TagField
    index: number
  } = $props()

  const tagEditorStore = useTagEditorStore()
</script>
