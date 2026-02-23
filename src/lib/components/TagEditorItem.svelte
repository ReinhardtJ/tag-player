<div class="grid grid-cols-[auto_1fr_2fr_auto_auto] gap-2">
  <div class="w-2 h-2 rounded-full self-center {
    tagField.status === TagStatus.EDITED ? 'bg-blue-600' :
    tagField.status === TagStatus.REMOVED ? 'bg-red-600' :
    tagField.status === TagStatus.ADDED ? 'bg-green-600' : ''
  }"></div>
  <!-- Tag Name Input -->
  <div class="flex items-center gap-2">
    <input
        type="text"
        value={tagField.tagName}
        oninput={(e) => renameTag(tagField, e.currentTarget.value)}
        list="suggested-tags"
        class="
          inset-shadow-sm inset-shadow-neutral-800 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 bg-transparent outline-none w-full
          {isTagSupported ? 'text-purple-700 dark:text-purple-400 dark:font-semibold': ''}
        "
        placeholder="Tag name"
    />
    <!-- Tag Name DataList -->
    <datalist id="suggested-tags">
      {#each suggestedTags as tag (tag)}
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
          value={tagField.tagValue}
          oninput={(e) => tagEditorStore.updateTagValue(tagField, e.currentTarget.value)}
          class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
      />
    </div>
  </div>
  <!-- Delete Button -->
  {#if tagField.status === TagStatus.REMOVED}
    <button
        onclick={() => tagEditorStore.readdTag(tagField)}
        class="hover:cursor-pointer dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
    >
      <RotateCcw size={16}/>
    </button>
  {:else}
    <button
        onclick={() => tagEditorStore.removeTag(tagField)}
        class="hover:cursor-pointer dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
    >
      <X size={16}/>
    </button>
  {/if}
  <button
      onclick={() => pinnedTagStore.togglePin(tagField.tagName)}
      class="text-gray-600 dark:text-gray-400 hover:cursor-pointer hover:text-gray-300"
  >
    {#if pinnedTagStore.isPinnedTag(tagField.tagName)}
      <Pin size={16} class="text-yellow-400"></Pin>
    {:else}
      <PinOff size={16}></PinOff>
    {/if}
  </button>
</div>
<script lang="ts">
  import { Pin, PinOff, X, RotateCcw } from '@lucide/svelte'
  import { useTagEditorStore, type TagField, TagStatus } from '$lib/stores/tagEditorStore.svelte'
  import { usePinnedTagStore } from '$lib/stores/pinnedTagStore.svelte'

  let {
    tagField,
  }: {
    tagField: TagField
  } = $props()

  const tagEditorStore = useTagEditorStore()
  const pinnedTagStore = usePinnedTagStore()


  const isTagSupported = $derived(tagEditorStore.isTagSupported(tagField.tagName))

  function renameTag(tagField: TagField, newName: string) {
      tagEditorStore.renameTag(tagField, newName)
  }

  const suggestedTags = $derived.by(() => {
    // suggest unused tags that are supported
    const usedTagFieldNames = tagEditorStore.sortedTagFields.map(tf => tf.tagName)
    return tagEditorStore.supportedTagNames
     .filter(name => usedTagFieldNames.includes(name))
  })
</script>
