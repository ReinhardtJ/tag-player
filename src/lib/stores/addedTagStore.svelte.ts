import { without } from 'lodash'
import { TagField } from '$lib/stores/tagEditorStore.svelte.ts'

export class AddedTagStore {
  addedTagFields = $state<TagField[]>([])

  removeAddedTag(tagField: TagField) {
    this.addedTagFields = without(this.addedTagFields, tagField)
  }

  resetTags() {
    this.addedTagFields = []
  }

  addTag() {
    this.addedTagFields.push(new TagField('', ''))
  }
}


let addedTagStore: AddedTagStore | undefined = undefined

export function useAddedTagStore() {
  if (addedTagStore === undefined) {
    addedTagStore = new AddedTagStore()
  }
  return addedTagStore
}
