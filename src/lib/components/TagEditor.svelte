<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined}
    <div class="flex-1 overflow-auto neo-scrollbar">
      <table class="w-full">
        <tbody>
          {#each editedTags as [editableTagKey, _] (editableTagKey)}
            <tr>
              <td class="font-semibold pr-4 py-1">{editableTagKey}</td>
              <td class="py-1">
                <div
                  class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2"
                >
                  <input
                    type="text"
                    bind:value={
                      () => editedTags.get(editableTagKey),
                      (v) => updateTag(editableTagKey, v)
                    }
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
  import { playerState } from '$lib/stores/player.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { RotateCcw, Save } from '@lucide/svelte'
  import { SvelteMap } from 'svelte/reactivity'
  import { onMount } from 'svelte'

  const song = $derived(playerState.currentSong)
  const tags = $derived(song?.tags)

  let editedTags = $state<SvelteMap<string, string>>(new SvelteMap())

  let isSaving = $state(false)
  let saveMessage = $state('')

  function updateTag(key: string, value: string | undefined) {
    if (value) {
      editedTags.set(key, value)
    }
  }

  onMount(() => {
    console.log(`loaded song ${song?.name} with tags ${JSON.stringify(tags)}`)
    console.log(tags?.keys())
  })

  // Update local state when song changes
  $effect(() => {
    console.log(`loaded song ${song?.name} with tags ${JSON.stringify(tags)}`)
    resetTags()
  })

  function resetTags() {
    if (tags) {
      editedTags = new SvelteMap<string, string>(tags);
    } else {
      editedTags = new SvelteMap()
    }
  }
  async function applyTags() {
    if (!song) return

    isSaving = true
    saveMessage = ''

    try {
      await invoke('write_tags', {
        path: song.path,
        tags: editedTags
      })

      // Update the song's tags in playerState
      if (playerState.currentSong) {
        playerState.currentSong.tags = editedTags
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
