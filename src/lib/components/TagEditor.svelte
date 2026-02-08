<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined}
    <div class="flex-1 overflow-auto neo-scrollbar">
        {#each tagEditorState.tagFields as tagField, index}
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
                      {tagEditorState.isTagSupported(tagField.tagName)
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
              {/each}
    </div>

    <div class="mt-4 flex flex-col gap-2">
      <div class="flex gap-2 justify-end">
        <button
          onclick={() => tagEditorState.resetTags()}
          class="neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600"
        >
          <RotateCcw size={20} />
        </button>

        <button
          onclick={() => tagEditorState.applyTags()}
          disabled={tagEditorState.isSaving}
          class="neo-raised-sm p-2 rounded-lg transition-all
                 bg-linear-to-r from-purple-700 to-violet-700 hover:from-purple-600 hover:to-violet-600
                 text-white disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Save size={20} />
        </button>
      </div>

      {#if tagEditorState.saveMessage}
        <div
          class="text-center text-sm"
          class:text-green-600={tagEditorState.saveMessage.startsWith('✓')}
          class:text-red-600={tagEditorState.saveMessage.startsWith('Error')}
        >
          {tagEditorState.saveMessage}
        </div>
      {/if}
    </div>
  {/if}
</div>

<script lang="ts">
  import { playerState } from '$lib/stores/player.svelte'
  import { useTagEditorState } from '$lib/stores/tagEditor.svelte'
  import { RotateCcw, Save, Trash, Plus } from '@lucide/svelte'

  const song = $derived(playerState.currentSong)
  const tags = $derived(song?.tags)
  const tagEditorState = useTagEditorState()

  // Update local state when song changes
  $effect(() => {
    tagEditorState.resetTags()
  })
</script>
