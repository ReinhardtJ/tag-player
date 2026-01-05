<script lang="ts">
  import { playerState } from '$lib/stores/player.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { RotateCcw, Save } from '@lucide/svelte'

  const song = $derived(playerState.currentSong)
  const tags = $derived(song?.tags)

  // Local editable state
  let editedTitle = $state('')
  let editedArtist = $state('')
  let editedAlbumArtist = $state('')
  let editedAlbum = $state('')
  let editedDate = $state('')
  let editedGenre = $state('')
  let editedMood = $state('')
  let editedTrackNumber = $state('')

  let isSaving = $state(false)
  let saveMessage = $state('')

  // Update local state when song changes
  $effect(() => {
    if (tags) {
      editedTitle = tags.title ?? ''
      editedArtist = tags.artist ?? ''
      editedAlbumArtist = tags.album_artist ?? ''
      editedAlbum = tags.album ?? ''
      editedDate = tags.date ?? ''
      editedGenre = tags.genre ?? ''
      editedMood = tags.mood ?? ''
      editedTrackNumber = tags.track_number?.toString() ?? ''
    }
  })

  function resetTags() {
    if (tags) {
      editedTitle = tags.title ?? ''
      editedArtist = tags.artist ?? ''
      editedAlbumArtist = tags.album_artist ?? ''
      editedAlbum = tags.album ?? ''
      editedDate = tags.date ?? ''
      editedGenre = tags.genre ?? ''
      editedMood = tags.mood ?? ''
      editedTrackNumber = tags.track_number?.toString() ?? ''
    }
  }

  async function applyTags() {
    if (!song) return

    isSaving = true
    saveMessage = ''

    try {
      const updatedTags = {
        title: editedTitle || null,
        artist: editedArtist || null,
        album_artist: editedAlbumArtist || null,
        album: editedAlbum || null,
        date: editedDate || null,
        genre: editedGenre || null,
        mood: editedMood || null,
        track_number: editedTrackNumber ? parseInt(editedTrackNumber) : null
      }

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

<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined }
    <div class="flex-1 overflow-auto">
      <table class="w-full">
        <tbody>
          <tr>
            <td class="font-semibold pr-4 py-1">Title</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedTitle}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Artist</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedArtist}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Album Artist</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedAlbumArtist}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Album</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedAlbum}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Date</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedDate}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Genre</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedGenre}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Mood</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="text" 
                  bind:value={editedMood}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
          <tr>
            <td class="font-semibold pr-4 py-1">Track Number</td>
            <td class="py-1">
              <div class="inset-shadow-sm inset-shadow-neutral-800 bg-gray-300 dark:bg-neutral-700 rounded-lg px-3 py-2">
                <input 
                  type="number" 
                  bind:value={editedTrackNumber}
                  class="bg-transparent outline-none w-full text-gray-900 dark:text-white"
                />
              </div>
            </td>
          </tr>
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
        <div class="text-center text-sm" class:text-green-600={saveMessage.startsWith('✓')} class:text-red-600={saveMessage.startsWith('Error')}>
          {saveMessage}
        </div>
      {/if}
    </div>
  {/if}
</div>
