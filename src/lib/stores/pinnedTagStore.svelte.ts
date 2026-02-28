import { concat, without } from 'lodash'

export class PinnedTagStore {
  private _pinnedTagNames = $state([
    'TrackTitle',
    'TrackArtist',
    'AlbumTitle',
    'AlbumArtist',
    'RecordingDate',
    'Genre',
    'Mood'
  ])

  get pinnedTagNames() {
    return this._pinnedTagNames
  }

  isPinnedTag(tagName: string): boolean {
    return this._pinnedTagNames.some((tag) => tag.toLowerCase() === tagName.toLowerCase())
  }

  togglePin(tagName: string) {
    if (this.isPinnedTag(tagName)) {
      this._pinnedTagNames = without(this._pinnedTagNames, tagName)
    } else {
      this._pinnedTagNames = concat(this._pinnedTagNames, tagName)
    }
  }
}

let pinnedTagStore: PinnedTagStore | undefined = undefined

export function usePinnedTagStore() {
  if (pinnedTagStore === undefined) {
    pinnedTagStore = new PinnedTagStore()
  }
  return pinnedTagStore
}
