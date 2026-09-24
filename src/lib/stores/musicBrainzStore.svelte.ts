import { invoke } from '@tauri-apps/api/core'
import { SvelteMap } from 'svelte/reactivity'
import type { Song } from './playerTypes.ts'
import type { MusicBrainzRecording } from './musicBrainzTypes.ts'
import { type TagEditorStore, useTagEditorStore } from './tagEditorStore.svelte.ts'

export class MusicBrainzStore {
  isSearching = $state(false)
  results = $state<MusicBrainzRecording[]>([])
  searchError = $state('')
  applyError = $state('')

  private tagEditorStore: TagEditorStore = useTagEditorStore()

  async search(song: Song) {
    this.isSearching = true
    this.results = []
    this.searchError = ''
    this.applyError = ''

    try {
      this.results = await invoke<MusicBrainzRecording[]>('search_musicbrainz', { song })
    } catch (error) {
      this.searchError = String(error)
    } finally {
      this.isSearching = false
    }
  }

  // Applies the tags of a selected match to the tag editor only (no file write),
  // as if the user had entered them by hand. Returns whether it succeeded.
  async applyRecording(recording: MusicBrainzRecording): Promise<boolean> {
    this.applyError = ''

    try {
      const tags = await invoke<Record<string, string>>('musicbrainz_recording_to_tags', {
        recording
      })
      this.tagEditorStore.applyTags(new SvelteMap(Object.entries(tags)))
      return true
    } catch (error) {
      this.applyError = String(error)
      return false
    }
  }
}

let musicBrainzStore: MusicBrainzStore | undefined = undefined

export function useMusicBrainzStore() {
  if (musicBrainzStore === undefined) {
    musicBrainzStore = new MusicBrainzStore()
  }
  return musicBrainzStore
}
