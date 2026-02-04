<div class="h-full gradient-border rounded-3xl p-2 flex flex-col">
  {#if tags !== undefined}
    <div class="flex-1 overflow-auto neo-scrollbar">
      <DraggableList
        items={tagFields}
        onReorder={handleReorder}
        keyFn={(field, index) => field.tagName + index}
      >
        {#snippet children(tagField: TagField, index: number)}
          {@const isNavidrome = isNavidromeTag(tagField.tagName)}
          <div class="grid grid-cols-[auto_1fr] gap-4">
            <div
              class="font-semibold py-1 flex items-center gap-2 {isNavidrome
                ? 'text-purple-700 dark:text-purple-400'
                : ''}"
            >
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
                class="inset-shadow-sm inset-shadow-neutral-800 dark:bg-neutral-700 rounded-lg px-3 py-2 flex gap-2 bg-transparent outline-none w-full text-gray-900 dark:text-white"
                placeholder="Tag name"
              />
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
    tagName: string
    tagValue: string
  }

  // Navidrome-relevant tags (highlighted)
  const navidromeTags = [
    'Title',
    'TitleSort',
    'Artist',
    'ArtistSort',
    'Artists',
    'ArtistsSort',
    'Arranger',
    'Composer',
    'ComposerSort',
    'Lyricist',
    'LyricistSort',
    'Conductor',
    'Director',
    'DjMixer',
    'Mixer',
    'Engineer',
    'Producer',
    'Remixer',
    'AlbumArtist',
    'AlbumArtistSort',
    'AlbumArtists',
    'AlbumArtistsSort',
    'Album',
    'AlbumSort',
    'AlbumVersion',
    'Genre',
    'Mood',
    'FlagCompilation',
    'TrackNumber',
    'TrackTotal',
    'DiscNumber',
    'DiscTotal',
    'DiscSubtitle',
    'Bpm',
    'Lyrics',
    'Comment',
    'OriginalDate',
    'RecordingDate',
    'ReleaseDate',
    'CatalogNumber',
    'MusicBrainz_ArtistId',
    'MusicBrainz_RecordingId',
    'MusicBrainz_TrackId',
    'MusicBrainz_AlbumArtistId',
    'MusicBrainz_AlbumId',
    'MusicBrainz_ReleaseGroupId',
    'MusicBrainz_ComposerId',
    'MusicBrainz_LyricistId',
    'MusicBrainz_DirectorId',
    'MusicBrainz_ProducerId',
    'MusicBrainz_EngineerId',
    'MusicBrainz_MixerId',
    'MusicBrainz_RemixerId',
    'MusicBrainz_DjMixerId',
    'MusicBrainz_ConductorId',
    'MusicBrainz_ArrangerId',
    'ReleaseType',
    'ReplayGain_Album_Gain',
    'ReplayGain_Album_Peak',
    'ReplayGain_Track_Gain',
    'ReplayGain_Track_Peak',
    'R128_Album_Gain',
    'R128_Track_Gain',
    'Performer',
    'MusicBrainz_PerformerId',
    'ExplicitStatus'
  ]

  let tagFields = $state<TagField[]>([])

  let isSaving = $state(false)
  let saveMessage = $state('')

  function removeTag(index: number) {
    tagFields.splice(index, 1)
  }

  function isNavidromeTag(tagName: string): boolean {
    return navidromeTags.some((tag) => tag.toUpperCase() === tagName.toUpperCase())
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

    // If renamed to a navidrome tag, reorder to match navidrome order
    if (isNavidromeTag(trimmedName)) {
      const navidromeIndex = navidromeTags.findIndex(
        (t) => t.toUpperCase() === trimmedName.toUpperCase()
      )
      const tag = tagFields.splice(index, 1)[0]

      // Find where to insert among navidrome tags
      let insertPos = 0
      for (let i = 0; i < tagFields.length; i++) {
        if (isNavidromeTag(tagFields[i].tagName)) {
          const currentIndex = navidromeTags.findIndex(
            (t) => t.toUpperCase() === tagFields[i].tagName.toUpperCase()
          )
          if (currentIndex > navidromeIndex) break
          insertPos = i + 1
        } else {
          break
        }
      }
      tagFields.splice(insertPos, 0, tag)
    }
  }

  function addTagBelow(index: number) {
    tagFields.splice(index + 1, 0, { tagName: '', tagValue: '' })
  }

  function handleReorder(newItems: TagField[]) {
    tagFields = newItems
  }

  // Update local state when song changes
  $effect(() => {
    resetTags()
  })

  function resetTags() {
    if (!tags) {
      tagFields = []
      return
    }

    const newTagFields: TagField[] = []

    // First, add navidrome tags in order
    for (const tagName of navidromeTags) {
      const value = tags.get(tagName)
      if (value !== undefined) {
        newTagFields.push({
          tagName,
          tagValue: value || ''
        })
      }
    }

    // Then add other tags
    for (const [tagName, value] of tags.entries()) {
      if (!newTagFields.some((f) => f.tagName === tagName)) {
        newTagFields.push({
          tagName,
          tagValue: value || ''
        })
      }
    }

    tagFields = newTagFields
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
