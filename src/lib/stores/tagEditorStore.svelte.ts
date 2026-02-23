import { invoke } from '@tauri-apps/api/core'
import { SvelteMap, SvelteSet } from 'svelte/reactivity'
import { type AddedTagStore, useAddedTagStore } from './addedTagStore.svelte.ts'
import { type PinnedTagStore, usePinnedTagStore } from './pinnedTagStore.svelte.ts'
import type { Song } from './playerTypes.ts'
import type { SortOrder } from '$lib/components/SortByToolbar.types.ts'
import { countBy, filter, find, findIndex, intersectionBy, keys, pickBy } from 'lodash'


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

  constructor(tagName: string, tagValue: string) {
    this.id = crypto.randomUUID()

    this.status = $state(TagStatus.UNCHANGED)
    this.tagName = $state(tagName)
    this.tagValue = $state(tagValue)

    this.originalName = tagName
    this.originalValue = tagValue
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

  sortedTagFields = $derived.by(() => {
    // pinned tags are always displayed first
    let isRelevantCallbacks = this.pinnedTagStore.pinnedTagNames.map(
      (pinnedTagName) => (tf: TagField) => matchesTagName(pinnedTagName, tf)
    )

    if (this.sortBy === 'relevance') {
      // we display supported tags before unsupported tags
      isRelevantCallbacks = [
        ...isRelevantCallbacks,
        ...this.supportedTagNames.map(
          (supportedTagName) => (tf: TagField) => matchesTagName(supportedTagName, tf)
        )
      ]

      return sortTagFieldsByRelevance(
        this.tagFields,
        isRelevantCallbacks,
        this.sortOrder,
      )
    }

    return sortTagFieldsByRelevance(
      this.tagFields,
      isRelevantCallbacks,
      this.sortOrder
    )
  })

  constructor() {
    $effect(() => {
      invoke<string[]>('get_supported_tags').then((tags) => {
        this.supportedTagNames = tags
      })
    })
  }

  renameTag(tagField: TagField, newName: string): string {
    if (newName === tagField.originalName) {
      tagField.status = TagStatus.UNCHANGED
      return ''
    }

    if (newName === tagField.tagName)
      return ''

    tagField.status = TagStatus.EDITED
    newName = newName.trim()



    const tagAlreadyExists = this.sortedTagFields.some(
      (f) => f.id !== tagField.id && f.tagName.toLowerCase() === newName.toLowerCase()
    )

    if (tagAlreadyExists)
      return 'Tag Name already Exists'


    tagField.tagName = newName
    if (!newName)
      return 'Please Enter a Name'
    return ''
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

  updateTagValue(tagField: TagField, newValue: string) {
    if (tagField.originalValue === newValue) {
      tagField.status = TagStatus.UNCHANGED
    } else {
      tagField.status = TagStatus.EDITED
    }
    tagField.tagValue = newValue
  }

  setTags(tags: Map<string, string> | undefined) {
    this.addedTagStore.resetTags()
    if (!tags) {
      this.tagFields = []
      return
    }

    if (this.supportedTagNames.length === 0)
      return

    const tagFields = []

    for (const [tagName, value] of tags.entries()) {
      tagFields.push(new TagField(tagName, value))
    }
    this.tagFields = tagFields
  }




  async applyTags(song: Song | null) {
    if (!song) return

    this.isSaving = true
    this.saveMessage = ''

    try {
      const newTags = new SvelteMap<string, string>(
        this.sortedTagFields
          .filter((field) => field.status === TagStatus.REMOVED)
          // filter blank tags. TODO: replace with validation
          .filter((field) => field.tagName.trim())
          .map((field) => [field.tagName, field.tagValue])
      )

      song.tags = newTags

      await invoke('write_tags', {
        path: song.path,
        tags: newTags
      })

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

let tagEditorStore: TagEditorStore | undefined = undefined

export function useTagEditorStore() {
  if (tagEditorStore === undefined) {
    tagEditorStore = new TagEditorStore()
  }
  return tagEditorStore
}
