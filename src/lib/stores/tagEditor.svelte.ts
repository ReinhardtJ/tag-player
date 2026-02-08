import { invoke } from '@tauri-apps/api/core'
import { SvelteMap } from 'svelte/reactivity'
import { playerState } from './player.svelte'

export interface TagField {
  id: string
  tagName: string
  tagValue: string
}

class TagEditorState {
  tagFields = $state<TagField[]>([])
  isSaving = $state(false)
  saveMessage = $state('')
  supportedTagsList = $state<string[]>([])

  tagsNotYetUsed = $derived(
    this.supportedTagsList.filter(
      (tag) => !this.tagFields.some((field) => field.tagName.toUpperCase() === tag.toUpperCase())
    )
  )

  constructor() {
    $effect(() => {
      invoke<string[]>('get_supported_tags').then((tags) => {
        this.supportedTagsList = tags
      })
    })
  }

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

    // Check if new name already exists
    if (
      this.tagFields.some(
        (f, i) => i !== index && f.tagName.toUpperCase() === trimmedName.toUpperCase()
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
    const tags = playerState.currentSong?.tags
    if (!tags) {
      this.tagFields = []
      return
    }

    // Wait for supported tags to be loaded
    if (this.supportedTagsList.length === 0) {
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
      if (this.isTagSupported(tagName)) {
        supportedFields.push(field)
      } else {
        otherFields.push(field)
      }
    }

    // Combine: supported tags first, then others
    this.tagFields = [...supportedFields, ...otherFields]
  }

  async applyTags() {
    const song = playerState.currentSong
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

      if (playerState.currentSong) {
        playerState.currentSong.tags = tagsMap
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

export const useTagEditorState = () => new TagEditorState()