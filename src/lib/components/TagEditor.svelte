<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined}
    <div class="flex-1 overflow-auto">
      <table class="w-full">
        <tbody>
          {#each tagFields as field}
            <tr>
              <td class="font-semibold pr-4 py-1">{field.label}</td>
              <td class="py-1">
                <div
                  class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2"
                >
                  <input
                    type={field.type}
                    bind:value={editedTags[field.key]}
                    class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                  />
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
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
  import { playerState, type Tags } from '$lib/stores/player.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { RotateCcw, Save } from '@lucide/svelte'
  import { transform } from 'lodash'

  const song = $derived(playerState.currentSong)
  const tags = $derived(song?.tags)

  const tagFields: { key: keyof Tags; label: string; type: 'text' | 'number' }[] = [
    { key: 'title', label: 'Title', type: 'text' },
    { key: 'artist', label: 'Artist', type: 'text' },
    { key: 'album_artist', label: 'Album Artist', type: 'text' },
    { key: 'album', label: 'Album', type: 'text' },
    { key: 'date', label: 'Date', type: 'text' },
    { key: 'genre', label: 'Genre', type: 'text' },
    { key: 'mood', label: 'Mood', type: 'text' },
    { key: 'track_number', label: 'Track Number', type: 'number' }
  ]

  let editedTags = $state<Record<string, string | number>>({})

  let isSaving = $state(false)
  let saveMessage = $state('')

  // Update local state when song changes
  $effect(() => {
    if (tags) {
      resetTags()
    }
  })

  function resetTags() {
    if (tags) {
      editedTags = Object.fromEntries(tagFields.map((field) => [field.key, tags[field.key] ?? '']))
    }
  }

  function toTags(editedTags: Record<string, string | number>): Tags {
    return transform(
      tagFields,
      (result, field) => {
        result[field.key] = editedTags[field.key] ?? null
      },
      {} as any
    ) as Tags
  }

  async function applyTags() {
    if (!song) return

    isSaving = true
    saveMessage = ''

    try {
      const updatedTags = toTags(editedTags)

      await invoke('write_tags', {
        path: song.path,
        tags: updatedTags
      })

      // Update the song's tags in playerState
      if (playerState.currentSong) {
        playerState.currentSong.tags = updatedTags
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
