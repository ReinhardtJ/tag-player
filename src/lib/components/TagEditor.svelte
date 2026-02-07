<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined}
    <div class="flex-1 overflow-auto neo-scrollbar">
      <DraggableList items={tagFields} onReorder={handleReorder} keyFn={(field, index) => field.id}>
        {#snippet children(tagField: TagField, index: number)}
          <div class="grid grid-cols-[1fr_2fr] gap-4">
            <div class="flex items-center gap-2">
              <button
                onclick={() => addTagBelow(index)}
                class="hover:text-gray-400 dark:hover:text-gray-300 text-gray-600 dark:text-gray-400"
              >
                <Plus size={16} />
              </button>
              <input
                type="text"
                bind:value={tagField.tagName}
                oninput={(e) => renameTag(index, e.currentTarget.value)}
                list="supported-tags"
                class="inset-shadow-sm inset-shadow-neutral-800 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 bg-transparent outline-none w-full
                {isTagSupported(tagField.tagName)
                  ? 'text-purple-700 dark:text-purple-400 dark:font-semibold'
                  : ''}
                "
                placeholder="Tag name"
              />
              <datalist id="supported-tags">
                {#each supportedTagsList as tag}
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
                  oninput={(e) => (tagFields[index].tagValue = e.currentTarget.value)}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
                <button
                  onclick={() => removeTag(index)}
                  class="hover:text-red-600 dark:hover:text-red-400 text-gray-600 dark:text-gray-400"
                >
                  <Trash size={16} />
                </button>
              </div>
            </div>
          </div>
        {/snippet}
      </DraggableList>
    </div>

    <div class="mt-4 flex flex-col gap-2">
      <div class="flex gap-2 justify-end">
        <button
          onclick={resetTags}
          class="neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600"
        >
          <RotateCcw size={20} />
        </button>

        <button
          onclick={applyTags}
          disabled={isSaving}
          class="neo-raised-sm p-2 rounded-lg transition-all
                 bg-linear-to-r from-purple-700 to-violet-700 hover:from-purple-600 hover:to-violet-600
                 text-white disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Save size={20} />
        </button>
      </div>

      {#if saveMessage}
        <div
          class="text-center text-sm"
          class:text-green-600={saveMessage.startsWith('✓')}
          class:text-red-600={saveMessage.startsWith('Error')}
        >
          {saveMessage}
        </div>
      {/if}
    </div>
  {/if}
</div>

<script lang="ts">
  import { playerState } from '$lib/stores/player.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { RotateCcw, Save, Trash, Plus } from '@lucide/svelte'
  import { SvelteMap } from 'svelte/reactivity'
  import DraggableList from './DraggableList.svelte'

  const song = $derived(playerState.currentSong)
  const tags = $derived(song?.tags)

  interface TagField {
    id: string // needed for draggable list
    tagName: string
    tagValue: string
  }

  let tagFields = $state<TagField[]>([])

  let isSaving = $state(false)
  let saveMessage = $state('')
  let supportedTagsList = $state<string[]>([])

  // Load supported tags on mount
  $effect(() => {
    invoke<string[]>('get_supported_tags').then((tags) => {
      supportedTagsList = tags
    })
  })

  function isTagSupported(tagName: string): boolean {
    return supportedTagsList.includes(tagName)
  }

  function removeTag(index: number) {
    tagFields.splice(index, 1)
  }

  function renameTag(index: number, newName: string) {
    const trimmedName = newName.trim()
    if (!trimmedName) {
      return
    }

    const oldName = tagFields[index].tagName
    if (oldName === trimmedName) {
      return
    }

    // Check if new name already exists
    if (
      tagFields.some((f, i) => i !== index && f.tagName.toUpperCase() === trimmedName.toUpperCase())
    ) {
      return
    }

    tagFields[index].tagName = trimmedName

    // No automatic reordering - user can manually reorder tags
  }

  function addTagBelow(index: number) {
    tagFields.splice(index + 1, 0, { id: crypto.randomUUID(), tagName: '', tagValue: '' })
  }

  function handleReorder(newItems: TagField[]) {
    tagFields = newItems
  }



  // Update local state when song changes
  $effect(() => {
    resetTags()
  })

  async function resetTags() {
    if (!tags) {
      tagFields = []
      return
    }

    // Wait for supported tags to be loaded
    if (supportedTagsList.length === 0) {
      return
    }

    // Separate tags into supported and other
    const supportedFields: TagField[] = []
    const otherFields: TagField[] = []

    for (const [tagName, value] of tags.entries()) {
      const field = {
        id: crypto.randomUUID(),
        tagName,
        tagValue: value || ''
      }
      if (isTagSupported(tagName)) {
        supportedFields.push(field)
      } else {
        otherFields.push(field)
      }
    }

    // Combine: supported tags first, then others
    tagFields = [...supportedFields, ...otherFields]
  }

  async function applyTags() {
    if (!song) return

    isSaving = true
    saveMessage = ''

    try {
      const tagsMap = new SvelteMap<string, string>(
        tagFields
          .filter((field) => field.tagName.trim())
          .map((field) => [field.tagName, field.tagValue])
      )

      await invoke('write_tags', {
        path: song.path,
        tags: tagsMap
      })

      if (playerState.currentSong) {
        playerState.currentSong.tags = tagsMap
      }

      saveMessage = '✓ Tags saved successfully'
      setTimeout(() => {
        saveMessage = ''
      }, 3000)
    } catch (error) {
      saveMessage = `Error: ${error}`
    } finally {
      isSaving = false
    }
  }
</script>
