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

export function findTagFieldByName(tagFields: TagField[], tagName: string) {
  return find(tagFields, (tagField) => matchesTagName(tagName, tagField))
}

// Merges external tags (e.g. from MusicBrainz) into the given fields without
// saving, mimicking tags the user would have typed in by hand. Matching fields
// are updated in place; unknown tags are returned as new ADDED fields.
export function applyTagsToTagFields(tags: Map<string, string>, tagFields: TagField[]): TagField[] {
  const addedTagFields: TagField[] = []

  for (const [tagName, tagValue] of tags.entries()) {
    const existingTagField = findTagFieldByName(tagFields, tagName)
    if (existingTagField) {
      existingTagField.updateValue(tagValue)
    } else {
      addedTagFields.push(new TagField(tagName, tagValue, TagStatus.ADDED))
    }
  }

  return addedTagFields
}

// Pinned tag names that the given fields don't have yet, in pinned order.
export function missingPinnedTagNames(
  pinnedTagNames: string[],
  tagFields: TagField[]
): string[] {
  return pinnedTagNames.filter(
    (pinnedTagName) => !findTagFieldByName(tagFields, pinnedTagName)
  )
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

  pendingCoverPath = $state<string | null>(null)
  isCoverRemoved = $state(false)

  private addedTagStore: AddedTagStore = useAddedTagStore()
  private pinnedTagStore: PinnedTagStore = usePinnedTagStore()

  private get allTagFields() {
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

  // Merges external tags (e.g. from MusicBrainz) into the editor without saving,
  // mimicking tags the user would have typed in by hand.
  applyTags(tags: Map<string, string>) {
    const addedTagFields = applyTagsToTagFields(tags, this.allTagFields)
    for (const addedTagField of addedTagFields) {
      this.addedTagStore.addTagField(addedTagField)
    }
  }

  setTags(tags: Map<string, string> | undefined) {
    this.addedTagStore.resetTags()
    this.isCoverRemoved = false
    this.pendingCoverPath = null
    if (!tags) {
      this.tagFields = []
      return
    }

    const tagFields = []

    for (const [tagName, value] of tags.entries()) {
      tagFields.push(new TagField(tagName, value))
    }
    this.tagFields = tagFields

    this.addPinnedTagPlaceholders()
  }

  setPendingCover(path: string) {
    this.pendingCoverPath = path
    this.isCoverRemoved = false
  }

  removeCover() {
    this.pendingCoverPath = null
    this.isCoverRemoved = true
  }

  restoreCover() {
    this.isCoverRemoved = false
  }


  // Pre-adds pinned tags the song doesn't have yet, so they show up as empty
  // added fields to focus on while tagging.
  private addPinnedTagPlaceholders() {
    const missingTagNames = missingPinnedTagNames(
      this.pinnedTagStore.pinnedTagNames,
      this.tagFields
    )
    for (const tagName of missingTagNames) {
      this.addedTagStore.addTag(tagName)
    }
  }

  async saveTags(song: Song | null) {
    if (!song) 
      return

    this.isSaving = true
    this.saveMessage = ''

    try {
      const newTags = new SvelteMap<string, string>(
        this.sortedTagFields
          .filter((field) => field.status !== TagStatus.REMOVED)
          // filter blank tags (e.g. pinned tags the user hasn't filled in yet)
          // TODO: replace with validation
          .filter((field) => field.tagName.trim() !== '' && field.tagValue.trim() !== '')
          .map((field) => [field.tagName, field.tagValue])
      )

      const coverChanged = this.pendingCoverPath !== null || this.isCoverRemoved

      await invoke('write_tags', {
        path: song.path,
        tags: newTags
      })

      if (coverChanged) {
        await invoke('write_cover', {
          path: song.path,
          coverPath: this.pendingCoverPath
        })
      }

      song.tags = newTags
      if (coverChanged) {
        song.cover_base64 = await invoke<string | null>('read_cover_data_url', {
          path: song.path
        })
      }
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

