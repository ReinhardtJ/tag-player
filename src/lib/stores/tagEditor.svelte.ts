import { invoke } from '@tauri-apps/api/core'
import { SvelteMap } from 'svelte/reactivity'
import { partition, sortBy } from 'lodash'
import { usePlayerState, type PlayerState } from './player.svelte'

export interface TagField {
  id: string
  tagName: string
  tagValue: string
}

export const PRIORITY_TAGS = [
  'TrackTitle',
  'TrackArtist',
  'AlbumTitle',
  'AlbumArtist',
  'RecordingDate',
  'Genre',
  'Mood'
]

export function sortTagFieldsByRelevance(
  tagFields: TagField[],
  priorityTags: string[],
  sortAscending: boolean
): TagField[] {
  // Partition fields into priority and other
  const [priorityFields, otherFields] = partition(tagFields, (field) =>
    priorityTags.some((tag) => tag.toLowerCase() === field.tagName.toLowerCase())
  )

  // Sort priority fields by their order in priorityTags array
  const sortedPriorityFields = sortBy(priorityFields, (field) =>
    priorityTags.findIndex((tag) => tag.toLowerCase() === field.tagName.toLowerCase())
  )

  if (sortAscending) {
    return [...otherFields, ...sortedPriorityFields.reverse()]
  }
  return [...sortedPriorityFields, ...otherFields]
}

class TagEditorState {
  tagFields = $state<TagField[]>([])
  isSaving = $state(false)
  saveMessage = $state('')
  supportedTagsList = $state<string[]>([])
  sortByOptions = ['relevance']
  sortBy = $state('relevance')
  sortAscending = $state(false)
  private playerState: PlayerState

  tagsNotYetUsed = $derived(
    this.supportedTagsList.filter(
      (tag) => !this.tagFields.some((field) => field.tagName.toLowerCase() === tag.toLowerCase())
    )
  )

  constructor(usePlayerState: () => PlayerState) {
    this.playerState = usePlayerState()
    $effect(() => {
      invoke<string[]>('get_supported_tags').then((tags) => {
        this.supportedTagsList = tags
      })
    })
  }

  sortedTagFields = $derived.by(() => {
    if (this.sortBy === 'relevance') {
      return sortTagFieldsByRelevance(this.tagFields, PRIORITY_TAGS, this.sortAscending)
    }
    return this.tagFields
  })

  isTagSupported(tagName: string): boolean {
    return this.supportedTagsList.includes(tagName)
  }

  removeTag(index: number) {
    this.tagFields.splice(index, 1)
  }

  renameTag(index: number, newName: string) {
    const trimmedName = newName.trim()
    if (!trimmedName) {
      return
    }

    const oldName = this.tagFields[index].tagName
    if (oldName === trimmedName) {
      return
    }

    if (
      this.tagFields.some(
        (f, i) => i !== index && f.tagName.toLowerCase() === trimmedName.toLowerCase()
      )
    ) {
      return
    }

    this.tagFields[index].tagName = trimmedName
  }

  addTagBelow(index: number) {
    this.tagFields.splice(index + 1, 0, { id: crypto.randomUUID(), tagName: '', tagValue: '' })
  }

  updateTagValue(index: number, value: string) {
    this.tagFields[index].tagValue = value
  }

  resetTags() {
    const tags = this.playerState.currentSong?.tags
    if (!tags) {
      this.tagFields = []
      return
    }

    if (this.supportedTagsList.length === 0) {
      return
    }

    const supportedFields: TagField[] = []
    const otherFields: TagField[] = []

    for (const [tagName, value] of tags.entries()) {
      const field = {
        id: crypto.randomUUID(),
        tagName,
        tagValue: value || ''
      }
      if (this.isTagSupported(tagName)) {
        supportedFields.push(field)
      } else {
        otherFields.push(field)
      }
    }

    this.tagFields = [...supportedFields, ...otherFields]
  }

  async applyTags() {
    const song = this.playerState.currentSong
    if (!song) return

    this.isSaving = true
    this.saveMessage = ''

    try {
      const tagsMap = new SvelteMap<string, string>(
        this.tagFields
          .filter((field) => field.tagName.trim())
          .map((field) => [field.tagName, field.tagValue])
      )

      await invoke('write_tags', {
        path: song.path,
        tags: tagsMap
      })

      if (this.playerState.currentSong) {
        this.playerState.currentSong.tags = tagsMap
      }

      this.saveMessage = '✓ Tags saved successfully'
      setTimeout(() => {
        this.saveMessage = ''
      }, 3000)
    } catch (error) {
      this.saveMessage = `Error: ${error}`
    } finally {
      this.isSaving = false
    }
  }
}

export const useTagEditorState = () => new TagEditorState(usePlayerState)
