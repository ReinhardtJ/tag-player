<div class="grid grid-cols-[1fr_2fr] gap-4">
  <div class="flex items-center gap-2">
    <button
      onclick={() => tagEditorState.addTagBelow(index)}
      class="hover:text-gray-400 dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
    >
      <Plus size={16} />
    </button>
    <input
      type="text"
      bind:value={tagField.tagName}
      oninput={(e) => tagEditorState.renameTag(index, e.currentTarget.value)}
      list="supported-tags"
      class="inset-shadow-sm inset-shadow-neutral-800 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 bg-transparent outline-none w-full
      {isEssentialTag(tagField.tagName)
        ? 'text-yellow-600 dark:text-yellow-400 dark:font-semibold'
        : tagEditorState.isTagSupported(tagField.tagName)
          ? 'text-purple-700 dark:text-purple-400 dark:font-semibold'
          : ''}
      "
      placeholder="Tag name"
    />
    <datalist id="supported-tags">
      {#each tagEditorState.tagsNotYetUsed as tag}
        <option value={tag}>{tag}</option>
      {/each}
    </datalist>
  </div>
  <div class="py-1">
    <div
      class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2"
    >
      <input
        type="text"
        bind:value={tagField.tagValue}
        oninput={(e) => tagEditorState.updateTagValue(index, e.currentTarget.value)}
        class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
      />
      <button
        onclick={() => tagEditorState.removeTag(index)}
        class="hover:text-red-600 dark:hover:text-red-400 text-gray-600 dark:text-gray-400"
      >
        <Trash size={16} />
      </button>
    </div>
  </div>
</div>

<script lang="ts">
  import { Plus, Trash } from '@lucide/svelte'
  import { ESSENTIAL_TAGS, type TagField } from '$lib/stores/tagEditorStore.svelte'

  interface Props {
    tagField: TagField
    index: number
    tagEditorState: any // Using any for now to avoid circular or complex imports if not needed, but we can type it better
  }

  let { tagField, index, tagEditorState }: Props = $props()

  function isEssentialTag(tagName: string): boolean {
    return ESSENTIAL_TAGS.some((tag) => tag.toLowerCase() === tagName.toLowerCase())
  }
</script>
