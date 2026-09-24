import { describe, it, expect } from 'vitest'
import { missingPinnedTagNames, TagField } from './tagEditorStore.svelte.ts'

describe('missingPinnedTagNames', () => {
  it('returns pinned tags that no field covers yet, in pinned order', () => {
    const tagFields = [new TagField('TrackTitle', 'A Song')]

    expect(missingPinnedTagNames(['TrackTitle', 'Genre', 'Mood'], tagFields)).toEqual([
      'Genre',
      'Mood'
    ])
  })

  it('treats existing tags case-insensitively', () => {
    const tagFields = [new TagField('genre', 'Rock')]

    expect(missingPinnedTagNames(['Genre'], tagFields)).toEqual([])
  })

  it('returns an empty list when every pinned tag already exists', () => {
    const tagFields = [new TagField('TrackTitle', 'A Song'), new TagField('Genre', 'Rock')]

    expect(missingPinnedTagNames(['TrackTitle', 'Genre'], tagFields)).toEqual([])
  })
})
