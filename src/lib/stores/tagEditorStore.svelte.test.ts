import { describe, it, expect } from 'vitest'
import { matchesTagName, sortTagFieldsByRelevance, TagField } from './tagEditorStore.svelte.js'

function createTagField(tagName: string): TagField {
  return new TagField(tagName, `some value for tag ${tagName}`)
}

function buildRelevanceCallbackFromTagNames(tagNames: string[]){
  return tagNames.map(tagName => (tf: TagField) => matchesTagName(tagName, tf))
}

describe('sortTagFieldsByRelevance', () => {
  it('descending: sorts tag fields by relevance groups', () => {
    const fields: TagField[] = [
      createTagField('Prio2'),
      createTagField('OtherTag1'),
      createTagField('Prio1'),
      createTagField('Prio3'),
      createTagField('OtherTag2'),
      createTagField('Prio4')
    ]

    const result = sortTagFieldsByRelevance(
      fields,
      buildRelevanceCallbackFromTagNames(['Prio1', 'Prio2', 'Prio3', 'Prio4']),
      'desc'
    )

    expect(result.map((f) => f.tagName).slice(0, 4)).toEqual(['Prio1', 'Prio2', 'Prio3', 'Prio4'])
  })
  
  it('ascending: sorts tag fields by relevance groups', () => {
    const fields: TagField[] = [
      createTagField('Prio2'),
      createTagField('OtherTag1'),
      createTagField('Prio1'),
      createTagField('Prio3'),
      createTagField('OtherTag2'),
      createTagField('Prio4')
    ]

    const result = sortTagFieldsByRelevance(
      fields,
      buildRelevanceCallbackFromTagNames(['Prio1', 'Prio2', 'Prio3', 'Prio4']),
      'asc'
    )


    expect(result.map((f) => f.tagName).slice(2)).toEqual(['Prio4', 'Prio3', 'Prio2', 'Prio1'])
  })

  it('does not duplicate fields when tag name appears in multiple relevance groups', () => {
    const fields: TagField[] = [createTagField('DuplicateTag')]

    const result = sortTagFieldsByRelevance(
      fields,
      buildRelevanceCallbackFromTagNames(['DuplicateTag', 'DuplicateTag']),
      'desc'
    )

    expect(result).toHaveLength(1)
    expect(result[0].tagName).toBe('DuplicateTag')
  })
})
