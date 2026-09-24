import { describe, it, expect } from 'vitest'
import { SvelteMap } from 'svelte/reactivity'
import {
  applyTagsToTagFields,
  findTagFieldByName,
  TagField,
  TagStatus
} from './tagEditorStore.svelte.ts'

describe('findTagFieldByName', () => {
  it('matches tag names case-insensitively', () => {
    const tagField = new TagField('AlbumTitle', 'Nevermind')
    expect(findTagFieldByName([tagField], 'albumtitle')).toBe(tagField)
  })

  it('returns undefined when there is no matching field', () => {
    const tagField = new TagField('AlbumTitle', 'Nevermind')
    expect(findTagFieldByName([tagField], 'TrackTitle')).toBeUndefined()
  })
})

describe('applyTagsToTagFields', () => {
  it('updates an existing tag in place and marks it edited', () => {
    const existing = new TagField('TrackTitle', 'Old title')

    const added = applyTagsToTagFields(
      new SvelteMap([['TrackTitle', 'Smells Like Teen Spirit']]),
      [existing]
    )

    expect(added).toHaveLength(0)
    expect(existing.tagValue).toBe('Smells Like Teen Spirit')
    expect(existing.status).toBe(TagStatus.EDITED)
  })

  it('returns unknown tags as new added fields', () => {
    const added = applyTagsToTagFields(new SvelteMap([['Genre', 'Grunge']]), [])

    expect(added).toHaveLength(1)
    expect(added[0].tagName).toBe('Genre')
    expect(added[0].tagValue).toBe('Grunge')
    expect(added[0].status).toBe(TagStatus.ADDED)
  })

  it('revives a removed tag when a new value is applied', () => {
    const removed = new TagField('Genre', 'Rock')
    removed.status = TagStatus.REMOVED

    applyTagsToTagFields(new SvelteMap([['Genre', 'Grunge']]), [removed])

    expect(removed.tagValue).toBe('Grunge')
    expect(removed.status).toBe(TagStatus.EDITED)
  })

  it('keeps a matching field unchanged when the value is identical', () => {
    const existing = new TagField('AlbumTitle', 'Nevermind')

    applyTagsToTagFields(new SvelteMap([['AlbumTitle', 'Nevermind']]), [existing])

    expect(existing.status).toBe(TagStatus.UNCHANGED)
  })
})
