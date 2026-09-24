import { without } from 'lodash'
import { TagField, TagStatus } from '$lib/stores/tagEditorStore.svelte.ts'

export class AddedTagStore {
  addedTagFields = $state<TagField[]>([])

  removeAddedTag(tagField: TagField) {
    this.addedTagFields = without(this.addedTagFields, tagField)
  }

  resetTags() {
    this.addedTagFields = []
  }

  addTag(tagName = '', tagValue = '') {
    this.addTagField(new TagField(tagName, tagValue, TagStatus.ADDED))
  }

  addTagField(tagField: TagField) {
    this.addedTagFields.push(tagField)
  }
}

let addedTagStore: AddedTagStore | undefined = undefined

export function useAddedTagStore() {
  if (addedTagStore === undefined) {
    addedTagStore = new AddedTagStore()
  }
  return addedTagStore
}
