import { invoke } from '@tauri-apps/api/core'
import { SvelteMap, SvelteSet } from 'svelte/reactivity'
import { type AddedTagStore, useAddedTagStore } from './addedTagStore.svelte.ts'
import { type PinnedTagStore, usePinnedTagStore } from './pinnedTagStore.svelte.ts'
import type { Song } from './playerTypes.ts'
import type { SortOrder } from '$lib/components/SortByToolbar.types.ts'
import { find } from 'lodash'

export enum TagStatus {
  UNCHANGED,
  EDITED,
  ADDED,
  REMOVED
}

export class TagField {
  readonly id: string
  tagName: string
  tagValue: string
  status: TagStatus
  readonly originalName: string
  readonly originalValue: string

  constructor(tagName: string, tagValue: string, status: TagStatus = TagStatus.UNCHANGED) {
    this.id = crypto.randomUUID()

    this.status = $state(status)
    this.tagName = $state(tagName)
    this.tagValue = $state(tagValue)

    this.originalName = tagName
    this.originalValue = tagValue
  }

  updateName(newName: string) {
    if (newName === this.originalName) {
      this.updateStatus(TagStatus.UNCHANGED)
    } else {
      this.updateStatus(TagStatus.EDITED)
    }

    newName = newName.trim()

    this.tagName = newName
  }

  updateValue(newValue: string) {
    if (this.originalValue === newValue) {
      this.updateStatus(TagStatus.UNCHANGED)
    } else {
      this.updateStatus(TagStatus.EDITED)
    }
    this.tagValue = newValue
  }

  reset() {
    this.tagName = this.originalName
    this.tagValue = this.originalValue
    this.updateStatus(TagStatus.UNCHANGED)
  }

  updateStatus(newStatus: TagStatus) {
    // we want to keep "added" tags as added
    if (this.status !== TagStatus.ADDED) {
      this.status = newStatus
    }
  }
}

export function matchesTagName(value: string, tagField: TagField) {
  return value.trim().toLowerCase() === tagField.tagName.trim().toLowerCase()
}

export function sortTagFieldsByRelevance(
  tagFields: TagField[],
  isRelevantCallbacks: ((tf: TagField) => boolean)[],
  sortOrder: SortOrder
): TagField[] {
  const relevantTagFields = []
  const otherTagFields = new SvelteSet(tagFields)

  for (const isRelevant of isRelevantCallbacks) {
    const relevantTagField = find([...otherTagFields], isRelevant)
    if (relevantTagField) {
      relevantTagFields.push(relevantTagField)
      otherTagFields.delete(relevantTagField)
    }
  }

  if (sortOrder === 'desc') {
    return [...relevantTagFields, ...otherTagFields]
  }

  return [...otherTagFields, ...relevantTagFields.reverse()]
}

export class TagEditorStore {
  private tagFields = $state<TagField[]>([])

  isSaving = $state(false)
  saveMessage = $state('')

  // which tags are supported as defined by the backend (see get_supported_tags() in lib.rs)
  supportedTagNames = $state<string[]>([])

  sortByOptions = ['relevance']
  sortBy = $state('relevance')
  sortOrder = $state<SortOrder>('desc')

  private addedTagStore: AddedTagStore = useAddedTagStore()
  private pinnedTagStore: PinnedTagStore = usePinnedTagStore()

  private get allTagFields () {
    return [...this.addedTagStore.addedTagFields, ...this.tagFields]
  }

  sortedTagFields = $derived.by(() => {
    if (this.sortBy !== 'relevance')
      return this.allTagFields

    // relevance order: Added > Pinned > Supported > Custom

    // store callbacks for determining if a tag field is relevant
    const areRelevant: ((tf: TagField) => boolean)[] = []

    // added tag?
    areRelevant.push((tf: TagField) => tf.status === TagStatus.ADDED)

    // pinned tag?
    areRelevant.push(...this.pinnedTagStore.pinnedTagNames.map(
      (pinnedTagName) => (tf: TagField) => matchesTagName(pinnedTagName, tf)
    ))

    // supported tag?
    areRelevant.push(...this.supportedTagNames.map(
      (supportedTagName) => (tf: TagField) => matchesTagName(supportedTagName, tf)
    ))

    return sortTagFieldsByRelevance(
      this.allTagFields, areRelevant, this.sortOrder
    )
  })

  constructor() {
    $effect(() => {
      invoke<string[]>('get_supported_tags').then((tags) => {
        this.supportedTagNames = tags
      })
    })
  }

  isTagSupported(tagName: string): boolean {
    return this.supportedTagNames.includes(tagName)
  }

  removeTag(tagField: TagField) {
    if (tagField.status === TagStatus.ADDED) {
      this.addedTagStore.removeAddedTag(tagField)
    } else {
      tagField.status = TagStatus.REMOVED
    }
  }

  readdTag(tagField: TagField) {
    if (tagField.status === TagStatus.REMOVED) {
      tagField.status = TagStatus.UNCHANGED
    }
  }

  setTags(tags: Map<string, string> | undefined) {
    this.addedTagStore.resetTags()
    if (!tags) {
      this.tagFields = []
      return
    }

    const tagFields = []

    for (const [tagName, value] of tags.entries()) {
      tagFields.push(new TagField(tagName, value))
    }
    this.tagFields = tagFields
  }

  async saveTags(song: Song | null) {
    if (!song) return

    this.isSaving = true
    this.saveMessage = ''

    try {
      const newTags = new SvelteMap<string, string>(
        this.sortedTagFields
          .filter((field) => field.status !== TagStatus.REMOVED)
          // filter blank tags. TODO: replace with validation
          .filter((field) => field.tagName.trim() !== '')
          .map((field) => [field.tagName, field.tagValue])
      )


      await invoke('write_tags', {
        path: song.path,
        tags: newTags
      })
      song.tags = newTags
      this.setTags(newTags)
      this.saveMessage = `${newTags.size} ✓ Tags saved successfully`
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

let tagEditorStore: TagEditorStore | undefined = undefined

export function useTagEditorStore() {
  if (tagEditorStore === undefined) {
    tagEditorStore = new TagEditorStore()
  }
  return tagEditorStore
}
